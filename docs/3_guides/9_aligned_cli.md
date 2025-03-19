# Aligned CLI Documentation

The Aligned CLI serves as an interface for users to interact with Aligned Layer.

This document serves as a reference for the commands of the Aligned CLI.

## Installation:

1. Download and install Aligned from the Aligned GitHub repo `https://github.com/yetanotherco/aligned_layer`:

    ```bash
    curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/install_aligned.sh | bash
    ```

2. A source command will be printed in your terminal after installation. Execute that command to update your shell environment.

3. Verify that the installation was successful:

    ```bash
    aligned --version
    ```

## Help:

To see the available commands, run:

```bash
aligned --help
```

To see the usage of a command, run:

```bash
aligned [COMMAND] --help
```

## CLI Commands

### **submit**

#### Description:

Submit a proof to the Aligned Layer batcher.

#### Command:

`submit [OPTIONS] --proving_system <proving_system> --proof <proof_file_path>`

#### Options:

- `--batcher_url <batcher_connection_address>`: Websocket URL for the Aligned Layer batcher  
  - Default: `ws://localhost:8080`  
  - Mainnet: `wss://mainnet.batcher.alignedlayer.com`
  - Holesky: `wss://batcher.alignedlayer.com`
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.
- `--proving_system <proving_system>`: Proof system of the submitted proof  
  - Possible values: `GnarkPlonkBls12_381`, `GnarkPlonkBn254`, `Groth16Bn254`, `SP1`, `Risc0`
- `--proof <proof_file_path>`: Path to the proof file.
- `--public_input <public_input_file_path>`: Path to the public input file.
- `--vk <verification_key_file_path>`: Path to the verification key file (required for specific proof systems).
- `--vm_program <vm_program_code_file_path>`: Path to the VM program code file (required for some specific proof systems).
- `--repetitions <n>`: Number of repetitions of the proof submission.  
  - Default: `1`
- `--proof_generator_addr <proof_generator_address>`: Proof generator address.  
  - Default: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (Anvil prefunded account 0)
- `--aligned_verification_data_path <aligned_verification_data_directory_path>`: Directory for aligned verification data.  
  - Default: `./aligned_verification_data/`
- `--keystore_path <path_to_local_keystore>`: Path to the local keystore.
- `--private_key <private_key>`: User's wallet private key.
- `--nonce <n>`: Proof nonce.
  - By default, the nonce is set automatically. By setting the nonce manually, you can perform a proof replacement.
- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.  
    - Default: `devnet`  
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`
- Max Fee allowed to be spent for each proof verification, use one of the following:
  - `--max_fee <max_fee (ether)>`: Specifies a `max_fee` in Ether.
  - `--default_fee_estimate`: Specifies a `max_fee` equivalent to the cost of 1 proof in a batch of size 10.
  - `--instant_fee_estimate`: Specifies a `max_fee` that ensures the proof is included instantly, equivalent to the cost of a proof in a batch of size 1.
  - `--custom_fee_estimate <amount_of_proofs_in_batch>`: Specifies a `max_fee` equivalent to the cost of 1 proof in a batch of size `num_proofs_in_batch`.


#### Example:

```bash
aligned submit  \
--proving_system Risc0 \
--proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
--vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
--public_input ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
--repetitions <BURST_SIZE> \
--keystore_path <KEYSTORE_PATH> \
--batcher_url wss://batcher.alignedlayer.com \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--max_fee 0.0013ether
```

---
### **verify-proof-onchain**

#### Description:

Check if a proof was verified by Aligned on Ethereum.

#### Command:

`verify-proof-onchain [OPTIONS] --aligned-verification-data <aligned_verification_data>`

#### Options:

- `--aligned-verification-data <aligned_verification_data>`: Path to the aligned verification data file.
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.
- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.  
    - Default: `devnet`  
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`

#### Example:

```bash
aligned verify-proof-onchain \
--aligned-verification-data ./aligned_verification_data/<VERIFICATION_DATA_FILE> \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```

---

### **get-vk-commitment**

#### Description:

Computes the verification data commitment from the verification data file.

#### Command:

`get-vk-commitment [OPTIONS] --verification_key_file <verification_key_file_path> --proving_system <proving_system>`

#### Options:

