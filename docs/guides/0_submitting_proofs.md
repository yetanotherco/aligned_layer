# Submitting Proofs

Make sure you have Aligned installed as specified [here](../introduction/1_getting_started.md#Quickstart).

If you run the examples below, make sure you are in Aligned's repository root.

## Supported Verifiers

The following is the list of the verifiers currently supported by Aligned:

- :white_check_mark: gnark - Groth16 (with BN254)
- :white_check_mark: gnark - Plonk (with BN254 and BLS12-381)
- :white_check_mark: SP1 [(v1.0.8-testnet)](https://github.com/succinctlabs/sp1/releases/tag/v1.0.8-testnet)
- :white_check_mark: Risc0 [(v1.0.1)](https://github.com/risc0/risc0/releases/tag/v1.0.1)

The following proof systems are going to be added soon:

- :black_square_button: Kimchi
- :black_square_button: Halo2 - Plonk/KZG
- :black_square_button: Halo2 - Plonk/IPA

## 1. Import/Create Keystore file

If you already have a keystore file, you can ignore this section and start sending proofs. We give two examples of how to generate one. The first one using Foundry, and the second one using EigenLayerCLI

### Alternative 1: With foundry

You need to have installed [Foundry](https://book.getfoundry.sh/getting-started/installation).

- If you are creating a new account. Create a private key with:

    ```bash
    cast wallet new-mnemonic --words 12
    ```

    It will show you a new mnemonic phrase, and a public-private key pair, similar to the following example:

    ```
    Phrase:
    test test test test test test test test test test test test
    
    Accounts:
    - Account 0:
    Address:     0xabcd...1234
    Private key: 0x1234...abcd
    ```
  
- Import the wallet using the private key previously generated, or whichever you want to use, and write a password to use it.

    ```bash
    mkdir -p ~/.aligned_keystore/
    cast wallet import ~/.aligned_keystore/keystore0 --interactive
    ```
  
  You have to paste your private key and set a password for the keystore file.

This will create the ECDSA keystore file in `~/.aligned_keystore/keystore0`

### Alternative 2: With EigenlayerCLI

- If you have the EigenLayer CLI installed, the keystore can be generated following [this](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation#import-keys) instructions. The key will be stored into `~/.eigenlayer/operator_keys`.

## 2. Fund the batcher

To be able to send proofs to Aligned using the Batcher, the user must fund its transactions. For this, there is a simple Batcher Payment System.

To use it you can use the `aligned` CLI, as shown with the following example:

```bash
aligned deposit-to-batcher \
--batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
--rpc https://ethereum-holesky-rpc.publicnode.com \
--chain holesky \
--keystore_path <keystore_path> \
--amount 0.1ether
```

This commands allows the usage of the following flags: 
- `--batcher_addr` to specify the address of the Batcher Payment Service smart contract.
- `--rpc` to specify the rpc url to be used.
- `--chain` to specify the chain id to be used. Could be holesky or devnet.
- `--keystore_path` the path to the keystore.
- `--amount` the amount of ethers to transfer to the Batcher.
- Note: `--amount` flag parameter must be with the shown format. The amount followed by the `ether` keyword to specify how many ethers you wish to deposit to the Batcher.

After depositing funds, you can verify the Service has correctly received them, executing the following command:

```bash
aligned get-user-balance \
--batcher_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
--rpc https://ethereum-holesky-rpc.publicnode.com \
--user_addr <user_addr>
```

This commands allows the usage of the following flags: 
- `--batcher_addr` to specify the address of the Batcher Payment Service smart contract.
- `--rpc` to specify the rpc url to be used.
- `--user_addr` the address of the user that funded the Batcher.

## 3. Send your proof to the batcher

### SP1 proof

The current SP1 version used in Aligned is v1.0.8-testnet.

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
--keystore_path <path_to_ecdsa_keystore> 
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

### Risc0 proof

The current Risc0 version used in Aligned is v1.0.1.

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
--keystore_path <path_to_ecdsa_keystore>
```

**Example**

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&                                                                                
aligned submit \
--proving_system Risc0 \
--proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
--vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
--conn wss://batcher.alignedlayer.com \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--keystore_path ~/.aligned_keystore/keystore0
```

### GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

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
--keystore_path <path_to_ecdsa_keystore>
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
