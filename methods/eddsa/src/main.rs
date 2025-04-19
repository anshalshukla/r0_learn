#![no_main]

use hashing::sign_message_ed25519;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let signing_key: ed25519_dalek::SigningKey = env::read();

    // Sign the message
    let signature = sign_message_ed25519(&signing_key, &message);

    env::commit(&signature);
}
