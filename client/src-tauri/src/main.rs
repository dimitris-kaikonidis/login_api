// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use num_bigint::BigUint;
use rand::rngs::OsRng;
use srp::groups::G_4096;

fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn hash_password<'a>(
    password: &'a str,
    salt: &'a SaltString,
) -> Result<argon2::PasswordHash<'a>, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(), salt)
}

#[tauri::command]
fn handle_submit(username: &str, password: &str) -> String {
    let salt = generate_salt();
    let g = &G_4096.g;
    let n = &G_4096.n;
    let password_hash = hash_password(password, &salt)
        .unwrap()
        .hash
        .unwrap()
        .as_bytes()
        .to_owned();

    let password_hash_big_uint = BigUint::from_bytes_be(&password_hash);
    let v = g.modpow(&password_hash_big_uint, n);

    todo!()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_submit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
