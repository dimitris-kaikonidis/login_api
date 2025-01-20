use actions::{login, register};
use clap::Command;
use psw::password_manager_client::PasswordManagerClient;

mod actions;
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
        .subcommands([
            Command::new("register").about("Register user."),
            Command::new("login").about("Login user."),
        ]);

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("register", _sub_matches)) => register(&mut client).await,
        Some(("login", _sub_matches)) => login(&mut client).await,
        _ => unreachable!(),
    }
}
