#![no_main]

use hashing::verify_signature_ecdsa;
use p256::ecdsa::{SigningKey, VerifyingKey};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let signature: p256::ecdsa::Signature = env::read();

    let mut signing_key_bytes = [0u8; 32];
    env::read_slice(&mut signing_key_bytes);
    let signing_key = SigningKey::from_slice(&signing_key_bytes).unwrap();
    let verifying_key = VerifyingKey::from(signing_key);

    // Verify Signature
    verify_signature_ecdsa(&verifying_key, &message, &signature);

    env::commit(&signature);
}
