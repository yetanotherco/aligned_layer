# Aligned Layer

> [!CAUTION]
> To be used in testnet only.

Basic repo demoing a Stark/Snark verifier AVS middleware with full EigenLayer integration.


## The Project

Aligned Layer works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduces the proving overhead and adds verifier overhead, now become economically feasable to verify thanks to Aligned Layer.

Full documentation and examples will be added soon


## Setup

### Dependencies

You will need [go](https://go.dev/doc/install), [foundry](https://book.getfoundry.sh/getting-started/installation), [zap-pretty](https://github.com/maoueh/zap-pretty), [abigen](https://geth.ethereum.org/docs/tools/abigen), [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git),
[celestia](https://docs.celestia.org/nodes/celestia-node#installing-from-source),
[jq](https://jqlang.github.io/jq/) and [yq](https://github.com/mikefarah/yq) to run the examples below.

To install zap-pretty and abigen
```bash
make deps
```

To install foundry
```bash
make install_foundry
```
Then follow the command line instructions
Make sure to run `foundryup`

To install eigenlayer-cli
```bash
make install_eigenlayer_cli
```

Make sure to build SP1 with:

```bash
make build_sp1_macos # or make build_sp1_linux on linux
```

### Keystores

To create a keystore, you can run the following commands:
    
```bash
cast wallet new-mnemonic
cast wallet import <keystore-name> --private-key <private-key>
```

To create a ECDSA keystore, you can run the following commands:

```bash
eigenlayer operator keys import --key-type ecdsa <keystore-name> <private-key>
```

To create a BLS keystore, you can run the following commands:

```bash
eigenlayer operator keys import --key-type bls <keystore-name> <private-key>
```

### Data Availability

#### EigenDA

You need the EigenDA Disperser to interact with EigenDA. You can find the EigenDA Disperser 
- [Holesky](https://docs.eigenlayer.xyz/eigenda/networks/holesky)
- [Mainnet](https://docs.eigenlayer.xyz/eigenda/networks/mainnet)

#### Celestia

To set up Celestia, you will need to install the Celestia-Node CLI.
Refer to [this resource](https://docs.celestia.org/nodes/celestia-node#installing-from-source)
for instructions on how to do so.

Then, to initialize the node store for the Arabica network run:
```bash
celestia light init --p2p.network arabica
```
The output in your terminal will show the location of your node store and config.

To start the node in the Arabica network run:
```bash

celestia light start --core.ip validator-1.celestia-arabica-11.com --p2p.network arabica
```


## Deploy Contracts

### EigenLayer

#### Anvil

When changing EigenLayer contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_eigen_contracts
```

You will also need to redeploy the MockStrategy & MockERC20 contracts:

```bash
make anvil_deploy_mock_strategy
```
#### Holesky

The current EigenLayer contracts for Holesky are available in the [eigenlayer-holesky-contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/testnet-holesky/script/configs/holesky/Holesky_current_deployment.config.json).

#### Mainnet

The current EigenLayer contracts for Mainnet are available in the [eigenlayer-mainnet-contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/script/configs/mainnet/Mainnet_current_deployment.config.json).

### AlignedLayer

#### Anvil

When changing AlignedLayer contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_aligned_contracts
```

#### Holesky/Mainnet

To deploy the contracts to Testnet/Mainnet, you will need to set environment variables in a `.env` file in the same directory as the deployment script (`contracts/scripts/`).

The necessary environment variables are:

| Variable Name                   | Description                                                           |
|---------------------------------|-----------------------------------------------------------------------|
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


## Run Devnet Locally with Anvil

Start anvil with every relevant contract deployed with:

```bash
make anvil_start
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and AlignedLayer contracts already deployed (but no operator registered).


## Aggregator

### Run

To start the aggregator run:

```bash
make aggregator_start CONFIG_FILE=<path_to_config_file>
```

If you want to run the aggregator with the default configuration, you can run:

```bash
make aggregator_start
```

### Config

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

## BLS Configurations
bls:
  private_key_store_path: <path_to_bls_private_key_store>
  private_key_store_password: <bls_private_key_store_password>

## Aggregator Configurations
aggregator:
  server_ip_port_address: <ip:port>
  bls_public_key_compendium_address: 
  avs_service_manager_address: 
  enable_metrics: <true/false>
```


## Operator

### Register into EigenLayer

To register an operator in EigenLayer run the following command:

```bash
make operator_register_with_eigen_layer CONFIG_FILE=<path_to_config_file>
```

To register an operator in EigenLayer with the default configuration, you can run:

```bash
make operator_register_with_eigen_layer
```

### Deposit Strategy Tokens

#### Anvil

There is an ERC20 token deployed in the Anvil chain to use as strategy token with EigenLayer.

To deposit strategy tokens in the Anvil chain, you can use the following command:

```bash
make operator_mint_mock_tokens CONFIG_FILE=<path_to_config_file>
make operator_deposit_into_mock_strategy CONFIG_FILE=<path_to_config_file>
```

To deposit strategy tokens in the Anvil chain with the default configuration, you can run:

```bash
make operator_mint_mock_tokens
make operator_deposit_into_mock_strategy
```

#### Holesky/Mainnet

EigenLayer strategies are available in [eigenlayer-strategies](https://holesky.eigenlayer.xyz/restake).

For Holesky, we are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To obtain HolETH and swap it for different strategies, you can use the following [guide](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/stage-2-testnet/obtaining-testnet-eth-and-liquid-staking-tokens-lsts).

### Register into AlignedLayer

To register an operator in AlignedLayer run the following command:

```bash
make operator_register_with_aligned_layer CONFIG_FILE=<path_to_config_file>
```

To register an operator in AlignedLayer with the default configuration, you can run:

```bash
make operator_register_with_aligned_layer
```

### Full Registration in Anvil

For devnet purposes, you can run the following command to register an operator in EigenLayer and AlignedLayer and deposit strategy tokens in EigenLayer:

```bash
make operator_full_registration CONFIG_FILE=<path_to_config_file>
```

To register an operator in EigenLayer and AlignedLayer and deposit strategy tokens in EigenLayer with the default configuration, you can run:

```bash
make operator_full_registration
```

### Run

To start the operator run:

```bash
make operator_start CONFIG_FILE=<path_to_config_file>
```

If you want to run the operator with the default configuration, you can run:

```bash
make operator_start
```

### Config

There is a default configuration for devnet purposes in `config-files/config.yaml`.
Also, there are 3 different configurations for the operator in `config-files/devnet/operator-1.yaml`, `config-files/devnet/operator-2.yaml` and `config-files/devnet/operator-3.yaml`.

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

## BLS Configurations
bls:
  private_key_store_path: <path_to_bls_private_key_store>
  private_key_store_password: <bls_private_key_store_password>

## EigenDA Configurations
eigen_da_disperser:
  url: <eigen_da_disperser_url> # This is the url of the EigenDA Disperser

## Celestia Configurations
celestia:
  url: <celestia_url> # This is the url of the deployed Celestia Light node
  keystore: <celestia_keystore> # This is the keystore of the Celestia Light node

## Operator Configurations
operator:
  aggregator_rpc_server_ip_port_address: <ip:port> # This is the aggregator url
  address: <operator_address>
  earnings_receiver_address: <earnings_receiver_address> # This is the address where the operator will receive the earnings, it can be the same as the operator address
  delegation_approver_address: "0x0000000000000000000000000000000000000000" # TODO This is 0x0 for now, check what to put here
  staker_opt_out_window_blocks: 0 # TODO This is 0 for now, check what to put here
  metadata_url: "https://yetanotherco.github.io/operator_metadata/metadata.json"
# Operators variables needed for register it in EigenLayer
el_delegation_manager_address: <el_delegation_manager_address> # This is the address of the EigenLayer delegationManager
private_key_store_path: <path_to_bls_private_key_store>
bls_private_key_store_path: <bls_private_key_store_password>
signer_type: local_keystore
chain_id: <chain_id>
```


## Task Sender

### Run

To send a single task run:

```bash
go run task_sender/cmd/main.go send-task
    --proving-system <prooving-system> \
    --proof <proof> \
    --public-input <public-input> \
    --verification-key <verification-key> \
    --config <config-file> \
    --da <da-solution>
```

To send tasks in loop run:

```bash
go run task_sender/cmd/main.go loop-tasks
    --proving-system <prooving-system> \
    --proof <proof> \
    --public-input <public-input> \
    --verification-key <verification-key> \
    --config <config-file> \
    --da <da-solution>
    --interval <interval-in-seconds>
```

### Config

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

### Send PLONK BLS12_381 Proof

To send a single PLONK BLS12_381 proof run:

```bash
make send_plonk_bls12_381_proof
```

To send PLONK BLS12_381 proofs in loop run:

```bash
make send_plonk_bls12_381_proof_loop
```

### Send PLONK BN254 Proof

To send a single PLONK BN254 proof run:

```bash
make send_plonk_bn254_proof
```

To send PLONK BN254 proofs in loop run:

```bash
make send_plonk_bn254_proof_loop
```


## Deployment

To build go binaries run:

```bash
make build_binaries
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
