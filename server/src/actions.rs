use crate::{models::User, schema::users::table as users_table};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use blake2::Blake2b512;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use futures::StreamExt;
use psw::password_manager_server::{PasswordManager, PasswordManagerServer};
use psw::{RegisterRequest, RegisterResponse};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use srp::{groups::G_4096, server::SrpServer};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct PasswordManagerService {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

#[tonic::async_trait]
impl PasswordManager for PasswordManagerService {
    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let connection = &mut self
            .pool
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
}

// pub async fn login_connection(
//     State(pool): State<Pool<ConnectionManager<PgConnection>>>,
//     ws: WebSocketUpgrade,
// ) -> impl IntoResponse {
//     ws.on_failed_upgrade(|_| println!("Failed to upgrade WebSocket connection"))
//         .on_upgrade(move |socket| login_auth(socket, pool))
// }

#[derive(Deserialize, Debug)]
struct UserID {
    email: String,
}

#[derive(Serialize, Debug)]
struct FirstStepServer<'a> {
    #[serde(rename = "type")]
    __type: String,
    #[serde(with = "serde_bytes")]
    salt: &'a [u8],
    #[serde(with = "serde_bytes")]
    public_b: &'a [u8],
}

#[derive(Serialize, Debug)]
struct SecondStepServer<'a> {
    #[serde(rename = "type")]
    __type: String,
    #[serde(with = "serde_bytes")]
    pub server_proof: &'a [u8],
}

// pub async fn login_auth(mut ws: WebSocket, pool: Pool<ConnectionManager<PgConnection>>) {
//     let user_id = ws
//         .next()
//         .await
//         .expect("WS: user_id is None")
//         .expect("WS: user_id results to Err");
//     let user_id = user_id
//         .to_text()
//         .expect("Failed to convert user id to text");
//     let user_id = serde_json::from_str::<UserID>(user_id).expect("Invalid JSON (UserEmail)");
//
//     let connection = &mut pool.get().expect("Failed to connect to database");
//
//     let (salt, v) = users_table
//         .filter(email_column.eq(&user_id.email))
//         .select((salt_column, verifier_column))
//         .get_result::<(Vec<u8>, Vec<u8>)>(connection)
//         .expect("User not found");
//
//     let mut private_b = [0u8; 32];
//     let mut rng = rand::rngs::OsRng;
//     rng.fill_bytes(&mut private_b);
//     let srp_server = SrpServer::<Blake2b512>::new(&G_4096);
//     let public_b = srp_server.compute_public_ephemeral(&private_b, &v);
//
//     let first_step_server = FirstStepServer {
//         __type: "first_step_server".to_string(),
//         salt: &salt,
//         public_b: &public_b,
//     };
//     let first_step_server =
//         serde_json::to_string(&first_step_server).expect("Failed to serialize first step server");
//
//     ws.send(Message::Text(first_step_server))
//         .await
//         .expect("Failed to send first step server to client");
//
//     let public_a = ws
//         .next()
//         .await
//         .expect("WS: public_a is None")
//         .expect("WS: public_a results to Err");
//     let public_a = serde_json::from_slice::<Vec<u8>>(&public_a.into_data())
//         .expect("Failed to deserialize public_a");
//
//     let verifier = srp_server
//         .process_reply(&private_b, &v, &public_a)
//         .expect("Failed to process reply");
//
//     let second_step_client = ws
//         .next()
//         .await
//         .expect("WS: second_step_client is None")
//         .expect("WS: second_step_client results to Err");
//
//     let client_proof: Vec<u8> = serde_json::from_slice(&second_step_client.into_data())
//         .expect("Failed to deserialize client proof");
//
//     verifier
//         .verify_client(&client_proof)
//         .expect("Failed to verify client");
//
//     let second_step_server = SecondStepServer {
//         __type: "second_step_server".to_string(),
//         server_proof: verifier.proof(),
//     };
//     let second_step_server =
//         serde_json::to_string(&second_step_server).expect("Failed to serialize second step server");
//
//     ws.send(Message::Text(second_step_server))
//         .await
//         .expect("Failed to send second step server to client");
//
//     ws.close().await.expect("Failed to close connection");
// }

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
