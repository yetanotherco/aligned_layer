# Submitting Proofs

## Send proofs

You can check the list of supported verifiers [here](architecture/0_supported_verifiers.md).

Make sure you have Aligned installed as specified [here](../introduction/1_getting_started.md#Quickstart).

If you run the examples below, make sure you are in Aligned's repository root.

### 1. Import/Create Keystore file

If you already have a keystore file, you can ignore this section and start sending proofs. We give two examples of how to generate one. The first one using Foundry, and the second one using EigenLayerCLI

#### Alternative 1: With foundry

Install foundry following this guide:

Install [Foundry](https://book.getfoundry.sh/getting-started/installation):

- If you are creating a new account. Create a private key with:

```bash
cast wallet new-mnemonic --words 12
```

If you are using this wallet outside testnet, write down the mnemonic phrase given by anvil

- Import the wallet using the private key previously generated, or whichever you want to use, and write a password to use it.

```bash
mkdir -p ~/.aligned_keystore/
cast wallet import --private-key <YOUR_ECDSA_PRIVATE_KEY>  ~/.aligned_keystore/keystore0
```

This will create the ECDSA keystore file in `~/.aligned_keystore/keystore0`

#### Alternative 2: With EigenlayerCLI

- If you have the EigenLayer CLI installed, the keystore can be generated following [this](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation#import-keys) instructions. The key will be stored into `~/.eigenlayer/operator_keys`.

### 2. Fund the batcher

To be able to send proofs to Aligned using the batcher, the user must fund its transactions. For this, there is a simple Batcher Payment System.

To use it you can use the `aligned` CLI, as shown with the following example:

```bash
aligned deposit-to-batcher --keystore_path <keystore_path> --amount 0.1ether
```

This commands also allows the usage of the flags: 
- `--batcher_addr` to specify the address of the Batcher Payment Service smart contract.
- `--rpc` to specify the rpc url to be used.
- `--chain` to specify the chain id to be used.
- Note: `--amount` flag parameter must be with the shown format, followed by the `ether` keyword to specify how many ethers you wish to deposit to the batcher.

After depositing funds, you can verify the Service has correctly received them, executing the following command:
```bash
cast call <payment_service_smart_contract_address> "UserBalances(address)(uint256)" <address>
```

### 3. Send your proof to the batcher

#### SP1 proof

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
--proof ./scripts/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./scripts/test_files/sp1/sp1_fibonacci.elf \
--conn wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0
```

#### Risc0 proof

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
--proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
--vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--keystore_path ~/.aligned_keystore/keystore0
```

#### GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

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
--proof ./scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Groth16Bn254 \
--proof ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
--public_input ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
--vk ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0
```
