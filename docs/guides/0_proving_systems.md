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

**Example**

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ./batcher/aligned/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./batcher/aligned/test_files/sp1/sp1_fibonacci-elf \
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
--proof ./batcher/aligned/test_files/plonk_bn254/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bn254/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bn254/plonk.vk \
--conn wss://batcher.alignedlayer.com
```


**Plonk BLS12-381 example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./batcher/aligned/test_files/plonk_bls12_381/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bls12_381/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

**Groth16 BN254 example**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Groth16Bn254 \
--proof ./batcher/aligned/test_files/groth16/ineq_1_groth16.proof \
--public_input ./batcher/aligned/test_files/groth16/ineq_1_groth16.pub \
--vk ./batcher/aligned/test_files/groth16/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com
```
