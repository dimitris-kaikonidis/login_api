use clap::{Command, Parser};
use psw::password_manager_client::PasswordManagerClient;
use psw::RegisterRequest;
use rpassword::read_password;
use serde_email::Email;
use srp::create_verifier_and_salt;
use tonic::Request;

#[derive(Parser, Debug)]
struct Args {}

mod actions;
mod srp;
mod psw {
    tonic::include_proto!("psw");
}

#[tokio::main]
async fn main() {
    let mut client = PasswordManagerClient::connect("http://localhost:3000")
        .await
        .unwrap();

    let cli = Command::new("psw")
        .about("Password Manager CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([Command::new("register").about("Register user.")]);

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("register", _sub_matches)) => {
            let mut email = String::new();
            println!("Enter email: ");
            std::io::stdin()
                .read_line(&mut email)
                .expect("Failed to read email");
            let email = Email::from_str(email.trim()).expect("Enter a valid email address");

            println!("Enter password: ");
            let password = read_password().expect("Failed to read password");

            let (salt, verifier) = create_verifier_and_salt(email.as_str(), &password);

            client
                .register(Request::new(RegisterRequest {
                    email: email.to_string(),
                    verifier,
                    salt,
                }))
                .await
                .unwrap();
        }
        _ => unreachable!(),
    }
}
