# Generate Proofs

To generate example proofs you can use the following commands:

Note: You have to be in the root of the project

## Generate SP1 Proof

```bash
make generate_sp1_fibonacci_proof
```

## Generate Risc0 Proof

```bash
make generate_risc_zero_fibonacci_proof
```

```bash
make generate_risc_zero_empty_journal_proof
```

## Generate Gnark Groth16 BN254 Proof

```bash
make generate_gnark_groth16_bn254_proof
```

## Generate Gnark Plonk BN254 Proof

```bash
make generate_gnark_plonk_bn254_proof
```

## Generate Gnark Plonk BLS12-381 Proof

```bash
make generate_gnark_plonk_bls12_381_proof
```

## Generate Circom Groth16 BN256 Proof

```bash
make generate_circom_groth16_bn256_proof
```

You can find more details about Circom in [./circom_groth16_bn256_script/README.md](./circom_groth16_bn256_script/README.md).
