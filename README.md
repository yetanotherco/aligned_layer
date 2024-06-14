# Aligned

> [!CAUTION]
> To be used in testnet only.

## Table of Contents

- [Aligned](#aligned)
  - [Table of Contents](#table-of-contents)
  - [The Project](#the-project)
  - [How to use the testnet](#how-to-use-the-testnet)
  - [Register as an Aligned operator in testnet](#register-as-an-aligned-operator-in-testnet)
  - [Local Devnet Setup](#local-devnet-setup)
  - [Deploying Aligned Contracts to Holesky or Testnet](#deploying-aligned-contracts-to-holesky-or-testnet)
  - [Metrics](#metrics)
  - [Explorer](#explorer)
  - [Notes on project creation / devnet deployment](#notes-on-project-creation--devnet-deployment)
  - [Tests](#tests)
  - [Verify Proofs](#verify-proofs)
  - [FAQ](#faq)

## The Project

Aligned works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduce the proving overhead and add verifier overhead, now become economically feasable to verify thanks to Aligned.

## How to use the testnet

Download and install Aligned to send proofs in the testnet: 

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/install_aligned.sh | bash
```

If you are experiencing issues, upgrade by running the same command.

### Try it!

Download an example SP1 proof file with it's ELF file using:

```bash
curl -L https://raw.githubusercontent.com/yetanotherco/aligned_layer/main/batcher/aligned/get_proof_test_files.sh | bash
```

Send the proof with:

```bash
aligned submit \
--proving_system SP1 \
--proof ~/.aligned/test_files/sp1_fibonacci.proof \
--vm_program ~/.aligned/test_files/sp1_fibonacci-elf \
--conn wss://batcher.alignedlayer.com
```

### Run

#### SP1 proof

The SP1 proof needs the proof file and the vm program file.

```bash
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
aligned submit \
--proving_system SP1 \
--proof ./batcher/aligned/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./batcher/aligned/test_files/sp1/sp1_fibonacci-elf \
--conn wss://batcher.alignedlayer.com
```

#### GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

The GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proofs need the proof file, the public input file and the verification key file.

```bash
aligned submit \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--public_input <public_input_file> \
--vk <verification_key_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path]
```

**Examples**:

```bash
aligned submit \
--proving_system GnarkPlonkBn254 \
--proof ./batcher/aligned/test_files/plonk_bn254/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bn254/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bn254/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

```bash
aligned submit \
--proving_system GnarkPlonkBls12_381 \
--proof ./batcher/aligned/test_files/plonk_bls12_381/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bls12_381/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

```bash
aligned submit \
--proving_system Groth16Bn254 \
--proof ./batcher/aligned/test_files/groth16/ineq_1_groth16.proof \
--public_input ./batcher/aligned/test_files/groth16/ineq_1_groth16.pub \
--vk ./batcher/aligned/test_files/groth16/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com
```

### Creating a transaction from the CLI to verify proof in Ethereum
After running the commands of the previous section to submit proofs to the batcher, you will receive responses that will be written to disk in a JSON format inside the `<batch_inclusion_data_directory_path>`, for example `19f04bbb143af72105e2287935c320cc2aa9eeda0fe1f3ffabbe4e59cdbab691_0.json`. By default, the `batch_inclusion_data` directory will be created where the submit command is being executed, but you can specify it with the `<batch_inclusion_data_directory_path>` argument. To verify their inclusion in a batch, run the following command, replacing the `<path_to_batch_inclusion_data>` placeholder with the path to your response file.

```bash
aligned verify-proof-onchain \
--aligned-verification-data <path_to_your_verification_data> \
--rpc <holesky_rpc_url> \
--chain holesky
```

As a quick example for trying it out, you can use verification data provided by us in `./batcher/aligned/test_files/batch_inclusion_data/17bd5db82ef731ba3710b22df8e3c1ca6a5cde0a8d1ca1681664e4ff9b25574f_295.json`:

```bash
aligned verify-proof-onchain \
--aligned-verification-data ./batcher/aligned/test_files/batch_inclusion_data/17bd5db82ef731ba3710b22df8e3c1ca6a5cde0a8d1ca1681664e4ff9b25574f_295.json \
--rpc https://ethereum-holesky-rpc.publicnode.com \
--chain holesky
```

## Register as an Aligned operator in testnet

### Requirements

> [!NOTE]
> You must be whitelisted to become an Aligned operator.

This guide assumes you are already [registered as an operator with EigenLayer](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation).

#### Hardware Requirements

Minimum hardware requirements:

| Component     | Specification     |
|---------------|-------------------|
| **CPU**       | 16 cores          |
| **Memory**    | 32 GB RAM         |
| **Bandwidth** | 1 Gbps            |
| **Storage**   | 256 GB disk space |

### Dependencies

#### From Source (Recommended)

Ensure you have the following installed:
- [Go](https://go.dev/doc/install)
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

Also, you have to install the following dependencies for Linux:

- pkg-config
- libssl-dev

To install foundry, run:

```bash
make install_foundry
foundryup
```

#### Using Docker

Ensure you have the following installed:
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Configuration

#### From source (Recommended)

Update the following placeholders in `./config-files/config-operator.yaml`:
- `"<operator_address>"`
- `"<earnings_receiver_address>"`
- `"<ecdsa_key_store_location_path>"`
- `"<ecdsa_key_store_password>"`
- `"<bls_key_store_location_path>"`
- `"<bls_key_store_password>"`

`"<ecdsa_key_store_location_path>"` and `"<bls_key_store_location_path>"` are the paths to your keys generated with the EigenLayer CLI, `"<operator_address>"` and `"<earnings_receiver_address>"` can be found in the `operator.yaml` file created in the EigenLayer registration process.
The keys are stored by default in the `~/.eigenlayer/operator_keys/` directory, so for example `<ecdsa_key_store_location_path>` could be `/path/to/home/.eigenlayer/operator_keys/some_key.ecdsa.key.json` and for `<bls_key_store_location_path>` it could be `/path/to/home/.eigenlayer/operator_keys/some_key.bls.key.json`.

#### Using docker

Update the following placeholders in `./config-files/config-operator.docker.yaml`:
- `"<operator_address>"`
- `"<earnings_receiver_address>"`
- `"<ecdsa_key_store_password>"`
- `"<bls_key_store_password>"`

Make sure not to update the `ecdsa_key_store_location_path` and `bls_key_store_location_path`
as they are already set to the correct path.

Then create a .env file in `operator/docker/.env`.
An example of the file can be found in `operator/docker/.env.example`.

The file should contain the following variables:

| Variable Name               | Description                                                                                                   |
|-----------------------------|---------------------------------------------------------------------------------------------------------------|
| `ECDSA_KEY_FILE_HOST`       | Absolute path to the ECDSA key file. If generated from Eigen cli it should be in ~/.eigenlayer/operator_keys/ |
| `BLS_KEY_FILE_HOST`         | Absolute path to the BLS key file. If generated from Eigen cli it should be in ~/.eigenlayer/operator_keys/   |
| `OPERATOR_CONFIG_FILE_HOST` | Absolute path to the operator config file. It should be path to config-files/config-operator.docker.yaml      |

### Deposit Strategy Tokens

We are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To do so there are 2 options, either doing it through EigenLayer's website, and following their guide, or running the commands specified by us below.

You will need to stake a minimum of a 1000 Wei in WETH. We recommend to stake a maximum amount of 10 WETH. If you are staking more than 10 WETH please unstake any surplus over 10.

#### Option 1:
EigenLayer's guide can be found [here](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/liquid-restaking/restake-lsts).

#### Option 2:
If you have ETH and need to convert it to WETH you can use the following command, that will convert 1 Eth to WETH.
Make sure to have [foundry](https://book.getfoundry.sh/getting-started/installation) installed.
Change the parameter in ```---value``` if you want to wrap a different amount:

```bash
cast send 0x94373a4919B3240D86eA41593D5eBa789FEF3848 --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> --value 1ether
```

Here `<private_key>` is the placeholder for the ECDSA key specified in the output when generating your keys with the EigenLayer CLI.

Finally, to end the staking process, you need to deposit into the WETH strategy,
as shown in the Eigen guide.

<details>
  <summary>An alternative using the CLI (only when running without docker)</summary>

  Run the following command to deposit one WETH
  ```bash
  ./operator/build/aligned-operator deposit-into-strategy --config ./config-files/config-operator.yaml --strategy-address 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9 --amount 1000000000000000000
  ```
</details>

If you don't have Holesky Eth, these are some useful faucets:

- [Google Cloud for Web3 Holesky Faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)
- [Holesky PoW Faucet](https://holesky-faucet.pk910.de/)

### Start the operator

#### From Source (Recommended)

```
./operator/build/aligned-operator start --config ./config-files/config-operator.yaml
```

#### Using Docker

```bash
make operator_start_docker
```

### Unregister the operator from Aligned

To unregister the Aligned operator, run:

```bash
cast send --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> 0x3aD77134c986193c9ef98e55e800B71e72835b62 'deregisterOperator(bytes)' 0x00
 ```

 `<private_key>` is the one specified in the output when generating your keys with the EigenLayer CLI.

## Local Devnet Setup

### Dependencies

Ensure you have the following installed:

- [Go](https://go.dev/doc/install)
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [zap-pretty](https://github.com/maoueh/zap-pretty)
- [abigen](https://geth.ethereum.org/docs/tools/abigen)
- [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git)
- [jq](https://jqlang.github.io/jq/)
- [yq](https://github.com/mikefarah/yq)

To install [Go](https://go.dev/doc/install), [Rust](https://www.rust-lang.org/tools/install), [jq](https://jqlang.github.io/jq/) and [yq](https://github.com/mikefarah/yq) go to the provided links and follow the instructions.

Install Go dependencies ([zap-pretty](https://github.com/maoueh/zap-pretty), [abigen](https://geth.ethereum.org/docs/tools/abigen), [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git)):

```bash
make go_deps
```

Install [Foundry](https://book.getfoundry.sh/getting-started/installation):

```bash
make install_foundry
foundryup
```

Install necessary submodules and build all the FFIs for your OS:

```bash
make deps
```

If you want to rebuild the FFIs you can use:

```bash
make build_all_ffi
```

### Booting Devnet with Default configs

Before starting you need to setup an S3 bucket. More data storage will be tested in the future.

You need to fill the data in:

```batcher/aligned-batcher/.env```

And you can use this file as an example on how to fill it:

```batcher/aligned-batcher/.env.example```

After having the env setup, run in different terminals the following commands to boot Aligned locally:

```bash
make anvil_start_with_block_time
```

```bash
make aggregator_start
```

```bash
make operator_register_and_start
```

```bash
make batcher_start
```

If you need to start again the operator, and it's already registered, use:

```bash
make operator_start
```

If you want to start the explorer for the devnet, see how to run it using it's [documentation](#explorer) below.

### Send test proofs to batcher for testing

All these proofs are for testing purposes

Send 8 proofs each second:

```bash
make batcher_send_burst_groth16
```

Send Groth 16 proofs each 2 seconds:

```bash
make batcher_send_infinite_groth16
```

Send an individual Groth 16 proof:

```bash
make batcher_send_groth16_task
```

To send an individual test SP1 proof:

```bash
make batcher_send_sp1_task
```

### Detailed Testnet Deployment

#### Changing operator keys

Operator keys can be changed if needed.

To create a keystore, run:

```bash
cast wallet new-mnemonic
cast wallet import <keystore-name> --private-key <private-key>
```

To create an ECDSA keystore, run:

```bash
eigenlayer operator keys import --key-type ecdsa <keystore-name> <private-key>
```

To create a BLS keystore, run:

```bash
eigenlayer operator keys import --key-type bls <keystore-name> <private-key>
```

#### Aggregator

If you want to run the aggregator with the default configuration, run:

```bash
make aggregator_start
```

To start the aggregator with a custom configuration, run:

```bash
make aggregator_start CONFIG_FILE=<path_to_config_file>
```

#### Operator

Operator needs to register in both EigenLayer and Aligned. Then it can start verifying proofs.

##### Register into EigenLayer

To register an operator in EigenLayer Devnet with the default configuration, run:

```bash
make operator_register_with_eigen_layer
```

To register an operator in EigenLayer with a custom configuration, run:

```bash
make operator_register_with_eigen_layer CONFIG_FILE=<path_to_config_file>
```

##### Register into Aligned

To register an operator in Aligned with the default configuration, run:

```bash
make operator_register_with_aligned_layer
```

To register an operator in Aligned with a custom configuration, run:

```bash
make operator_register_with_aligned_layer CONFIG_FILE=<path_to_config_file>
```

##### Full Registration in Anvil with one command

To register an operator in EigenLayer and Aligned and deposit strategy tokens in EigenLayer with the default configuration, run:

```bash
make operator_full_registration
```

To register an operator in EigenLayer and Aligned and deposit strategy tokens in EigenLayer with a custom configuration, run:

```bash
make operator_full_registration CONFIG_FILE=<path_to_config_file>
```

##### Deposit Strategy Tokens in Anvil local devnet

There is an ERC20 token deployed in the Anvil chain to use as strategy token with EigenLayer.

To deposit strategy tokens in the Anvil chain with the default configuration, run:

```bash
make operator_mint_mock_tokens
make operator_deposit_into_mock_strategy
```

To deposit strategy tokens in the Anvil chain with a custom configuration, run:

```bash
make operator_mint_mock_tokens CONFIG_FILE=<path_to_config_file>
make operator_deposit_into_mock_strategy CONFIG_FILE=<path_to_config_file>
```

#### Deposit Strategy tokens in Holesky/Mainnet

EigenLayer strategies are available in [eigenlayer-strategies](https://holesky.eigenlayer.xyz/restake).

For Holesky, we are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To obtain HolETH and swap it for different strategies, you can use the following [guide](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/stage-2-testnet/obtaining-testnet-eth-and-liquid-staking-tokens-lsts).

#### Config

There is a default configuration for devnet purposes in `config-files/config.yaml`.
Also, there are 3 different configurations for the operator in `config-files/devnet/operator-1.yaml`, `config-files/devnet/operator-2.yaml` and `config-files/devnet/operator-3.yaml`.

The configuration file has the following structure:

```yaml
# Common variables for all the services
# 'production' only prints info and above. 'development' also prints debug
environment: <production/development>
aligned_layer_deployment_config_file_path: <path_to_aligned_layer_deployment_config_file>
eigen_layer_deployment_config_file_path: <path_to_eigen_layer_deployment_config_file>
eth_rpc_url: <http_rpc_url>
eth_ws_url: <ws_rpc_url>
eigen_metrics_ip_port_address: <ip:port>

## ECDSA Configurations
ecdsa:
  private_key_store_path: <path_to_ecdsa_private_key_store>
  private_key_store_password: <ecdsa_private_key_store_password>

## BLS Configurations
bls:
  private_key_store_path: <path_to_bls_private_key_store>
  private_key_store_password: <bls_private_key_store_password>

## Operator Configurations
operator:
  aggregator_rpc_server_ip_port_address: <ip:port> # This is the aggregator url
  address: <operator_address>
  earnings_receiver_address: <earnings_receiver_address> # This is the address where the operator will receive the earnings, it can be the same as the operator address
  delegation_approver_address: "0x0000000000000000000000000000000000000000"
  staker_opt_out_window_blocks: 0
  metadata_url: "https://yetanotherco.github.io/operator_metadata/metadata.json"
# Operators variables needed for register it in EigenLayer
el_delegation_manager_address: <el_delegation_manager_address> # This is the address of the EigenLayer delegationManager
private_key_store_path: <path_to_bls_private_key_store>
bls_private_key_store_path: <bls_private_key_store_password>
signer_type: local_keystore
chain_id: <chain_id>
```

#### Run

If you want to run the operator with the default configuration, run:

```bash
make operator_start
```

To start the operator with a custom configuration, run:

```bash
make operator_start CONFIG_FILE=<path_to_config_file>
```

### Batcher

#### Config

To run the batcher, you will need to set environment variables in a `.env` file in the same directory as the batcher (`batcher/aligned-batcher/`).

The necessary environment variables are:

| Variable Name         | Description                                                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| AWS_SECRET_ACCESS_KEY | Secret key to authenticate and authorize API requests to the AWS S3 Bucket.                                                    |
| AWS_REGION            | Geographical region where the AWS S3 Bucket will be accessed.                                                                  |
| AWS_ACCESS_KEY_ID     | Access key used in combination with the AWS_SECRET_ACCESS_KEY to authenticate and authorize API requests to the AWS S3 Bucket. |
| AWS_BUCKET_NAME       | Name of the AWS S3 Bucket.                                                                                                     |
| RUST_LOG              | Rust log level (info, debug, error, warn, etc.).                                                                               |

You can find an example `.env` file in [.env.example](batcher/aligned-batcher/.env.example)

You can configure the batcher in `config-files/config.yaml`:

```yaml
# Common variables for all the services
eth_rpc_url: <http_rpc_url>
eth_ws_url: <ws_rpc_url>
aligned_layer_deployment_config_file_path: <path_to_aligned_layer_deployment_config_file>

## Batcher Configurations
batcher:
  block_interval: <block_interval>
  batch_size_interval: <batch_size_interval>

## ECDSA Configurations
ecdsa:
  private_key_store_path: <path_to_ecdsa_private_key_store>
  private_key_store_password: <ecdsa_private_key_store_password>
```

#### Run

```bash
make batcher_start
```

### Send tasks

#### Sending a Task to the Batcher using our Rust TaskSender CLI

#### Send one SP1 proof

```bash
make batcher_send_sp1_task
```

#### Send one Groth 16 proof

```bash
make batcher_send_groth16_bn254_task
```

#### Send infinite Groth 16 proofs

```bash
make batcher_send_infinite_groth16
```

#### Send burst of Groth 16 proofs

```bash
make batcher_send_burst_groth16
```

#### Send specific proof

To install the batcher client to send a specific proof, run:

```bash
make install_batcher_client
```

The SP1 proof needs the proof file and the vm program file.
The GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proofs need the proof file, the public input file and the verification key file.

```bash
aligned \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--public-input <public_input_file> \
--vm_program <vm_program_file> \
--proof_generator_addr [proof_generator_addr] \
--aligned_verification_data_path [aligned_verification_data_path]
```

### Task Sender

#### Config

There is a default configuration for devnet purposes in `config-files/config.yaml`.

The configuration file have the following structure:

```yaml
# Common variables for all the services
# 'production' only prints info and above. 'development' also prints debug
environment: <production/development>
aligned_layer_deployment_config_file_path: <path_to_aligned_layer_deployment_config_file>
eigen_layer_deployment_config_file_path: <path_to_eigen_layer_deployment_config_file>
eth_rpc_url: <http_rpc_url>
eth_ws_url: <ws_rpc_url>
eigen_metrics_ip_port_address: <ip:port>

## ECDSA Configurations
ecdsa:
  private_key_store_path: <path_to_ecdsa_private_key_store>
  private_key_store_password: <ecdsa_private_key_store_password>
```

### Send PLONK BLS12_381 proof

To send a single PLONK BLS12_381 proof, run:

```bash
make send_plonk_bls12_381_proof
```

To send PLONK BLS12_381 proofs in loop, run:

```bash
make send_plonk_bls12_381_proof_loop
```

#### Send PLONK BN254 proof

To send a single PLONK BN254 proof, run:

```bash
make send_plonk_bn254_proof
```

To send PLONK BN254 proofs in loop, run:

```bash
make send_plonk_bn254_proof_loop
```

#### Send Groth 16 BN254 proof

To send a single Groth 16 BN254 proof, run:

```bash
make send_groth16_bn254_proof
```

To send Groth 16 BN254 proofs in loop, run:

```bash
make send_groth16_bn254_proof_loop
```

To send different Groth 16 BN254 proofs in loop, run:

```bash
make send_infinite_groth16_bn254_proof
```

#### Send SP1 proof

To send a single SP1 proof, run:

```bash
make send_sp1_proof
```

#### Send a specific proof

```bash
go run task_sender/cmd/main.go send-task \
--proving-system <plonk_bls12_381|plonk_bn254|groth16_bn254|sp1> \
--proof <proof_file> \
--public-input <public_input_file> \
--verification-key <verification_key_file> \
--config <config_file> \
--quorum-threshold <quorum_threshold> \
2>&1 | zap-pretty
```

#### Send a specific proof in loop

```bash
go run task_sender/cmd/main.go loop-tasks \
    --proving-system <plonk_bls12_381|plonk_bn254|groth16_bn254|sp1> \
    --proof <proof_file> \
    --public-input <public_input_file> \
    --verification-key <verification_key_file> \
    --config <config_file> \
    --quorum-threshold <quorum_threshold> \
    --interval <interval-in-seconds>
```

## Deploying Aligned Contracts to Holesky or Testnet

### Eigenlayer Contracts: Anvil

If EigenLayer contracts change, the anvil state needs to be updated with:

```bash
make anvil_deploy_eigen_contracts
```

You will also need to redeploy the MockStrategy & MockERC20 contracts:

```bash
make anvil_deploy_mock_strategy
```

### Eigenlayer Contracts: Holesky/Mainnet

These contracts are not deployed by Aligned. Current EigenLayer contracts:

- [Holesky Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/testnet-holesky/script/configs/holesky/Holesky_current_deployment.config.json)
- [Mainnet Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/script/configs/mainnet/Mainnet_current_deployment.config.json)

### Aligned Contracts: Anvil

When changing Aligned contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_aligned_contracts
```

To test the upgrade script for ServiceManager in the local devnet, run:

```bash
make anvil_upgrade_aligned_contracts
```

To test the upgrade script for RegistryCoordintator in the local devnet, run:

```bash
make anvil_upgrade_registry_coordinator
```

#### Aligned Contracts: Holesky/Mainnet

To deploy the contracts to Testnet/Mainnet, you will need to set environment variables in a `.env` file in the same directory as the deployment script (`contracts/scripts/`).

The necessary environment variables are:

| Variable Name                   | Description                                                           |
| ------------------------------- | --------------------------------------------------------------------- |
| `RPC_URL`                       | The RPC URL of the network you want to deploy to.                     |
| `PRIVATE_KEY`                   | The private key of the account you want to deploy the contracts with. |
| `EXISTING_DEPLOYMENT_INFO_PATH` | The path to the file containing the deployment info about EigenLayer. |
| `DEPLOY_CONFIG_PATH`            | The path to the deployment config file.                               |
| `OUTPUT_PATH`                   | The path to the file where the deployment info will be saved.         |

You can find an example `.env` file in [.env.example.holesky](contracts/scripts/.env.example.holesky)

Then run the following command:

```bash
make deploy_aligned_contracts
```

You need to complete the `DEPLOY_CONFIG_PATH` file with the following information:

```json
{
    "chainInfo": {
      "chainId": "<chain_id>"
    },
    "permissions" : {
      "owner": "<owner_address>",
      "aggregator": "<aggregator_address>",
      "upgrader": "<upgrader_address>",
      "churner": "<churner_address>",
      "ejector": "<ejector_address>",
      "deployer": "<deployer_address>",
      "initalPausedStatus": 0
    },
    "minimumStakes": [],  
    "strategyWeights": [],
    "operatorSetParams": [],
    "uri": ""
  }
```

You can find an example config file in `contracts/script/deploy/config/holesky/aligned.holesky.config.json`.

To upgrade the Service Manager Contract in Testnet/Mainnet, run:

```bash
make upgrade_aligned_contracts
```

To upgrade the Registry Coordinator in Testnet/Mainnet, run:

```bash
make upgrade_registry_coordinator
```

Make sure to set environment variables in a `.env` file in the same directory as the upgrade script (`contracts/scripts/`).

### Bindings

Also make sure to re-generate the Go smart contract bindings:

```bash
make bindings
```

### Deployment

To build go binaries run:

```bash
make build_binaries
```

## Metrics

### Aggregator Metrics

Aggregator metrics are exposed on the `/metrics` endpoint.

If you are using the default config, you can access the metrics on `http://localhost:9091/metrics`.

To run Prometheus and Grafana just run:

```bash
make run_metrics
```

Then you can access Grafana on `http://localhost:3000` with the default credentials `admin:admin`.

If you want to install Prometheus and Grafana manually, you can follow the instructions below.

To install Prometheus, you can follow the instructions on the [official website](https://prometheus.io/docs/prometheus/latest/getting_started/).

To install Grafana, you can follow the instructions on the [official website](https://grafana.com/docs/grafana/latest/setup-grafana/installation/).

## Explorer

### Minimum Requirements

- [Erlang 26](https://github.com/asdf-vm/asdf-erlang)
- [Elixir 1.16.2](https://elixir-ko.github.io/install.html), compiled with OTP 26
- [Docker](https://docs.docker.com/get-docker/)

### DB Setup

To setup the explorer, an installation of the DB is needed.

First you'll need to install docker if you don't have it already. You can follow the instructions [here](https://docs.docker.com/get-docker/).

The explorer uses a PostgreSQL database. To build and start the DB using docker, just run:

```bash
make build_db
```

This will build the docker image to be used as our database.

After this, both `make run_explorer` and `make run_devnet_explorer` (see [this](#running-for-local-devnet) for more details) will automatically start, setup and connect to the database, which will be available on `localhost:5432` and the data is persisted in a volume.

<details>

<summary>
  (Optional) The steps to manually execute the database are as follows...
</summary>

- Run the database container, opening port `5432`:

```bash
make run_db
```

- Configure the database with ecto running `ecto.create` and `ecto.migrate`:

```bash
make ecto_setup_db
```

- Start the explorer:

```bash
make run_explorer # or make run_devnet_explorer
```

</details>

<br>

In order to clear the DB, you can run:

```bash
make clean_db
```

If you need to dumb the data from the DB, you can run:

```bash
make dump_db
```

This will create a `dump.$date.sql` SQL script on the `explorer` directory with all the existing data.

Data can be recovered from a `dump.$date.sql` using the following command:

```bash
make recover_db
```

Then you'll be requested to enter the file name of the dump you want to recover already positioned in the `/explorer` directory.

This will update your database with the dumped database data.

### Extra scripts

If you want to fetch past batches that for any reason were not inserted into the DB, you will first need to make sure you have the ELIXIR_HOSTNAME .env variable configured. You can get the hostname of your elixir by running `elixir -e 'IO.puts(:inet.gethostname() |> elem(1))'`

Then you can run:

```bash
make explorer_fetch_old_batches
```

You can modify which blocks are being fetched by modify the parameters the `explorer_fetch_old_batches.sh` is being recieved

### Running for local devnet

To run the explorer for the local devnet, you'll need to have the devnet running (see [local devnet setup](#local-devnet-setup)) and the DB already setup.

To run the explorer, just run:

```bash
make run_devnet_explorer
```

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
You can access to a tasks information by visiting `localhost:4000/batches/:merkle_root`.

### Run with custom env / other devnets

Create a `.env` file in the `/explorer` directory of the project. The `.env` file needs to contain the following variables:

| Variable      | Description                                                                                     |
| ------------- | ----------------------------------------------------------------------------------------------- |
| `RPC_URL`     | The RPC URL of the network you want to connect to.                                              |
| `ENVIRONMENT` | The environment you want to run the application in. It can be `devnet`, `holesky` or `mainnet`. |
| `PHX_HOST`    | The host URL where the Phoenix server will be running.                                          |
| `DB_NAME` | The name of the postgres database. |
| `DB_USER` | The username of the postgres database. |
| `DB_PASS` | The password of the postgres database. |
| `DB_HOST` | The host URL where the postgres database will be running. |
| `ELIXIR_HOSTNAME` |  The hostname of your running elixir. Read [Extra Scripts](#extra-scripts) section for more details |

Then you can run the explorer with this env file config by entering the following command:

```make run_explorer```

### Send example data

If you want to have some data to see on it, you can start our infinite task sender, which will constantly send new proofs to the batcher.

```sh
make batcher_send_burst_groth16
```

## Notes on project creation / devnet deployment

Eigenlayer middleware was installed as a submodule with:

```sh
mkdir contracts
cd contacts
forge init . --no-commit
forge install Layr-Labs/eigenlayer-middleware@mainnet
```

Then to solve the issue <https://github.com/Layr-Labs/eigenlayer-middleware/issues/229>, we changed it to:

```forge install yetanotherco/eigenlayer-middleware@yac-mainnet --no-commit```

As soon as it gets fixed in mainnet we can revert it.

Base version of middleware used is ```7229f2b```.

The script to initialize the devnet can be found on  ```contracts/scripts/anvil```.

The addresses of the relevant contracts after running the anvil script is dumped on ```contracts/script/output/devnet```.

The state is backuped on ```contracts/scripts/anvil/state```.

Eigenlayer contract deployment is almost the same as the EigenLayer contract deployment on mainnet. Changes are described on the file.

## Tests

To run the go tests

```bash
make test
```

## Verify Proofs

### SP1

#### SP1 Dependencies

This guide assumes that:

- sp1 prover installed (instructions [here](https://succinctlabs.github.io/sp1/getting-started/install.html))
- sp1 project to generate the proofs (instructions [here](https://succinctlabs.github.io/sp1/generating-proofs/setup.html))
- aligned layer repository cloned:

    ```bash
    git clone https://github.com/yetanotherco/aligned_layer.git
    ```

#### How to generate a proof

> AlignedLayer only verifies SP1 in compressed version.
> You can check you are using compressed by opening script/src/main.rs
and check that the proof is generated with `client.prove_compressed` instead of `client.prove`.

First, open a terminal and navigate to the script folder in the sp1 project directory

Then, run the following command to generate a proof:

```bash
cargo run --release
```

#### How to get the proof verified by AlignedLayer

After generating the proof, you will have to find two different files:

- proof file: usually found under `script` directory, with the name `proof.json` or similar
- elf file: usually found under `program/elf/` directory

Then, you can send the proof to the AlignedLayer network by running the following command
from `batcher/aligned` folder inside the AlignedLayer repository directory:

```bash
cargo run --release -- \
--proving_system SP1 \
--proof <proof_path> \
--vm_program <vm_program_path> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr] \
--aligned_verification_data_path [aligned_verification_data_path]
```

## FAQ

### What is the objective of Aligned?
    
Aligned’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain the zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t know what that future will look like, but we are certain it will be in Ethereum. The question we want to share is: If we are certain zero-knowledge proofs are the future of Ethereum but we are not certain which of the many possible zero-knowledge futures will win. How can we build an infrastructure for Ethereum to be compatible with any future zero-knowledge proving system?
    
### What is the throughput of Aligned?
    
Aligned runs the verifier’s code natively. The verification time depends on the proof system, program run, and public input. Generally, most verifiers can be run in the order of ms on consumer-end hardware. We can optimize the code for speed and leverage parallelization by running it natively. Taking 3 ms per proof, Aligned could verify 300 proofs per second and, using parallelization, over 10,000 proofs per second.
    
### How does the throughput of Aligned compare with Ethereum?
    
Ethereum runs on top of the EVM. Each block is limited to 30,000,000 gas. Since the most efficient proof systems take at least 250,000 gas, Ethereum can verify 120 proofs per block. Aligned runs the code natively and leverages parallelization, reaching 10,000 proofs in the same period.
    
### Is Aligned an Ethereum L2?
    
Aligned is related to Ethereum but is not an L2 since it does not produce blocks. It is a decentralized network of verifiers.
    
### Does Aligned compete with L2s?
    
No. Aligned is a decentralized network of verifiers and has proof aggregation. It does not produce blocks or generate proofs of execution. Aligned provides L2s with fast and cheap verification for the proofs they generate, reducing settlement costs and enhancing cross-chain interoperability with quick and cheap bridging.
    
### What are the costs for Aligned?
    
The costs depend on task creation, aggregated signature or proof verification, and reading the results. The cost C per proof by batching N proofs is roughly:
    
$$
  C =\frac{C_{task} + C_{verification}}{N} + C_{read}
$$
    
Batching 1024 proofs using Aligned’s fast mode can cost around 2,100 gas in Ethereum (for a gas price of 8 gwei/gas and ETH = $3000, $0.05). As a helpful comparison, a transaction in Ethereum costs 21,000 gas, so you get proof verification for 1/10th of the transaction cost!
    
### Why do you have a fast and slow mode?
    
The fast mode is designed to offer very cheap verification costs and low latency. It uses crypto-economic guarantees provided by restaking; costs can be as low as 2100 gas. The slow mode works with proof aggregation, with higher fees and latency, and achieves the complete security of Ethereum. We verify an aggregated BLS signature (around 113,000 gas) in the fast mode. We verify an aggregated proof (around 300,000 gas) in the slow mode.
    
### Why don’t you run Aligned on top of a virtual machine?
    
Running on a virtual machine adds complexity to the system and an additional abstraction layer. It can also reduce Aligned's throughput, which is needed to offer really fast and cheap verification.
    
### Why don’t you build Aligned on top of a rollup?
    
The main problem with settling on top of a rollup is that you still need confirmation in Ethereum, which adds latency to the process. Besides, most rollups are not fully decentralized, even if they were, not to the extent of Ethereum. Aligned also achieves an already low verification cost in Ethereum, so it would not be convenient to build Aligned on top of a rollup in terms of latency, costs, and decentralization.
    
An L2 needs to use the EVM to settle in Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.
    
The EVM is not designed for ZK Verification, so most verifications are expensive.
    
To solve this, for pairing-based cryptography, Ethereum has added a precompile for verifications using the curve BN254.
    
But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast Starks need efficient hashing for fields. Which is the best field? Mersenne’s? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?
    
The amount of progress in the field is big, and nobody can predict the endgame.
    
Even more, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems.
    
Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or one hash not created yet.
    
Aligned solves all of this. No matter how or what you want to prove, it can be verified efficiently here while still inheriting the security of Ethereum as other L2s.
    
### Is Aligned an aggregation layer?
    
Aligned provides proof aggregation as part of its slow mode, a feature shared with all aggregation layers. However, Aligned offers a unique fast mode designed to provide cheap and low-latency proof verification, leveraging the power of restaking. Aligned is a decentralized network designed to verify zero-knowledge proofs and uses recursive proof aggregation as one of its tools. 
    
### What proof systems do you support?
    
Aligned is designed to support any proof system. Currently supported ones are Groth 16 and Plonk (gnark), SP1, Halo 2 (IPA and KZG)
    
### How hard is it to add new proof systems?
    
Aligned is designed to make adding new proof systems easy. The only thing needed is the verifier function, which is written in a high-level language like Rust. For example, we could integrate Jolt into one of our testnets just a few hours after it was released.
    
### What are BLS signatures?
    
[Boneh-Lynn-Shacham](https://en.wikipedia.org/wiki/BLS_digital_signature) is a cryptographic signature that allows a user to verify that a signer is authentic. It relies on elliptic curve pairings and is used by Ethereum due to its aggregation properties.
    
### How does Aligned work?
    
The flow for fast verification is as follows:
    
1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum)
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments to all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verify all the proofs. 
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
   
### What is restaking?
    
EigenLayer introduced the concept of Restaking. It allows Ethereum’s validators to impose additional slashing conditions on their staked ETH to participate in Actively Validated Services (AVS) and earn additional rewards. This creates a marketplace where applications can rent Ethereum's trust without competing for blockspace. Aligned is an example of an AVS.
    
### How can I verify proofs in Aligned?
    
You can verify proofs in Aligned using our CLI.
    
### Can you provide an estimate of Aligned’s savings?
    
In Ethereum (does not include access cost): 
    
- Groth 16 proofs: 250,000 gas
- Plonk/KZG proofs: >300,000 gas
- STARKs: >1,000,000 gas
- Binius/Jolt: too expensive to run!
    
In Aligned, fast mode:
    
- Just one proof (any!): 120,000 gas
- Batching 1024 proofs: 120 gas + reading cost
    
It’s over 99% savings!
    
### I want to verify just one proof. Can I use Aligned for cheap and fast verification?
    
Yes!
    
### Is Aligned open-source?
    
Yes!
    
### What are the goals of Aligned?
    
Aligned is an infrastructure that offers fast and cheap verification for zero-knowledge and validity proofs. It can take any proof system and verify it cheaply and fast.
    
This means that what Aligned wants to achieve is to allow anyone to build zk applications. This can only be achieved by:
    
- Reducing operational costs when maintaining a zk application -> anyone can afford to build zk apps.
- Offering more options so developers can choose how they want to build their protocols -> everyone can choose their tools.
- Offer the latest zk that allows anyone to build zk applications by just proving rust -> anyone can code a zk application.
    
### What’s the role of Aligned in Ethereum?
    
Aligned’s role is to help advance the adoption of zero-knowledge proofs in Ethereum, increase verification throughput, and reduce on-chain verification time and costs. Aligned can easily incorporate proof systems without any further changes in Ethereum. In a more straightforward analogy, Aligned is like a GPU for Ethereum. 
    
### What is proof recursion?
    
Zero-knowledge proofs let you generate proofs that show the correct execution of programs. If a program is the verification of a proof, then we will be getting a proof that we verified the proof and the result was valid. The validity of the second proof implies the validity of the original proof. This is the idea behind proof recursion, and it can be used with two main goals:
    
1. Convert one proof type to another (for example, a STARK proof to a Plonk proof) either to reduce the proof size, have efficient recursion, or because the proof system cannot be verified where we want.
2. Proof aggregation: if we have to verify N proofs on-chain, we can generate a single proof that we verified the N proofs off-chain and just check the single proof in Ethereum.
    
Proof recursion is the primary tool of Aligned’s slow mode.
    
### What are the use cases of Aligned?
    
Among the possible use cases of Aligned, we have:
    
Soft finality for Rollups and Appchains, fast bridging, new settlement layers (use Aligned + EigenDA) for Rollups and Intent-based systems, P2P protocols based on SNARKs such as payment systems and social networks, alternative L1s interoperable with Ethereum, Verifiable Machine Learning, cheap verification and interoperability for Identity Protocols, ZK Oracles, new credential protocols such as zkTLS based systems, ZK Coprocessor, encrypted Mempools using SNARKs to show the correctness of the encryption, protocols against misinformation and fake news, and on-chain gaming.
    
### Why build Aligned on top of Ethereum?
    
Ethereum is the most decentralized and most significant source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.
    
### Why EigenLayer?
    
We believe Ethereum is the best settlement layer, and zero-knowledge will play a key role in helping it become the settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs, but how can we build such a decentralized network that will help Ethereum? Creating a new L1 doesn’t benefit Ethereum because it will add new trust assumptions to the Ethereum protocols relying on it. So, if we must have:
1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in Ethereum
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### How does it compare to the Polygon aggregation layer?

Aligned is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and help it increase its capabilities.
