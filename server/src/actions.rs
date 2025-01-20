use crate::{
    models::User,
    psw::{
        password_manager_server::PasswordManager,
        {
            login_request, login_response, LoginRequest, LoginResponse, LoginResponsePartOne,
            LoginResponsePartTwo, RegisterRequest, RegisterResponse,
        },
    },
    schema::users::{
        email as email_column, salt as salt_column, table as users_table,
        verifier as verifier_column,
    },
};
use blake2::Blake2b512;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use rand::RngCore;
use srp::{groups::G_4096, server::SrpServer};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

type LoginResult<T> = Result<Response<T>, Status>;
type LoginResponseStream = Pin<Box<dyn Stream<Item = Result<LoginResponse, Status>> + Send>>;

#[derive(Debug)]
pub struct PasswordManagerService {
    pub pool: Arc<Mutex<Pool<ConnectionManager<PgConnection>>>>,
}

#[tonic::async_trait]
impl PasswordManager for PasswordManagerService {
    type LoginStream = LoginResponseStream;

    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let connection = &mut self
            .pool
            .lock()
            .unwrap()
            .get()
            .map_err(|e| Status::internal(format!("Failed to connect to database: {}", e)))?;
        let stream = req.into_inner();

        match insert_into(users_table)
            .values(User {
                email: stream.email,
                verifier: stream.verifier,
                salt: stream.salt,
            })
            .execute(connection)
        {
            Ok(_) => Ok(Response::new(RegisterResponse { status_code: 201 })),
            Err(_) => Err(Status::new(tonic::Code::InvalidArgument, "Bad Request.")),
        }
    }

    async fn login(&self, req: Request<Streaming<LoginRequest>>) -> LoginResult<Self::LoginStream> {
        let mut connection = Arc::clone(&self.pool)
            .lock()
            .expect("Mutex error occured.")
            .get()
            .map_err(|e| Status::internal(format!("Failed to connect to database: {}", e)))?;

        let (tx, rx) = mpsc::channel(128);

        let mut private_b = [0u8; 32];
        let mut rng = rand::rngs::OsRng;
        rng.fill_bytes(&mut private_b);
        let srp_server = SrpServer::<Blake2b512>::new(&G_4096);
        let mut v: Vec<u8> = Vec::new();

        tokio::spawn(async move {
            let mut stream: Streaming<LoginRequest> = req.into_inner();

            while let Some(incoming_stream) = stream.next().await {
                match incoming_stream {
                    Ok(stream) => {
                        match stream.request {
                            Some(login_request::Request::LoginRequestPartOne(payload)) => {
                                let db_result = users_table
                                    .filter(email_column.eq(payload.email))
                                    .select((salt_column, verifier_column))
                                    .get_result::<(Vec<u8>, Vec<u8>)>(&mut connection)
                                    .expect("User not found");
                                v = db_result.1;

                                println!("{:?}", v);

                                tx.send(Ok(LoginResponse {
                                    response: Some(login_response::Response::LoginResponsePartOne(
                                        LoginResponsePartOne {
                                            status_code: 200,
                                            public_b: srp_server
                                                .compute_public_ephemeral(&private_b, &v),
                                            salt: db_result.0,
                                        },
                                    )),
                                }))
                                .await
                                .unwrap();
                            }
                            Some(login_request::Request::LoginRequestPartTwo(payload)) => {
                                let verifier = srp_server
                                    .process_reply(&private_b, &v, &payload.public_a)
                                    .expect("Failed to process reply");

                                println!("{:?} {:?}", v, payload.public_a);

                                verifier
                                    .verify_client(&payload.client_proof)
                                    .expect("Failed to verify client");

                                tx.send(Ok(LoginResponse {
                                    response: Some(login_response::Response::LoginResponsePartTwo(
                                        LoginResponsePartTwo {
                                            status_code: 200,
                                            server_proof: verifier.proof().to_vec(),
                                        },
                                    )),
                                }))
                                .await
                                .unwrap();
                            }
                            None => todo!(),
                        };
                    }
                    Err(error) => println!("error"),
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

// pub async fn create_password(
//     State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//     Json(password): Json<Password>,
// ) -> Result<StatusCode, ActionError> {
//     let connection = &mut pool.get()?;
//
//     match insert_into(passwords_table)
//         .values(password)
//         .execute(connection)
//     {
//         Ok(_) => Ok(StatusCode::CREATED),
//         Err(_) => Err(ActionError::BadRequest),
//     }
// }
