# RISC Zero Benchmarking

## Execution Cycles

keccak256 - 22796

sha256 - 10207

poseidon - 1199735

mimc - 9596643

ecdsa sign - 13224530

ecdsa verify - 18050797

eddsa sign - 1786282

eddsa verify - 2617506

fibonacci - 2396

## Precompile Execution Cycles

keccak256 - 22796

sha256 - 6463

ecdsa sign - 12882829

ecdsa verify - 17523275

### Run

`RISC0_DEV_MODE=0 RUST_LOG=info cargo run --release`
