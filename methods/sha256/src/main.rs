#![no_main]

use hashing::sha256;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let hash = sha256(&message);

    env::commit(&hash);
}
