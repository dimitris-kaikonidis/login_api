use crate::{
    error::ActionError,
    models::{Password, User},
    schema::{
        passwords::table as passwords_table,
        users::table as users_table,
        users::{email as email_column, salt as salt_column, verifier as verifier_column},
    },
    utils::compute_server_value_b,
};
use argon2::{password_hash::Salt, Argon2, PasswordHasher};
use axum::{extract::State, http::StatusCode, Json};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use num_bigint::{BigUint, RandomBits};
use rand::Rng;
use serde::{Deserialize, Serialize};
use srp::groups::G_4096;

pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<User>,
) -> Result<StatusCode, ActionError> {
    let connection = &mut pool.get()?;

    match insert_into(users_table).values(&user).execute(connection) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(ActionError::BadRequest),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginData {
    email: String,
    public_value_a: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    public_b: String,
}

pub async fn login(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<LoginData>,
) -> Result<Json<LoginResponse>, ActionError> {
    let connection = &mut pool.get()?;

    match users_table
        .filter(email_column.eq(&user.email))
        .select((salt_column, verifier_column))
        .get_result::<(String, String)>(connection)
    {
        Ok((salt, verifier)) => {
            let mut rng = rand::thread_rng();
            let private_b: BigUint = rng.sample(RandomBits::new(256));
            let public_b = compute_server_value_b(&salt, &verifier, &private_b)?;
            let n = &G_4096.n;

            let mut concanted_ab = Vec::new();
            concanted_ab.extend_from_slice(user.public_value_a.as_bytes());
            concanted_ab.extend_from_slice(public_b.as_bytes());

            let argon2 = Argon2::default();

            let u = argon2.hash_password(&concanted_ab, Salt::from_b64(&salt)?)?;
            let u = match u.hash {
                Some(hash) => hash,
                None => return Err(ActionError::InternalServerError),
            };
            let u = u.as_bytes();
            let u = BigUint::from_bytes_be(&u);

            let public_a = BigUint::from_bytes_be(user.public_value_a.as_bytes());
            let verifier = BigUint::from_bytes_be(verifier.as_bytes());

            let premaster_secret = (public_a * verifier ^ &u) ^ private_b % n;

            println!("Premaster secret: {}", premaster_secret);

            Ok(Json(LoginResponse { public_b }))
        }
        Err(_) => Err(ActionError::NotFound),
    }
}

pub async fn create_password(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(password): Json<Password>,
) -> Result<StatusCode, ActionError> {
    let connection = &mut pool.get()?;

    match insert_into(passwords_table)
        .values(password)
        .execute(connection)
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(ActionError::BadRequest),
    }
}
