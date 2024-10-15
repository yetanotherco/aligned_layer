# Generating Proofs For Aligned

## SP1

### Dependencies

This guide assumes that:

- sp1 prover installed (instructions [here](https://succinctlabs.github.io/sp1/getting-started/install.html))
- sp1 project to generate the proofs
  (instructions [here](https://succinctlabs.github.io/sp1/generating-proofs/setup.html))
- aligned installed (instructions [here](../1_introduction/1_try_aligned.md#quickstart))

### How to generate a proof

> Aligned only verifies SP1 in a compressed version.
> You can check you are using compressed by opening script/src/main.rs
> and check that the proof is generated with `client.prove_compressed` instead of `client.prove`.

First, open a terminal and navigate to the script folder in the sp1 project directory

Then, run the following command to generate a proof:

```bash
cargo run --release
```

### How to get the proof verified by Aligned

After generating the proof, you will have to find two different files:

- **proof file**: usually found under `script` directory, with the name `proof.json` or similar
- **elf file**: usually found under `program/elf/` directory

Then, you can send the proof to the Aligned network by running the following command

```bash
aligned submit \
--proving_system SP1 \
--proof <proof_path> \
--vm_program <vm_program_path> \
--batcher_url wss://batcher.alignedlayer.com \
--proof_generator_addr <proof_generator_addr> \
--rpc_url https://ethereum-holesky-rpc.publicnode.com 
```

Where `proof_path` is the path to the proof file, `vm_program_path` is the path to the ELF file. `proof_generator_addr` is an optional parameter that works as a helper for some applications where you can be frontrunned.

For more instructions on how to submit proofs, check the [Submitting proofs guide](../3_guides/0_submitting_proofs.md).

## Gnark

### Dependencies

This guide assumes that:

- Gnark library is installed. If not, install it using the following command inside your Go module:

 ```bash
 go get github.com/consensys/gnark@v0.10.0
 ```

- Gnark project to generate the proofs' instructions [here](https://docs.gnark.consensys.io/category/how-to)

### How to generate a proof

Open a terminal and navigate to the Gnark project directory. Then, run the following command to generate the proof:

 ```bash
 go run circuit.go
 ```

### How to get the proof verified by Aligned

After generating the proof, you will have to have three different files:

- proof file
- verification key file
- public input file

For a detailed guide on how to generate those files, refer to our [detailed guide](3.2_generate_gnark_proof.md).

Then, you can send the proof to the Aligned network by running the following command

```bash
aligned submit \
--proving_system GnarkPlonkBn254 \
--proof <proof_path> \
--public_input <public_input_path>
--vk <verification_key_path> \
--batcher_url wss://batcher.alignedlayer.com \
--proof_generator_addr <proof_generator_addr> \
--rpc_url https://ethereum-holesky-rpc.publicnode.com 
```

Where proof path is the path to the proof file, `public_input_path` is the path to the public input file,
and `verification_key_path` is the path to the verification key file.

For more instructions on how to submit proofs, check the [Submitting proofs guide](../3_guides/0_submitting_proofs.md).

## Risc0

### Dependencies

This guide assumes that:

- Risc0 toolchain installed (instructions [here](https://dev.risczero.com/api/zkvm/quickstart#1-install-the-risc-zero-toolchain))
- Risc0 project to generate the proofs (instructions [here](https://dev.risczero.com/api/zkvm/quickstart#2-create-a-new-project))
- Aligned installed (instructions [here](../1_introduction/1_try_aligned.md#quickstart))

### How to generate a proof

First, open the risc0 host file and add the following code to export proof, image id & public input needed by Aligned.

```rust
fn main() {
    // your code here

    // <proof_file_path> is the path where the proof will be saved
    // Note that we serialize receipt.inner to avoid serializing the public inputs along with the proof
    let serialized = bincode::serialize(&receipt.inner).expect("Failed to serialize the receipt");
    std::fs::write("<proof_file_path", serialized).expect("Failed to write proof file");

    // <METHOD_ID> is the method id of the function you want to prove
    // <method_id_file_path> is the path where the method id will be saved
    std::fs::write("<method_id_file_path>", convert(&<METHOD_ID>))
            .expect("Failed to write method_id file");

    // <pub_input_file_path> is the path where the public input will be saved
    std::fs::write("<pub_input_file_path>", receipt.journal.bytes)
            .expect("Failed to write pub_input file");
}


// Convert u32 array to u8 array for storage
pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
```

Note that METHOD_ID will be imported from guest, but it will be under a different name.

Then run the following command to generate the proof:

```bash
cargo run --release
```

### How to get the proof verified by Aligned

After generating the proof, you will have to find three different files:

- Proof file
- Image id file
- Public input file

Then, you can send the proof to the Aligned network by running the following command

```bash
aligned submit \
  --proving_system Risc0 \
  --proof <proof_file_path> \
  --vm_program <method_id_file_path> \
  --public_input <pub_input_file_path> \
  --batcher_url wss://batcher.alignedlayer.com \
  --proof_generator_addr <proof_generator_addr> \
  --rpc_url https://ethereum-holesky-rpc.publicnode.com \
  --payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

For more instructions on how to submit proofs, check the [Submitting proofs guide](../3_guides/0_submitting_proofs.md).

## ZkRust

`zkRust` is a CLI tool maintained by Aligned that aims to simplify the developing applications in Rust using zkVM's such as SP1 or Risc0.

zkRust can be installed directly by downloading the latest release binaries:

```sh
curl -L https://raw.githubusercontent.com/yetanotherco/zkRust/main/install_zkrust.sh | bash
```

Then, to get started you can create a workspace for your project in zkRust by running:

```sh
cargo new <PROGRAM_DIRECTORY>
```

It is that simple.

## Usage

To use zkRust, users specify a `fn main()` whose execution is proven within the zkVM. This function must be defined in a `main.rs` file in a directory with the following structure:

```
.
└── <PROGRAM_DIRECTORY>
    ├── Cargo.toml
    └── src
        └── main.rs
```

For using more complex programs you can import a separate lib/ crate into the `PROGRAM_DIRECTORY`

```
.
└── <PROGRAM_DIRECTORY>
    ├── Cargo.toml
    ├── lib/
    └── src
        └── lib
```

### Inputs and Outputs

The user may also define a `input()` and `output()` functions in addition to `main()`, that define code that runs outside of the zkVM, before and after the VM executes

- The `input()` function executes before the zkVM code is executed and allows the user to define inputs passed to the vm such as a deserialized Tx or data fetched from an external source at runtime.
- Within the `main()` (guest) function the user may write information from the computation performed in the zkVM to an output buffer to be used after proof generation.
- The `output()` defines code that allows the user to read the information written to that buffer of the and perform post-processing of that data.

The user may specify inputs into the VM (guest) code using `zk_rust_io::write()` as long on the type of rust object they are writing implements `Serializable`.

Within the `main()` function (guest) the user may read in the inputs by specifying `zk_rust_io::read()` and output data computed during the execution phase of the code within the VM (guest) program by specifying `zk_rust_io::commit()`.

To read the output of the output of the VM (guest) program you declare `zk_rust_io::out()`. The `zk_rust_io` crate defines function headers that are not inlined and are purely used as compile time symbols to ensure a user can compile their rust code before running it within one of the zkVMs available in zkRust.

To use the I/O imports import the `zk_rust_io` crate by adding the following to the `Cargo.toml` in your project directory.

```sh
zk_rust_io = { git = "https://github.com/yetanotherco/zkRust.git", version = "v0.1.0" }
```

## Example

### input.rs

```rust
use zk_rust_io;

pub fn input() {
    let pattern = "a+".to_string();
    let target_string = "an era of truth, not trust".to_string();

    // Write in a simple regex pattern.
    zk_rust_io::write(&pattern);
    zk_rust_io::write(&target_string);
}
```

### main.rs

```rust
use regex::Regex;
use zk_rust_io;

pub fn main() {
    // Read two inputs from the prover: a regex pattern and a target string.
    let pattern: String = zk_rust_io::read();
    let target_string: String = zk_rust_io::read();

    // Try to compile the regex pattern. If it fails, write `false` as output and return.
    let regex = match Regex::new(&pattern) {
        Ok(regex) => regex,
        Err(_) => {
            panic!("Invalid regex pattern");
        }
    };

    // Perform the regex search on the target string.
    let result = regex.is_match(&target_string);

    // Write the result (true or false) to the output.
    zk_rust_io::commit(&result);
}
```

### output.rs

```rust
use zk_rust_io;

pub fn output() {
    // Read the output.
    let res: bool = zk_rust_io::out();
    println!("res: {}", res);
}
```

To generate a proof of the execution of your code run the following:

- **Sp1**:

```sh
  cargo run --release -- prove-sp1 <PROGRAM_DIRECTORY_PATH> .
```

- **Risc0**:
  ```sh
  cargo run --release -- prove-risc0  <PROGRAM_DIRECTORY_PATH> .
  ```
  Make sure to have [Risc0](https://dev.risczero.com/api/zkvm/quickstart#1-install-the-risc-zero-toolchain) installed with version `v1.0.1`

For additional information on using zkRust and using it to submit proofs to Aligned see the [zkRust](https://github.com/yetanotherco/zkRust) Github Repository.
