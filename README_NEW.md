# Aligned Layer

> [!CAUTION]
> To be used in testnet only.

Basic repo demoing a Stark/Snark verifier AVS middleware with full EigenLayer integration.

## The Project

Aligned Layer works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduces the proving overhead and adds verifier overhead, now become economically feasable to verify thanks to Aligned Layer.

Full documentation and examples will be added soon


## Dependencies

You will need [go](https://go.dev/doc/install), [foundry](https://book.getfoundry.sh/getting-started/installation), [zap-pretty](https://github.com/maoueh/zap-pretty), [abigen](https://geth.ethereum.org/docs/tools/abigen), [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git),
[celestia](https://docs.celestia.org/nodes/celestia-node#installing-from-source),
[jq](https://jqlang.github.io/jq/) and [yq](https://github.com/mikefarah/yq) to run the examples below.

To install zap-pretty and abigen
```bash
make deps
```

To install foundry
```bash
make install-foundry
```
Then follow the command line instructions
Make sure to run `foundryup`

To install eigenlayer-cli
```bash
make install-eigenlayer-cli
```


## Deploy Contracts

### EigenLayer

#### Anvil

When changing EigenLayer contracts, the anvil state needs to be updated with:

```bash
make anvil-deploy-eigen-contracts
```

You will also need to redeploy the MockStrategy & MockERC20 contracts:

```bash
make anvil-deploy-mock-strategy
```
#### Holesky

The current EigenLayer contracts for Holesky are available in the [eigenlayer-holesky-contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/testnet-holesky/script/configs/holesky/Holesky_current_deployment.config.json).

#### Mainnet

The current EigenLayer contracts for Mainnet are available in the [eigenlayer-mainnet-contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/script/configs/mainnet/Mainnet_current_deployment.config.json).


### AlignedLayer

#### Anvil

When changing AlignedLayer contracts, the anvil state needs to be updated with:

```bash
make anvil-deploy-aligned-contracts
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
make deploy-aligned-contracts
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
make anvil-start
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and AlignedLayer contracts already deployed (but no operator registered).


## Aggregator

### Run

To start the aggregator run:

```bash
make aggregator-start CONFIG_FILE=<path_to_config_file>
```

If you want to run the aggregator with the default configuration, you can run:

```bash
make aggregator-start
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
make operator-register-with-eigen-layer CONFIG_FILE=<path_to_config_file>
```

To register an operator in EigenLayer with the default configuration, you can run:

```bash
make operator-register-with-eigen-layer
```

### Deposit Strategy Tokens

#### Anvil

There is an ERC20 token deployed in the Anvil chain to use as strategy token with EigenLayer.

To deposit strategy tokens in the Anvil chain, you can use the following command:

```bash
make operator-mint-mock-tokens CONFIG_FILE=<path_to_config_file>
make operator-deposit-into-mock-strategy CONFIG_FILE=<path_to_config_file>
```

To deposit strategy tokens in the Anvil chain with the default configuration, you can run:

```bash
make operator-mint-mock-tokens
make operator-deposit-into-mock-strategy
```

#### Holesky/Mainnet

EigenLayer strategies are available in [eigenlayer-strategies](https://holesky.eigenlayer.xyz/restake).

For Holesky, we are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To obtain HolETH and swap it for different strategies, you can use the following [guide](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/stage-2-testnet/obtaining-testnet-eth-and-liquid-staking-tokens-lsts).

### Register into AlignedLayer

To register an operator in AlignedLayer run the following command:

```bash
make operator-register-with-aligned-layer CONFIG_FILE=<path_to_config_file>
```

To register an operator in AlignedLayer with the default configuration, you can run:

```bash
make operator-register-with-aligned-layer
```

### Full Registration in Anvil

For devnet purposes, you can run the following command to register an operator in EigenLayer and AlignedLayer and deposit strategy tokens in EigenLayer:

```bash
make operator-full-registration CONFIG_FILE=<path_to_config_file>
```

To register an operator in EigenLayer and AlignedLayer and deposit strategy tokens in EigenLayer with the default configuration, you can run:

```bash
make operator-full-registration
```

### Run

To start the operator run:

```bash
make operator-start CONFIG_FILE=<path_to_config_file>
```

If you want to run the operator with the default configuration, you can run:

```bash
make operator-start
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


## Data Availability (DA)

### Calldata

This stores the proof data into Ethereum.

Limitations -> Expensive # TODO

### EigenDA

Limitations -> Slow, 2MB # TODO

### Celestia

Limitations -> 2MB # TODO 

#### Setup

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

## Task Sender



### Run

## Keystores


## Deployment



