# Circom

# Circom Groth16 BN256 Script

The proof contained here is generated using the steps from [snarkjs repository](https://github.com/iden3/snarkjs) guide.

The example uses the following dependencies versions:

- Node version `v22.16.0`
- Circom version `2.2.2`
- Snarkjs version `0.7.5`

You can find how to install all dependencies in the snarkjs repository.

## Powers Of Tau Setup

You can run the following command from the repository root to create the setup:

```bash
make generate_circom_groth16_bn256_setup
```

## Generate the Circuit

You can modify `circuit.circom` and `input.json` files to create your own circuit and input.

## Generate the Proof

You can run the following command from the repository root to generate the proof:

```bash
make generate_circom_groth16_bn256_proof
```

This will generate the following files `proof.json`, `public.json`, and `verification_key.json` that can be sent to Aligned.

## Send the Proof to Aligned

You can run the following command from the repository root to send the proof to Aligned:

```bash
make batcher_send_circom_groth16_bn256_task
```