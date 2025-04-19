#![no_main]

use hashing::sign_message_ecdsa;
use p256::ecdsa::SigningKey;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let mut signing_key_bytes = [0u8; 32];
    env::read_slice(&mut signing_key_bytes);
    let signing_key = SigningKey::from_slice(&signing_key_bytes).unwrap();

    // Sign the message
    let signature = sign_message_ecdsa(&signing_key, &message);

    env::commit(&signature);
}
