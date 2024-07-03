# Generating Proofs For Aligned

## SP1

### Dependencies

This guide assumes that:

- sp1 prover installed (instructions [here](https://succinctlabs.github.io/sp1/getting-started/install.html))
- sp1 project to generate the proofs
  (instructions [here](https://succinctlabs.github.io/sp1/generating-proofs/setup.html))
- aligned installed (instructions [here](../introduction/1_getting_started.md#quickstart))

### How to generate a proof

> Aligned only verifies SP1 in compressed version.
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
--proof_generator_addr <proof_generator_addr>
```

Where proof path is the path to the proof file and vm program path is the path to the elf file.

For more instructions on how to submit proofs, check the [Submitting proofs guide](../guides/0_submitting_proofs.md).
