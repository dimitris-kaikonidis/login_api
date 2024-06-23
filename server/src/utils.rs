use super::error::ActionError;
use argon2::{password_hash::Salt, Argon2, PasswordHasher};
use num_bigint::BigUint;
use srp::groups::G_4096;

pub fn compute_server_value_b(
    salt: &str,
    v: &str,
    private_b: &BigUint,
) -> Result<String, ActionError> {
    let g = &G_4096.g;
    let n = &G_4096.n;

    let argon2 = Argon2::default();
    let k = argon2.hash_password(&(g + n).to_bytes_be(), Salt::from_b64(salt)?)?;
    let k = match k.hash {
        Some(hash) => hash,
        None => return Err(ActionError::InternalServerError),
    };
    let k = k.as_bytes();
    let k = BigUint::from_bytes_be(&k);

    let v = BigUint::from_bytes_be(v.as_bytes());

    let public_b = k * v * g.modpow(&private_b, n);

    Ok(public_b.to_string())
}
