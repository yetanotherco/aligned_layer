# Submitting Proofs

Make sure you have Aligned installed as specified [here](../1_introduction/1_try_aligned.md#quickstart).

If you run the examples below, make sure you are in Aligned's repository root.

You can check your submitted proofs on [Mainnet Explorer](https://explorer.alignedlayer.com) and [Holesky Explorer](https://holesky.explorer.alignedlayer.com).

## Supported Verifiers

The following is the list of the verifiers currently supported by Aligned:

- :white_check_mark: gnark - Groth16 (with BN254)
- :white_check_mark: gnark - Plonk (with BN254 and BLS12-381)
- :white_check_mark: SP1 [(v3.0.0)](https://github.com/succinctlabs/sp1/releases/tag/v3.0.0)
- :white_check_mark: Risc0 [(v1.1.2)](https://github.com/risc0/risc0/releases/tag/v1.1.2)

Learn more about future verifiers [here](../2_architecture/0_supported_verifiers.md).

## 1. Import/Create Keystore file

If you already have a keystore file, you can ignore this section and start sending proofs. We give two examples of how to generate one. The first one using Foundry, and the second one using EigenLayer CLI

### Alternative 1: With foundry

You need to have installed [Foundry](https://book.getfoundry.sh/getting-started/installation).

{% hint style="warning" %}
When creating a new wallet keystore and private key please use strong passwords for your own protection.
{% endhint %}

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

## 2. Send funds to Aligned

To send proofs to Aligned using the Batcher, the user must first deposit some funds in Aligned to pay for the verification of his proofs.

To use it, you can use the `aligned` CLI, as shown with the following example:

```bash
aligned deposit-to-batcher \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--network holesky \
--keystore_path <keystore_path> \
--amount 0.1ether
```

This command allows the usage of the following flags:

- `--rpc_url` to specify the rpc url to be used.
- `--network` to specify the network to be used. Can be `devnet`, `holesky` or `mainnet`.
- `--keystore_path` the path to the keystore.
- `--amount` the number of ethers to transfer to the Batcher.
- Note: `--amount` flag parameter must be with the shown format, `XX.XXether`.

After depositing funds, you can verify the Service has correctly received them by executing the following command:

```bash
aligned get-user-balance \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--network holesky \
--user_addr <user_addr>
```

These commands allow the usage of the following flags:

- `--rpc_url` to specify the rpc url to be used.
- `--network` to specify the network to be used. Can be `devnet`, `holesky` or `mainnet`.
- `--user_addr` the address of the user that funded the Batcher.

## 3. Submit your proof to the batcher

This guide will focus on how to submit proofs using the Aligned CLI. To integrate the proof submission process into your application, check the [First Aligned Application tutorial](../3_guides/2_build_your_first_aligned_application.md) where we explain how to generate and submit a proof using the Aligned SDK.

Proof submission is done via the `submit` command of the Aligned CLI. The arguments for the submit command are:

* `proving_system`: The proving system corresponding to the proof you want to submit.
* `proof`: The path of the proof associated to the computation to be verified.
* `vm_program`: When the proving system involves the execution of a program in a zkVM, this argument is associated with the compiled program or some other identifier of the program.
* `pub_input`: The path to the file with the public input associated with the proof.
* `batcher_url`: The batcher websocket URL. Can be:
  * mainnet: `wss://mainnet.batcher.alignedlayer.com`
  * holesky: `wss://batcher.alignedlayer.com`
  * devnet: `ws://localhost:8080`
* `network` to specify the network to be used. Can be `devnet`, `holesky` or `mainnet`.
* `rpc_url`: The RPC Ethereum node URL.
* `proof_generator_addr`: An optional parameter that can be used in some applications to avoid front-running.
* `batch_inclusion_data_directory_path`: An optional parameter indicating the directory where to store the batcher response data. If not provided, the folder with the responses will be created in the current directory.

### SP1 proof

The current SP1 version used in Aligned is `v3.0.0`.

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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```

### Risc0 proof

The current Risc0 version used in Aligned is `v1.1.2`.

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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```

**NOTE**: As said above, Aligned currently supports Risc0 proofs from `risc0-zkvm` version `v1.1.2`. For generating proofs using `cargo risc-zero` please ensure you are using `v1.1.2` or your proof will not be verified. 

If you can't install `cargo-risczero` `v1.1.2`, you can manually modify your `cargo.toml` on the host project to point to `v1.1.2`:

```toml
risc0-zkvm = { git = "https://github.com/risc0/risc0", tag = "v1.1.2", default-features = false, features = [
    "prove",
] }
```
- Note: In Risc0 verification `--pub_input` contains the bytes of the `receipt.journal.bytes` which contains both the public input (`env::read()`) and public output (`env::commit()`) values of a program executed in the Risc0 VM. If your Risc0 program contains public outputs, but no public inputs you still need to submit the serialized `receipt.journal.bytes` with your proof using the Aligned CLI for your proof to be verified.

**Example**

```bash
rm -rf ~/.aligned/aligned_verification_data/ &&
aligned submit \
--proving_system Risc0 \
--proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
--vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
--public_input ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
--batcher_url wss://batcher.alignedlayer.com \
--aligned_verification_data_path ~/.aligned/aligned_verification_data \
--keystore_path ~/.aligned_keystore/keystore0 \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
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
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```
