use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router, Server,
};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use dotenvy::dotenv;
use models::{Password, User};
use schema::{passwords::dsl::passwords, users::dsl::users};
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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

async fn insert_user(State(pool): State<ConnectionPool>, Json(user): Json<User>) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match insert_into(users).values(user).execute(connection) {
            Ok(res) => {
                println!("{res}");
                StatusCode::CREATED
            }
            Err(res) => {
                println!("{res}");
                StatusCode::BAD_REQUEST
            }
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn list_users(State(pool): State<ConnectionPool>) -> (StatusCode, Json<Option<Vec<User>>>) {
    if let Ok(connection) = &mut pool.get() {
        match users.select(User::as_select()).load(connection) {
            Ok(res) => (StatusCode::FOUND, Json(Some(res))),
            Err(_) => (StatusCode::NOT_FOUND, Json(None)),
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
    }
}

async fn create_password(
    State(pool): State<ConnectionPool>,
    Json(password): Json<Password>,
) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match insert_into(passwords).values(password).execute(connection) {
            Ok(res) => {
                println!("{res}");
                StatusCode::CREATED
            }
            Err(res) => {
                println!("{res}");
                StatusCode::BAD_REQUEST
            }
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn login(State(pool): State<ConnectionPool>, Json(user): Json<User>) -> StatusCode {
    use crate::schema::users::{email, password};

    if let Ok(connection) = &mut pool.get() {
        match users
            .filter(email.eq(user.email))
            .filter(password.eq(user.password))
            .select(email)
            .get_result::<String>(connection)
        {
            Ok(_) => StatusCode::FOUND,
            Err(_) => StatusCode::UNAUTHORIZED,
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[tokio::main]
async fn main() {
    let pool = connection_pool();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .route("/list_users", get(list_users))
        .route("/register", post(insert_user))
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
