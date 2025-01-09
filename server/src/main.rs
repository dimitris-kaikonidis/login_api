use actions::PasswordManagerService;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use psw::password_manager_server::PasswordManagerServer;
use tonic::transport::Server;

mod actions;
mod error;
mod models;
mod schema;
mod psw {
    tonic::include_proto!("psw");
}

fn connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Connection pool could not be built.")
}

#[tokio::main]
async fn main() {
    let password_manager = PasswordManagerService {
        pool: connection_pool(),
    };
    let psw = PasswordManagerServer::new(password_manager);
    let addr = "0.0.0.0:3000".parse().unwrap();

    Server::builder()
        .add_service(psw)
        .serve(addr)
        .await
        .unwrap();
}
