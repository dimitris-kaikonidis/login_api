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
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::Request;

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

    // Part One
    let stream = tokio_stream::once(LoginRequest {
        request: Some(login_request::Request::LoginRequestPartOne(
            LoginRequestPartOne {
                email: email.clone(),
            },
        )),
    });

    let mut response = client.login(stream).await.unwrap().into_inner();
    let login_response_part_one = response.next().await.unwrap().unwrap().response;

    if let Some(login_response::Response::LoginResponsePartOne(payload)) = login_response_part_one {
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
    }

    // Part Two
    let stream = tokio_stream::once(LoginRequest {
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
    });

    let mut response = client.login(stream).await.unwrap().into_inner();
    let login_response_part_two = response.next().await.unwrap().unwrap().response;

    if let Some(login_response::Response::LoginResponsePartTwo(payload)) = login_response_part_two {
        match verifier
            .expect("Uninitialized verifier.")
            .verify_server(&payload.server_proof)
        {
            Ok(_) => println!("Login success."),
            Err(_) => println!("Login denied."),
        }
    }
}
