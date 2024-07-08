# Generating & submitting proofs to Aligned using zkRust

With zkRust, you can generate proofs of Rust code and then submit them to Aligned to be verified with only one command. You can choose between different Rust zkVMs for generating your proof. The supported provers at the moment are:
* [Risc0](https://github.com/risc0/risc0)
* [SP1](https://github.com/succinctlabs/sp1)

## Dependencies

To generate and submit proofs to Aligned using ZKRust, you need to have the following dependencies installed:
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation) (**Optional**, needed only to create a local keystore to sign Ethereum transactions if you didn't already have one).

## Generate & Submit proofs

To generate and submit proofs to Aligned testnet using zkRust, you can follow the steps below:

### 1. Clone the zkRust repository:

```bash
git clone https://github.com/lambdaclass/zkRust
cd zkRust
```

### 2. Generate a keystore:

You can use cast to create a local keystore.
If you already have one you can skip this step.

```bash
cast wallet new-mnemonic
```

Then you can import your created keystore using:

```bash
cast wallet import --interactive <path_to_keystore.json>
```

Make sure to send at least 0.1 Holesky ETH to the address in the keystore.
You can get Holesky ETH from the [faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)

### 3. Generate and submit the proof with zkRust:

The zkRust repo has some predefined examples that can be used to generate a proof. You can find them in `zkRust/examples`. For example, to generate a proof of a `fibonacci` program with Risc0 and submit it to aligned, run

```bash
cargo run --release -- prove-risc0 \
    --submit-to-aligned-with-keystore <path_to_keystore> \
    examples/fibonacci .
```

This command will generate a proof for the fibonacci example program and submit it to Aligned using the keystore provided for signing the transaction.

Take into consideration that the proof generation can take some time. Once the proof has been generated, a prompt will appear asking for the passphrase of your keystore and then send it to Aligned.

The same program can be proved using SP1 just changing the zkRust subcommand,

```bash
cargo run --release -- prove-sp1 \
    --submit-to-aligned-with-keystore <path_to_keystore> \
    examples/fibonacci .
```

## Caveats

For the moment, the Rust code that can be proven has some limitations:
* Programs with I/O are not supported
* Programs that rely on zkVMs precompiles may take too much time and computing resources.   
