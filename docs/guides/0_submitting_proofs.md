# Submitting Proofs

Make sure you have Aligned installed as specified [here](../introduction/1_getting_started.md#Quickstart).

If you run the examples below, make sure you are in Aligned's repository root.

## Supported Verifiers

The following is the list of the verifiers currently supported by Aligned:

- :white_check_mark: gnark - Groth16 (with BN254)
- :white_check_mark: gnark - Plonk (with BN254 and BLS12-381)
- :white_check_mark: SP1 [(v1.0.1)](https://github.com/succinctlabs/sp1/releases/tag/v1.0.1)
- :white_check_mark: Risc0 [(v1.0.1)](https://github.com/risc0/risc0/releases/tag/v1.0.1)
- :white_check_mark: Halo2 - Plonk/KZG
- :white_check_mark: Halo2 - Plonk/IPA

Learn more about future verifiers [here](../architecture/0_supported_verifiers.md).

## 1. Import/Create Keystore file

If you already have a keystore file, you can ignore this section and start sending proofs. We give two examples of how to generate one. The first one using Foundry, and the second one using EigenLayer CLI

### Alternative 1: With foundry

You need to have installed [Foundry](https://book.getfoundry.sh/getting-started/installation).

- If you are creating a new account, create a private key with:

    ```bash
    cast wallet new-mnemonic --words 12
    ```

    It will show you a new mnemonic phrase and a public-private key pair, similar to the following example:

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

### Alternative 2: With EigenLayer CLI

- If you have the EigenLayer CLI installed, the keystore can be generated following [these](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation#import-keys) instructions. The key will be stored into `~/.eigenlayer/operator_keys`.

## 2. Fund the batcher

To be able to send proofs to Aligned using the Batcher, the user must fund its transactions. For this, there is a simple Batcher Payment System.

To use it, you can use the `aligned` CLI, as shown with the following example:

```bash
aligned deposit-to-batcher \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--chain holesky \
--keystore_path <keystore_path> \
--amount 0.1ether
```

These commands allow the usage of the following flags:

- `--payment_service_addr` to specify the address of the Batcher Payment Service smart contract.
- `--rpc_url` to specify the rpc url to be used.
- `--chain` to specify the chain id to be used. Could be holesky or devnet.
- `--keystore_path` the path to the keystore.
- `--amount` the number of ethers to transfer to the Batcher.
- Note: `--amount` flag parameter must be with the shown format. The amount followed by the `ether` keyword to specify how many ethers you wish to deposit to the Batcher.

After depositing funds, you can verify the Service has correctly received them by executing the following command:

```bash
aligned get-user-balance \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--user_addr <user_addr>
```

These commands allow the usage of the following flags:

- `--payment_service_addr` to specify the address of the Batcher Payment Service smart contract.
- `--rpc_url` to specify the rpc url to be used.
- `--user_addr` the address of the user that funded the Batcher.

## 3. Submit your proof to the batcher

This guide will focus on how to submit proofs using the Aligned CLI. To integrate the proof submission process into your application, check the [Aligned SDK guide](../guides/1_SDK.md).

Proof submission is done via the `submit` command of the Aligned CLI. The arguments for the submit command are:

* `proving_system`: The proving system corresponding to the proof you want to submit.
* `proof`: The path of the proof associated to the computation to be verified.
* `vm_program`: When the proving system involves the execution of a program in a zkVM, this argument is associated with the compiled program or some other identifier of the program. 
* `pub_input`: The path to the file with the public input associated with the proof.
* `batcher_url`: The batcher websocket URL.
* `rpc_url`: The RPC Ethereum node URL.
* `payment_service_addr`: The Ethereum address of the Batcher Payments System contract.
* `proof_generator_addr`: An optional parameter that can be used in some applications to avoid front-running.
* `batch_inclusion_data_directory_path`: An optional parameter indicating the directory where to store the batcher response data. If not provided, the folder with the responses will be created in the current directory.

### SP1 proof

The current SP1 version used in Aligned is v1.0.1.

The SP1 proof needs the proof file and the vm program file.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof <proof_file> \
--vm_program <vm_program_file> \
--batcher_url wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_to_ecdsa_keystore> \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

**Example**

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system SP1 \
--proof ./scripts/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./scripts/test_files/sp1/sp1_fibonacci.elf \
--batcher_url wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
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
--pub_input <pub_input_file> \
--batcher_url wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_to_ecdsa_keystore> \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

**Example**

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&
aligned submit \
--proving_system Risc0 \
--proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
--vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
--public_input ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--keystore_path ~/.aligned_keystore/keystore0 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
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
--batcher_url wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path <path_to_ecdsa_keystore> \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

**Examples**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBn254 \
--proof ./scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
--batcher_url wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0 \
--eth_rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
--public_input ./scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
--vk ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
--batcher_url wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
--proving_system Groth16Bn254 \
--proof ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
--public_input ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
--vk ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
--batcher_url wss://batcher.alignedlayer.com \
--keystore_path ~/.aligned_keystore/keystore0 \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

### Halo2 KZG and Halo2 IPA

The Halo2PlonkKzg and Halo2PlonkIpa proofs need the proof file, the public input file and the verification key file.

If you are using the Halo2PlonkKzg proving system, you need to specify the `--proving_system Halo2KZG` flag.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
  --proving_system Halo2KZG \
  --proof <proof_file_path> \
  --vk <method_id_file_path> \
  --public_input <pub_input_file_path> \
  --batcher_url wss://batcher.alignedlayer.com \
  --keystore_path <path_to_ecdsa_keystore> \
  --proof_generator_addr <proof_generator_addr> \
  --rpc_url https://ethereum-holesky-rpc.publicnode.com \
  --payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003 \
```

If you are using the Halo2PlonkIpa proving system, you need to specify the `--proving_system Halo2IPA` flag.

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
  --proving_system Halo2IPA \
  --proof <proof_file_path> \
  --vk <method_id_file_path> \
  --public_input <pub_input_file_path> \
  --batcher_url wss://batcher.alignedlayer.com \
  --keystore_path <path_to_ecdsa_keystore> \
  --proof_generator_addr <proof_generator_addr> \
  --rpc_url https://ethereum-holesky-rpc.publicnode.com \
  --payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

**Examples**:

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
  --proving_system Halo2KZG \
  --proof ./scripts/test_files/halo2_kzg/proof.bin \
  --vk ./scripts/test_files/halo2_kzg/params.bin \
  --public_input ./scripts/test_files/halo2_kzg/pub_input.bin \
  --batcher_url wss://batcher.alignedlayer.com \
  --keystore_path ~/.aligned_keystore/keystore0 \
  --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
  --rpc_url https://ethereum-holesky-rpc.publicnode.com \
  --payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

```bash
rm -rf ./aligned_verification_data/ &&
aligned submit \
  --proving_system Halo2IPA \
  --proof ./scripts/test_files/halo2_ipa/proof.bin \
  --vk ./scripts/test_files/halo2_ipa/params.bin \
  --public_input ./scripts/test_files/halo2_ipa/pub_input.bin \
  --batcher_url wss://batcher.alignedlayer.com \
  --keystore_path ~/.aligned_keystore/keystore0 \
  --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
  --rpc_url https://ethereum-holesky-rpc.publicnode.com \
  --payment_service_addr 0x815aeCA64a974297942D2Bbf034ABEe22a38A003
```

