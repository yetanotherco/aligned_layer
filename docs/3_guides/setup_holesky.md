# Deploying Aligned Contracts to Holesky

## Eigenlayer Contracts: Holesky/Mainnet

These contracts are not deployed by Aligned. Current EigenLayer contracts:

- [Holesky Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/testnet-holesky/script/configs/holesky/Holesky_current_deployment.config.json)
- [Mainnet Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/script/configs/mainnet/Mainnet_current_deployment.config.json)

## Aligned Contracts: Holesky/Mainnet

### Deploy Service Manager

To deploy the AlignedLayer contracts to Testnet/Mainnet, you will need to set environment variables in a `.env` file in the same
directory as the deployment script (`contracts/scripts/`).

The necessary environment variables are:

| Variable Name                         | Description                                                             |
|---------------------------------------|-------------------------------------------------------------------------|
| `RPC_URL`                             | The RPC URL of the network you want to deploy to.                       |
| `PRIVATE_KEY`                         | The private key of the account you want to deploy the contracts with.   |
| `EXISTING_DEPLOYMENT_INFO_PATH`       | The path to the file containing the deployment info about EigenLayer.   |
| `DEPLOY_CONFIG_PATH`                  | The path to the deployment config file for the Service Manager.         |
| `OUTPUT_PATH`                         | The path to the file where the deployment info will be saved.           |
| `ETHERSCAN_API_KEY`                   | API KEY to verify the contracts in Etherscan.                           |
| `MULTISIG`                            | This is required for upgrade to specify is you are using a multisig.    |

You can find an example `.env` file in [.env.example.holesky](../../contracts/scripts/.env.example.holesky)

Note: all file paths must be inside the `script/` folder, as shown in `.env.example.holesky` because of `foundry`'s permissions to read and write files.

You need to complete the `DEPLOY_CONFIG_PATH` file with the following information:

```json
{
  "chainInfo": {
    "chainId": "<chain_id>"
  },
  "permissions": {
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

Then run the following command:

```bash
make deploy_aligned_contracts
```

### Deploy Batcher Payment Service

### Deploy Service Manager

To deploy the Batcher Payment Service contract to Testnet/Mainnet, you will need to set environment variables in a `.env` file in the same
directory as the deployment script (`contracts/scripts/`).

The necessary environment variables are:

| Variable Name                         | Description                                                             |
|---------------------------------------|-------------------------------------------------------------------------|
| `RPC_URL`                             | The RPC URL of the network you want to deploy to.                       |
| `PRIVATE_KEY`                         | The private key of the account you want to deploy the contracts with.   |
| `EXISTING_DEPLOYMENT_INFO_PATH`       | The path to the file containing the deployment info about EigenLayer.   |
| `DEPLOY_CONFIG_PATH`                  | The path to the deployment config file for the Service Manager.         |
| `BATCHER_PAYMENT_SERVICE_CONFIG_PATH` | The path to the deployment config file for the Batcher Payment Service. |
| `OUTPUT_PATH`                         | The path to the file where the deployment info will be saved.           |
| `ETHERSCAN_API_KEY`                   | API KEY to verify the contracts in Etherscan.                           |
| `MULTISIG`                            | This is required for upgrade to specify is you are using a multisig.    |

You can find an example `.env` file in [.env.example.holesky](../../contracts/scripts/.env.example.holesky)

You need to complete the `BATCHER_PAYMENT_SERVICE_CONFIG_PATH` file with the following information:

```json
{
  "address": {
    "batcherWallet": "<batcher_wallet_address>",
    "alignedLayerServiceManager": "<aligned_layer_service_manager_address>"
  },
  "permissions": {
    "owner": "<owner_address>"
  },
  "eip712": {
    "noncedVerificationDataTypeHash": "0x41817b5c5b0c3dcda70ccb43ba175fdcd7e586f9e0484422a2c6bba678fdf4a3"
  }
}
```

Then run the following command:

```bash
make deploy_batcher_payment_service
```

### Upgrade Service Manager

To upgrade the Service Manager Contract in Testnet/Mainnet, run:

```bash
make upgrade_aligned_contracts
```

### Upgrade Registry Coordinator

To upgrade the Registry Coordinator in Testnet/Mainnet, run:

```bash
make upgrade_registry_coordinator
```

Make sure to set environment variables in a `.env` file in the same directory as the upgrade
script (`contracts/scripts/`).

### Go smart contract Bindings

Also, you must re-generate the Go smart contract bindings:

```bash
make bindings
```

### Deployment

And finally you must rebuild go binaries, for Operator and Aggregator:

```bash
make build_binaries
```
