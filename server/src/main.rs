use actions::{create_password, login, register};
use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::post,
    Router, Server,
};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod actions;
mod error;
mod models;
mod schema;
mod utils;

fn connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Connection pool could not be built.")
}

#[tokio::main]
async fn main() {
    let pool = connection_pool();

    let cors = CorsLayer::new()
        .allow_headers([CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/create_password", post(create_password))
        .with_state(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
