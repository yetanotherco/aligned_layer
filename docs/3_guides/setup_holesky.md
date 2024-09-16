# Deploying Aligned Contracts to Holesky

## Eigenlayer Contracts: Holesky/Mainnet

These contracts are not deployed by Aligned. Current EigenLayer contracts:

- [Holesky Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/testnet-holesky/script/configs/holesky/Holesky_current_deployment.config.json)
- [Mainnet Contracts](https://github.com/Layr-Labs/eigenlayer-contracts/blob/mainnet/script/configs/mainnet/Mainnet_current_deployment.config.json)

### Aligned Contracts: Holesky/Mainnet

To deploy the contracts to Testnet/Mainnet, you will need to set environment variables in a `.env` file in the same
directory as the deployment script (`contracts/scripts/`).

The necessary environment variables are:

| Variable Name                   | Description                                                           |
|---------------------------------|-----------------------------------------------------------------------|
| `RPC_URL`                       | The RPC URL of the network you want to deploy to.                     |
| `PRIVATE_KEY`                   | The private key of the account you want to deploy the contracts with. |
| `EXISTING_DEPLOYMENT_INFO_PATH` | The path to the file containing the deployment info about EigenLayer. |
| `DEPLOY_CONFIG_PATH`            | The path to the deployment config file.                               |
| `OUTPUT_PATH`                   | The path to the file where the deployment info will be saved.         |

You can find an example `.env` file in [.env.example.holesky](../../contracts/scripts/.env.example.holesky)

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

To upgrade the Service Manager Contract in Testnet/Mainnet, run:

```bash
make upgrade_aligned_contracts
```

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
