#![no_main]

use hashing::poseidon;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let hash = poseidon(&message);

    env::commit(&hash);
}
