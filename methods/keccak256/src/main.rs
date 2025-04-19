#![no_main]

use hashing::keccak256;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let hash = keccak256(&message);

    env::commit(&hash);
}
