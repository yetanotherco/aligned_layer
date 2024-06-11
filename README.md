# Aligned

> [!CAUTION]
> To be used in testnet only.

## Table of Contents

- [Aligned](#aligned)
  - [Table of Contents](#table-of-contents)
  - [The Project](#the-project)
  - [How to use the testnet](#how-to-use-the-testnet)
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
aligned \
--proving_system SP1 \
--proof ~/.aligned/test_files/sp1_fibonacci.proof \
--vm_program ~/.aligned/test_files/sp1_fibonacci-elf \
--conn wss://batcher.alignedlayer.com
```

### Run

#### SP1 proof

The SP1 proof needs the proof file and the vm program file.

```bash
aligned \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--vm_program <vm_program_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr]
```

**Example**

```bash
aligned \
--proving_system SP1 \
--proof ./batcher/aligned/test_files/sp1/sp1_fibonacci.proof \
--vm_program ./batcher/aligned/test_files/sp1/sp1_fibonacci-elf \
--conn wss://batcher.alignedlayer.com
```

#### GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254

The GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proofs need the proof file, the public input file and the verification key file.

```bash
aligned \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254> \
--proof <proof_file> \
--public_input <public_input_file> \
--vk <verification_key_file> \
--conn wss://batcher.alignedlayer.com \
--proof_generator_addr [proof_generator_addr]
```

**Examples**

```bash
aligned \
--proving_system GnarkPlonkBn254 \
--proof ./batcher/aligned/test_files/plonk_bn254/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bn254/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bn254/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

```bash
aligned \
--proving_system GnarkPlonkBls12_381 \
--proof ./batcher/aligned/test_files/plonk_bls12_381/plonk.proof \
--public_input ./batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub \
--vk ./batcher/aligned/test_files/plonk_bls12_381/plonk.vk \
--conn wss://batcher.alignedlayer.com
```

```bash
aligned \
--proving_system Groth16Bn254 \
--proof ./batcher/aligned/test_files/groth16/ineq_1_groth16.proof \
--public_input ./batcher/aligned/test_files/groth16/ineq_1_groth16.pub \
--vk ./batcher/aligned/test_files/groth16/ineq_1_groth16.vk \
--conn wss://batcher.alignedlayer.com
```

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
--proof_generator_addr [proof_generator_addr]
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
- [Phoenix 1.7.12](https://hexdocs.pm/phoenix/installation.html)
- [Ecto 3.11.2](https://hexdocs.pm/ecto/getting-started.html)

### Running for local devnet

```make run_devnet_explorer```

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
You can access to a tasks information by visiting `localhost:4000/batches/:merkle_root`.

### Run with custom env / other devnets

Create a `.env` file in the `/explorer` directory of the project. The `.env` file needs to contain the following variables:

| Variable      | Description                                                                                     |
| ------------- | ----------------------------------------------------------------------------------------------- |
| `RPC_URL`     | The RPC URL of the network you want to connect to.                                              |
| `ENVIRONMENT` | The environment you want to run the application in. It can be `devnet`, `holesky` or `mainnet`. |
| `PHX_HOST`    | The host URL where the Phoenix server will be running.                                          |

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

#### Dependencies
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
--proof_generator_addr [proof_generator_addr]
```

## FAQ

### What is the objective of Aligned?

Aligned’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain the zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t know what that future will look like, but we are certain it will be in Ethereum. The question we want to share is: If we are certain zero-knowledge proofs are the future of Ethereum but we are not certain which of the many possible zero-knowledge futures will win. How can we build an infrastructure for Ethereum to be compatible with any future zero-knowledge proving system?

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and help it increase its capabilities.

### What are the use cases of Aligned?

Among the possible use cases of Aligned we have:

Soft finality for Rollups and Appchains, fast bridging, new settlement layers (use Aligned + EigenDA) for Rollups and Intent based systems, P2P protocols based on SNARKs such as payment systems and social networks, alternative L1s interoperable with Ethereum, Verifiable Machine Learning, cheap verification and interoperability for Identity Protocols, ZK Oracles, new credential protocols such as zkTLS based systems, ZK Coprocessor, encrypted Mempools using SNARKs to show the correctness of the encryption, protocols against misinformation and fake news, and on-chain gaming.

### Why build on top of Ethereum?

Ethereum is the most decentralized and biggest source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

### Why not do this directly on top of Ethereum?

In order to do this we would have to aggregate all the proofs into a single proof. This is not a good solution considering that we would need some way to wrap proofs (for example, by means of recursion), which involves complex operations such as field emulation, bitwise, and/or elliptic curve operations.

### Why not make Aligned a ZK L1?

An L1 would not have the security properties of Ethereum consensus, and bootstrapping a new decentralized network is not only expensive but might be an impossible task. Zero-knowledge proofs are a nascent technology, and change is a constant. The best solution for today may not be the best for tomorrow; modifying L1s is extremely costly, especially as time progresses.

### Why not a ZK L2?

An L2 needs to use the EVM to settle in Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.

The EVM is not designed for ZK Verification, so most verifications are expensive.

To solve this, for pairing-based cryptography, Ethereum has added a precompile for verifications using the curve BN254.

But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast Starks need efficient hashing for fields. Which is the best field? Mersenne’s? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?

The amount of progress in the field is big, and nobody can predict the endgame.

Even more, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems.

Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or one hash not created yet.

Aligned solves all of this. No matter how or what you want to prove, it can be verified efficiently here while still inheriting the security of Ethereum as other L2s.

### Why EigenLayer?

We believe Ethereum is the best settlement layer, and zero-knowledge will play a key role in helping it be THE settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs, but how can we build such a decentralized network that will help Ethereum? Creating a new L1 doesn’t benefit Ethereum because using it will add new trust assumptions to the Ethereum protocols relying on it. So, if we must have:

1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in Ethereum
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### Will you aggregate proofs?

Proof aggregation can also be supported by proving the verification of many of these different verifications. This will likely not be an urgent feature, but it will be needed in the future with more demand.

### How does it compare to the Polygon aggregation layer?

Aligned is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.
