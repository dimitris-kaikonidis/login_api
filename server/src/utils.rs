use super::error::ActionError;
use crate::models::User;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use jwt_simple::prelude::{Claims, Duration, HS256Key, MACLike};
use serde::Serialize;

const N: u128 = 500;
const g: u8 = 2;

// pub fn hash_user_password(user: &mut User) -> Result<(), ActionError> {
//     let argon2 = Argon2::default();
//     let salt = SaltString::generate(&mut OsRng);
//     let password = argon2.hash_password(user.verifier.as_bytes(), &salt)?;
//     user.verifier = password.to_string();
//
//     println!("{password}");
//     println!("{}", password.to_string().parse::<u128>().unwrap());
//
//     // let verifier = g ^ password % N;
//
//     Ok(())
// }

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

pub fn verify_user_password(password: String, user: User) -> Result<Json<AuthBody>, ActionError> {
    let parsed_hash = PasswordHash::new(&password)?;
    Argon2::default().verify_password(user.verifier.as_bytes(), &parsed_hash)?;
    Ok(generate_token(user)?)
}

pub fn generate_token(user: User) -> Result<Json<AuthBody>, jwt_simple::Error> {
    let claims = Claims::with_custom_claims(user, Duration::from_mins(10));
    let key = HS256Key::generate();
    let token = key.authenticate(claims)?;

    Ok(Json(AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}