- `--verification_key_file <path_to_file>`: Path to the verification key file.
- `--proving_system <proving_system>`: Proof system of the verification data file.  
  - Possible values: `GnarkPlonkBls12_381`, `GnarkPlonkBn254`, `Groth16Bn254`, `SP1`, `Risc0`
- `--output <path_to_file>`: File path to write the output.

---

### **deposit-to-batcher**

#### Description:

Deposits Ethereum into the Aligned Layer's `BatcherPaymentService.sol` contract.

#### Command:

`deposit-to-batcher [OPTIONS] --keystore_path <path_to_local_keystore> --amount <amount_to_deposit>`

#### Options:

- `--keystore_path <path_to_local_keystore>`: Path to the local keystore.
- `--private_key <private_key>`: User's wallet private key.
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.
- `--amount <amount (ether)>`: Amount of Ether to deposit.
- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.  
    - Default: `devnet`  
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`
  
#### Example:

```bash
aligned deposit-to-batcher \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com \
--amount 0.5ether \
--keystore_path <KEYSTORE_PATH>
```

---

### **get-user-balance**

#### Description:

Retrieves the user's balance in the Aligned Layer's contract.

#### Command:

`get-user-balance [OPTIONS] --user_addr <user_ethereum_address>`


#### Options:

- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.  
    - Default: `devnet`  
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.
- `--user_addr`: User's Ethereum address.

#### Example:

```bash
aligned get-user-balance \
--user_addr <WALLET_ADDRESS> \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```

---

### **get-user-nonce**

#### Description:

Retrieves the user's current nonce from the Batcher.

#### Command:

`get-user-nonce [OPTIONS] --user_addr <user_ethereum_address>`

#### Options:

- `--user_addr <user_address>`: User's Ethereum address.
- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.
    - Default: `devnet`
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`

#### Example:

```bash
aligned get-user-nonce \
--user_addr <USER_ETH_ADDRESS> \
--network holesky
```

---

### **get-user-nonce-from-ethereum**

#### Description:

Retrieves the user's current nonce from the Blockhain, in the Batcher Payment Service Contract.

#### Command:

`get-user-nonce-from-ethereum [OPTIONS] --user_addr <user_ethereum_address>`

#### Options:

- `--user_addr <user_address>`: User's Ethereum address.
- One of the following, to specify which Network to interact with:
  - `--network <working_network_name>`: Network name to interact with.  
    - Default: `devnet`  
    - Possible values: `devnet`, `holesky`, `mainnet`
  - For a custom Network, you must specify the following parameters:
    - `--aligned_service_manager <aligned_service_manager_contract_address>`
    - `--batcher_payment_service <batcher_payment_service_contract_address>`
    - `--batcher_url <batcher_websocket_url>`
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.

#### Example:

```bash
aligned get-user-nonce-from-ethereum \
--user_addr <USER_ETH_ADDRESS> \
--network holesky \
--rpc_url https://ethereum-holesky-rpc.publicnode.com
```

---

### **get-user-amount-of-queued-proofs**

#### Description:

Retrieves the user's amount of queued proofs in the Batcher.

#### Command:

`get-user-amount-of-queued-proofs [OPTIONS] --user_addr <user_ethereum_address>`

#### Options:

- `--user_addr <user_address>`: User's Ethereum address.
- `--network <working_network_name>`: Network name to interact with.  
  - Default: `devnet`  
  - Possible values: `devnet`, `holesky`, `mainnet`
- `--rpc_url <RPC_provider_url>`: User's Ethereum RPC provider connection address. 
  - Default: `http://localhost:8545`
  - Mainnet: `https://ethereum-rpc.publicnode.com`
  - Holesky: `https://ethereum-holesky-rpc.publicnode.com`
  - Also, you can use your own Ethereum RPC providers.
- `--batcher_url <batcher_connection_address>`: Websocket URL for the Aligned Layer batcher  
  - Default: `ws://localhost:8080`  
  - Mainnet: `wss://mainnet.batcher.alignedlayer.com`
  - Holesky: `wss://batcher.alignedlayer.com`

#### Example:

```bash
aligned get-user-amount-of-queued-proofs  \
--user_addr <USER_ETH_ADDRESS> \
--network holesky \
--batcher_url wss://batcher.alignedlayer.com
```
