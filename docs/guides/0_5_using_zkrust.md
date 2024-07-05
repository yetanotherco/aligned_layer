# Generating & Submiting proofs to Aligned using ZKRust

## Dependencies

To generate and submit proofs to Aligned using ZKRust, you need to have the following dependencies installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [ZKRust](https://github.com/lambdaclass/zkRust)

## Generate & Submit proofs

To generate and submit proofs to Aligned using ZKRust, you can follow the steps below:

1. Clone the ZKRust repository:

```bash
git clone https://github.com/lambdaclass/zkRust
cd zkRust
```

2. Generate a keystore:

You can use cast to create a local keystore.
If you already have one you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <path_to_keystore.json>
```

Make sure to send at least 0.1 ETH to the address in the keystore.
You can get holesky eth from the [faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)

2. Generate and submit the proof:

```bash
cargo run --release -- prove-sp1 \
    --submit-to-aligned-with-keystore <path_to_keystore> \
    examples/fibonacci_no_std .
```

This command will generate a proof for the Fibonacci example circuit and submit it to Aligned using the keystore provided.
