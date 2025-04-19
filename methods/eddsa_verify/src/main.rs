#![no_main]

use ed25519_dalek::VerifyingKey;
use hashing::verify_signature_ed25519;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let signature: ed25519_dalek::Signature = env::read();

    let verifying_key: VerifyingKey = env::read();

    // Verify the signature
    verify_signature_ed25519(&verifying_key, &message, &signature);

    env::commit(&signature);
}
