use crate::psw::{
    login_request, login_response, password_manager_client::PasswordManagerClient, LoginRequest,
    LoginRequestPartOne, LoginRequestPartTwo, RegisterRequest,
};
use blake2::Blake2b512;
use rand::RngCore;
use rpassword::read_password;
use serde_email::Email;
use srp::{
    client::{SrpClient, SrpClientVerifier},
    groups::G_4096,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Channel, Request};

fn get_credentials() -> (String, String) {
    let mut email = String::new();
    println!("Enter email: ");
    std::io::stdin()
        .read_line(&mut email)
        .expect("Failed to read email");
    let email = Email::from_str(email.trim()).expect("Enter a valid email address");

    println!("Enter password: ");
    let password = read_password().expect("Failed to read password");

    return (email.to_string(), password);
}

pub async fn register(client: &mut PasswordManagerClient<Channel>) {
    let (email, password) = get_credentials();

    let mut salt = [0u8; 32];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut salt);

    let srp_client = SrpClient::<Blake2b512>::new(&G_4096);
    let v = srp_client.compute_verifier(email.as_bytes(), password.as_bytes(), &salt);

    client
        .register(Request::new(RegisterRequest {
            email: email.to_string(),
            verifier: v,
            salt: salt.to_vec(),
        }))
        .await
        .expect("Something went wrong. Registration failed.");

    println!("Registration complete.")
}

pub async fn login(client: &mut PasswordManagerClient<Channel>) {
    let (email, password) = get_credentials();

    let mut private_a = [0u8; 32];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut private_a);

    let srp_client = SrpClient::<Blake2b512>::new(&G_4096);
    let mut verifier: Option<SrpClientVerifier<Blake2b512>> = None;

    let (tx, rx) = tokio::sync::mpsc::channel::<LoginRequest>(128);

    let stream = ReceiverStream::new(rx);
    let request = Request::new(stream);
    let mut response = client
        .login(request)
        .await
        .expect("Failed to create response stream.")
        .into_inner();

    tx.send(LoginRequest {
        request: Some(login_request::Request::LoginRequestPartOne(
            LoginRequestPartOne {
                email: email.clone(),
            },
        )),
    })
    .await
    .expect("Failed to send first login request.");

    if let Some(login_response::Response::LoginResponsePartOne(payload)) = response
        .message()
        .await
        .expect("Server failed to respond. (Part one)")
        .expect("Empty server response. (Part one)")
        .response
    {
        verifier = Some(
            srp_client
                .process_reply(
                    &private_a,
                    email.as_bytes(),
                    password.as_bytes(),
                    &payload.salt,
                    &payload.public_b,
                )
                .expect("Failed to create client verifier."),
        );

        tx.send(LoginRequest {
            request: Some(login_request::Request::LoginRequestPartTwo(
                LoginRequestPartTwo {
                    public_a: srp_client.compute_public_ephemeral(&private_a),
                    client_proof: verifier
                        .as_mut()
                        .expect("Uninitialized verifier.")
                        .proof()
                        .to_vec(),
                },
            )),
        })
        .await
        .expect("Failed to send second login request.");
    }

    if let Some(login_response::Response::LoginResponsePartTwo(payload)) = response
        .message()
        .await
        .expect("Server failed to respond. (Part two)")
        .expect("Empty server response. (Part two)")
        .response
    {
        match verifier
            .expect("Uninitialized verifier.")
            .verify_server(&payload.server_proof)
        {
            Ok(_) => println!("Login success."),
            Err(_) => println!("Login denied."),
        }
    }
}
