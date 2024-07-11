use blake2::Blake2b512;
use rand::RngCore;
use srp::{
    client::{SrpClient, SrpClientVerifier},
    groups::G_4096,
};
use std::sync::{Arc, Mutex};

struct Client<'a> {
    pub srp_client: Arc<Mutex<SrpClient<'a, Blake2b512>>>,
    pub private_a: Arc<Mutex<Vec<u8>>>,
    pub verifier: Arc<Mutex<Option<SrpClientVerifier<Blake2b512>>>>,
}

impl<'a> Client<'a> {
    fn new() -> Client<'a> {
        Self {
            srp_client: Arc::new(Mutex::new(SrpClient::<'a, Blake2b512>::new(&G_4096))),
            private_a: Arc::new(Mutex::new(Vec::new())),
            verifier: Arc::new(Mutex::new(None)),
        }
    }
}

#[tauri::command]
fn create_salt_and_verifier(username: &str, password: &str) -> (Vec<u8>, Vec<u8>) {
    let mut salt = [0u8; 32];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut salt);
    let srp_client = SrpClient::<Blake2b512>::new(&G_4096);

    let v = srp_client.compute_verifier(username.as_bytes(), password.as_bytes(), &salt);

    (salt.to_vec(), v)
}

#[tauri::command]
fn public_a(state: tauri::State<Client>) -> Vec<u8> {
    let mut private_a = [0u8; 32];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut private_a);
    let public_a = state
        .srp_client
        .lock()
        .unwrap()
        .compute_public_ephemeral(&private_a);

    *state.private_a.lock().unwrap() = private_a.to_vec();

    public_a
}

#[tauri::command]
fn compute_verifier(
    username: &str,
    password: &str,
    salt: Vec<u8>,
    public_b: Vec<u8>,
    state: tauri::State<Client>,
) -> Vec<u8> {
    let verifier = state
        .srp_client
        .lock()
        .unwrap()
        .process_reply(
            &state.private_a.lock().unwrap(),
            username.as_bytes(),
            password.as_bytes(),
            &salt,
            &public_b,
        )
        .unwrap();

    let proof = verifier.proof().to_vec();

    *state.verifier.lock().unwrap() = Some(verifier);

    proof
}

#[tauri::command]
fn verify_server_proof(server_proof: Vec<u8>, state: tauri::State<Client>) -> Result<(), String> {
    let verifier = state.verifier.lock().unwrap();

    match verifier.as_ref() {
        Some(verifier) => {
            if verifier.verify_server(&server_proof).is_ok() {
                return Ok(());
            }

            return Err("Server proof verification failed".to_string());
        }
        None => Err("No verifier found".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Client::new())
        .invoke_handler(tauri::generate_handler![
            create_salt_and_verifier,
            public_a,
            compute_verifier,
            verify_server_proof
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
