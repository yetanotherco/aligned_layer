# Generating Proofs For Aligned

## SP1

### Dependencies

This guide assumes that:

- sp1 prover installed (instructions [here](https://succinctlabs.github.io/sp1/getting-started/install.html))
- sp1 project to generate the proofs
  (instructions [here](https://succinctlabs.github.io/sp1/generating-proofs/setup.html))
- aligned installed (instructions [here](../introduction/1_getting_started.md#quickstart))

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
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr <proof_generator_addr\
--rpc https://ethereum-holesky-rpc.publicnode.com \
--batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003``

Where `proof_path` is the path to the proof file and vm program path is the path to the elf file.

For more instructions on how to submit proofs, check the [Submitting proofs guide](../guides/0_submitting_proofs.md).

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
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr <proof_generator_addr> \
--rpc https://ethereum-holesky-rpc.publicnode.com \
--batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

Where proof path is the path to the proof file, `public_input_path` is the path to the public input file,
and `verification_key_path` is the path to the verification key file.

For more instructions on how to submit proofs, check the [Submitting proofs guide](../guides/0_submitting_proofs.md).

## Risc0

### Dependencies

This guide assumes that:

- Risc0 toolchain installed (instructions [here](https://dev.risczero.com/api/zkvm/quickstart#1-install-the-risc-zero-toolchain))
- Risc0 project to generate the proofs (instructions [here](https://dev.risczero.com/api/zkvm/quickstart#2-create-a-new-project))
- Aligned installed (instructions [here](../introduction/1_getting_started.md#quickstart))

### How to generate a proof

First, open the risc0 host file and add the following code to export image id & public input needed by Aligned.

```rust
fn main() {
    // your code here

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
  --conn wss://batcher.alignedlayer.com \
  --proof_generator_addr <proof_generator_addr> \
  --rpc https://ethereum-holesky-rpc.publicnode.com \
  --batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

For more instructions on how to submit proofs, check the [Submitting proofs guide](../guides/0_submitting_proofs.md).

## Halo2

### Dependencies

This guide assumes that:

- You are using PSE fork of the Halo2 [proof system](https://github.com/privacy-scaling-explorations/halo2).
- You have a strong understanding of Halo2 circuit development and are familiar with the Halo2 proof system.
- Aligned installed (instructions [here](../introduction/1_getting_started.md#quickstart)).

### Import the Halo2 fork library

Aligned supports verification of Halo2 proofs using the IPA and KZG backends. To verify Halo2 proofs on Aligned a description of your Halo2 circuits [constraint system](https://github.com/privacy-scaling-explorations/halo2/blob/main/halo2_backend/src/plonk/circuit.rs#L63) must be serialized and sent over the wire to Aligned in addition to the ciruits verification parameters, verification key, and public inputs. 

Aligned maintains its own fork of the PSE's Halo2 repository that provides helper methods to serialize and send Halo2 proofs to Aligned.

```rust
halo2_backend = { git = "https://github.com/yetanotherco/yet-another-halo2-fork.git", branch = "feat/serde_constraint_system" }
halo2_proofs = { git = "https://github.com/yetanotherco/yet-another-halo2-fork.git", branch = "feat/serde_constraint_system" }
```

### How to generate a proof

Once you have developed your circuit and generated its respective, prover key, verifier key, and public input.

You can add `prove_and_serialize_kzg_circuit` or `prove_and_serialize_ipa_circuit` from Aligned's Halo2 fork to generate a proof for your cicuit and serialize the circuit and its public inputs to be sent to Aligned.

```rust
fn main() {

  // your code here
  pub struct YourCircuit (pub Fr);

  impl Circuit<Fr> for YourCircuit {
    type Config = YourCircuitConfig;
    type FloorPlanner = YourCircuitPlanner;

    fn without_witnesses(&self) -> Self {
      // ... //
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        YourCircuitConfig::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), ErrorFront> {
      // ... //
    }
  }

  let circuit = YourCircuit(Fr::random(OsRng));
  let params = ParamsKZG::setup(4, OsRng);
  let compress_selectors = true;
  let vk = keygen_vk_custom(&params, &circuit, compress_selectors).expect("vk should not fail");
  let pk = keygen_pk(&params, vk.clone(), &circuit).expect("pk should not fail");
  let input: Vec<Vec<Fr>> = vec![vec![circuit.0]];
  prove_and_serialize_kzg_circuit(&params, &pk, &vk, circuit.clone(), &vec![input.clone()])
      .unwrap();
}
```

Then run the following command to generate the proof, parameters, and public inputs:

```bash
cargo run --release
```

The files will be saved within a `proof_files/` directory containing:
- `proof.bin`
- `params.bin`
- `public_input.bin`

### How to get the proof verified by Aligned

After generating the proof, you can send the proof for the respective proof systems of Halo2 to the Aligned network by running one of the following commands:

```bash
aligned submit \
  --proving_system Halo2KZG \
  --proof <proof_file_path> \
  --vk <method_id_file_path> \
  --public_input <pub_input_file_path> \
  --conn wss://batcher.alignedlayer.com \
  --proof_generator_addr <proof_generator_addr> \
  --rpc https://ethereum-holesky-rpc.publicnode.com \
  --batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

```bash
aligned submit \
  --proving_system Halo2IPA \
  --proof <proof_file_path> \
  --vk <method_id_file_path> \
  --public_input <pub_input_file_path> \
  --conn wss://batcher.alignedlayer.com \
  --proof_generator_addr <proof_generator_addr> \
  --rpc https://ethereum-holesky-rpc.publicnode.com \
  --batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

For more instructions on how to submit proofs, check the [Submitting proofs guide](../guides/0_submitting_proofs.md).