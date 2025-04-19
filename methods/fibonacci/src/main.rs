#![no_main]

use core::num::{NonZeroU128, NonZeroU16};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);
fn main() {
    let input = env::read();

    let result: u128 = match fibonacci(input) {
        Some((_, num)) => num.get(),
        None => 0,
    };

    env::commit(&result);
}

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
fn fibonacci(n: NonZeroU16) -> Option<(u128, NonZeroU128)> {
    const MAX_SUPPORTED: NonZeroU16 = match NonZeroU16::new(186) {
        Some(n) => n,
        None => panic!("max supported must be positive"),
    };

    const FIB_0: u128 = 0;

    const FIB_1: NonZeroU128 = match NonZeroU128::new(1) {
        Some(n) => n,
        None => panic!("first fibonacci number must be positive"),
    };

    n.le(&MAX_SUPPORTED).then(|| {
        let mut a = FIB_0;
        let mut b = FIB_1;

        (1..n.get()).for_each(|_| {
            let c = b.saturating_add(a);
            a = b.get();
            b = c;
        });

        (a, b)
    })
}
