#![no_main]

use hashing::mimc;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let message: Vec<u8> = env::read();

    let hash = mimc(&message);

    env::commit(&hash);
}
