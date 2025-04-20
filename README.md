# RISC Zero Benchmarking

## Execution Cycles v2

keccak256 - 22796

sha256 - 10207

poseidon - 1199735

mimc - 9596643

ecdsa sign - 13224530

ecdsa verify - 18050797

eddsa sign - 1786282

eddsa verify - 2617506

fibonacci - 2396

## Precompile Execution Cycles v2

keccak256 - 22796

sha256 - 6463

ecdsa sign - 12882829

ecdsa verify - 17523275

## Execution Cycles v1

keccak256 - 30760

sha256 - 11551

poseidon - 1263749

mimc - 11232751

ecdsa sign - 13700410

ecdsa verify - 18648258

eddsa sign - 2019626

eddsa verify - 2878020

fibonacci - 2327

## Precompile Execution Cycles v1

keccak256 - 30760

sha256 - 5701

ecdsa sign - 13343906

ecdsa verify - 18117087

### Run

`RISC0_DEV_MODE=0 RUST_LOG=info cargo run --release`
