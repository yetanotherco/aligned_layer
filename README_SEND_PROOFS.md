# Send proofs

Make sure you have Aligned installed as specified [here](./README.md#how-to-use-the-testnet).

If you run the examples below, make sure you are in Aligned's repository root.

## Keystore file
You will need a ECDSA keystore file to manage your private keys securely, which is needed for signing messages sent to the batcher.
If you already have a keystore file, you can ignore this section and start sending proofs.

- If you have the EigenLayer CLI installed, the keystore can be generated following [this](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation#import-keys) instructions. The key will be stored into `~/.eigenlayer/operator_keys`.
- If you have `foundry` installed, you can run the following:
```bash
cast wallet import --private-key <YOUR_ECDSA_PRIVATE_KEY> <KEYSTORE_FILE_NAME>
```
This will create the ECDSA keystore file in `~/.foundry/keystores`

As an example,

```bash
cast wallet import --private-key 6842fb8f5fa574d0482818b8a825a15c4d68f542693197f2c2497e3562f335f6 test 
```

will create a keystore file `~/.foundry/keystores/test`

## SP1 proof

The SP1 proof needs the proof file and the vm program file.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof <proof_file> \
--vm_program <vm_program_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_yo_ecdsa_keystore> 
```

**Example**

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ./batcher/aligned/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./batcher/aligned/test_files/sp1/sp1_fibonacci-elf \
--conn wss://batcher.alignedlayer.com \
--keystore_path ./config-files/anvil.ecdsa.key.json
```

## Risc0 proof

The Risc0 proof needs the proof file and the vm program file (vm program file is the image id).

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Risc0 \
--proof <proof_file> \
--vm_program <vm_program_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_yo_ecdsa_keystore>
```

**Example**

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&                                                                                
aligned submit \
--proving_system Risc0 \
--proof ./batcher/aligned/test_files/risc_zero/risc_zero_fibonacci.proof \
--vm_program ./batcher/aligned/test_files/risc_zero/fibonacci_id.bin \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--keystore_path ./config-files/anvil.ecdsa.key.json
```

## GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

The GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proofs need the proof file, the public input file and the verification key file.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system <GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--public_input <public_input_file> \
--vk <verification_key_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_yo_ecdsa_keystore>
```

**Examples**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBn254 \
--proof ./batcher/aligned/test_files/plonk_bn254/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bn254/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bn254/plonk.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ./config-files/anvil.ecdsa.key.json
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./batcher/aligned/test_files/plonk_bls12_381/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bls12_381/plonk.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ./config-files/anvil.ecdsa.key.json
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Groth16Bn254 \
--proof ./batcher/aligned/test_files/groth16/ineq_1_groth16.proof \
--public_input ./batcher/aligned/test_files/groth16/ineq_1_groth16.pub \
--vk ./batcher/aligned/test_files/groth16/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ./config-files/anvil.ecdsa.key.json
```
