use crate::{
    models::{Password, User},
    schema::{
        passwords::table as passwords_table,
        users::table as users_table,
        users::{email as email_column, password as password_column},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

pub async fn insert_user(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<User>,
) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match insert_into(users_table).values(user).execute(connection) {
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

pub async fn create_password(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(password): Json<Password>,
) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match insert_into(passwords_table)
            .values(password)
            .execute(connection)
        {
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

pub async fn authenticate_user(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<User>,
) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        match users_table
            .filter(email_column.eq(user.email))
            .filter(password_column.eq(user.password))
            .select(email_column)
            .get_result::<String>(connection)
        {
            Ok(_) => StatusCode::FOUND,
            Err(_) => StatusCode::UNAUTHORIZED,
        }
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
