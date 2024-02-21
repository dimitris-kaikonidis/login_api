use crate::{
    error::ActionError,
    models::{Password, User},
    schema::{
        passwords::table as passwords_table,
        users::table as users_table,
        users::{email as email_column, verifier as verifier_column},
    },
    utils::{verify_user_password, AuthBody},
};
use axum::{extract::State, http::StatusCode, Json};
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use srp::groups::G_4096;

pub async fn register(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(mut user): Json<User>,
) -> Result<(StatusCode, Json<AuthBody>), ActionError> {
    let connection = &mut pool.get()?;

    let grp_4096 = G_4096.n;

    println!("{user:?}");

    // hash_user_password(&mut user)?;
    //
    // match insert_into(users_table).values(&user).execute(connection) {
    //     Ok(_) => Ok((StatusCode::CREATED, generate_token(user)?)),
    //     Err(_) => Err(ActionError::BadRequest),
    // }

    todo!()
}

pub async fn login(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(user): Json<User>,
) -> Result<(StatusCode, Json<AuthBody>), ActionError> {
    let connection = &mut pool.get()?;

    match users_table
        .filter(email_column.eq(&user.email))
        .select(verifier_column)
        .get_result::<String>(connection)
    {
        Ok(password) => Ok((StatusCode::OK, verify_user_password(password, user)?)),
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
