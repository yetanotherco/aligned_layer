# Supported verifiers

The following is the list of the verifiers currently supported by Aligned:

- :white_check_mark: gnark - Groth16 (with BN254)
- :white_check_mark: gnark - Plonk (with BN254 and BLS12-381)
- :white_check_mark: SP1

The following proof systems are going to be added soon:

- :black_square_button: Risc0
- :black_square_button: Kimchi
- :black_square_button: Halo2 - Plonk/KZG
- :black_square_button: Halo2 - Plonk/IPA

## SP1 proof

These are [STARK proofs](https://eprint.iacr.org/2018/046) that attest to the validity of the execution of a given program over the SP1 virtual machine. 

The SP1 proving system needs the proof file and the vm program file.
The arguments for SP1 are
```
--proving_system SP1 
--proof <proof_file> \
--vm_program <elf_file> \
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--vm_program <vm_program_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path]
```

**example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ./scripts/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./scripts/test_files/sp1/sp1_fibonacci.elf \
--conn wss://batcher.alignedlayer.com
```

## GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proving systems need the proof file, the public input file and the verification key file.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--public_input <public_input_file> \
--vk <verification_key_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path]
```

**Plonk BN254 example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBn254 \
--proof ./scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

**Plonk BLS12-381 example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

**Groth16 BN254 example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Groth16Bn254 \
--proof ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
--public_input ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
--vk ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com
```
