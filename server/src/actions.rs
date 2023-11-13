use crate::{
    models::{Password, User},
    schema::{
        passwords::table as passwords_table,
        users::table as users_table,
        users::{email as email_column, password as password_column},
    },
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{extract::State, http::StatusCode, Json};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use rand::rngs::OsRng;

fn hash_password(password: &str) -> Option<String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => Some(hash.to_string()),
        Err(_) => None,
    }
}

fn hash_user_password(user: &mut User) -> Result<(), ()> {
    if let Some(password) = hash_password(&user.password) {
        user.password = password;
        return Ok(());
    }

    Err(())
}

pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(mut user): Json<User>,
) -> StatusCode {
    if let (Ok(connection), Ok(())) = (&mut pool.get(), hash_user_password(&mut user)) {
        return match insert_into(users_table).values(user).execute(connection) {
            Ok(_) => StatusCode::CREATED,
            Err(_) => StatusCode::BAD_REQUEST,
        };
    }

    StatusCode::INTERNAL_SERVER_ERROR
}

pub async fn login(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<User>,
) -> StatusCode {
    if let Ok(connection) = &mut pool.get() {
        return match users_table
            .filter(email_column.eq(user.email))
            .select(password_column)
            .get_result::<String>(connection)
        {
            Ok(password) => {
                let parsed_hash = PasswordHash::new(&password).unwrap();
                if Argon2::default()
                    .verify_password(user.password.as_bytes(), &parsed_hash)
                    .is_err()
                {
                    return StatusCode::UNAUTHORIZED;
                }
                StatusCode::FOUND
            }
            Err(_) => StatusCode::NOT_FOUND,
        };
    }

    StatusCode::INTERNAL_SERVER_ERROR
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
