use axum::{extract::State, http::StatusCode, routing::get, Json, Router, Server};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use dotenvy::dotenv;
use models::User;
use schema::users::dsl::users;
use std::{env, net::SocketAddr};

mod models;
mod schema;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

fn connection_pool() -> ConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Connection pool could not be built.")
}

fn insert_user(user: &User, State(pool): State<ConnectionPool>) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match insert_into(users).values(user).execute(connection) {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::BAD_REQUEST,
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn list_users(State(pool): State<ConnectionPool>) -> (StatusCode, Json<Vec<User>>) {
    if let Ok(connection) = &mut pool.get() {
        match users.select(User::as_select()).load(connection) {
            Ok(res) => (StatusCode::FOUND, Json(res)),
            Err(_) => (StatusCode::NOT_FOUND, Json(Vec::new())),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
    }
}

#[tokio::main]
async fn main() {
    let pool = connection_pool();

    insert_user(
        &User {
            first_name: Some("FIRST".into()),
            last_name: Some("LAST".into()),
            display_name: Some("DISPLAYNAME".into()),
            email: "EMAIL".into(),
            password: "password".into(),
        },
        State(pool.clone()),
    );

    let app = Router::new()
        .route("/list_users", get(list_users))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
