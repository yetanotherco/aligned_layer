# Ethereum Package

> [!WARNING]
> [Kurtosis](https://github.com/kurtosis-tech/kurtosis) must be installed.

## Usage and Setup

To start a local network run:

```bash
make ethereum_package_start
```

To see the status of the network run:

```bash
make ethereum_package_inspect
```

This will show you all the containers running and their respective ports. The most interesting ones are prometheus, grafana and el_forkmon, they will provide you with metrics to see the gasPrice, gasUsed, txPerBlock, and others that will be in our interest to create chaos on the network.

To stop the network run the following:

```bash
make ethereum_package_rm
```

To start the batcher run:

```bash
make batcher_start_ethereum_package
```

To start the aggregator run:

```bash
make aggregator_start_ethereum_package
```

To start an operator run:

```bash
make operator_register_start_ethereum_package
```

If you want to deploy more operators, you must duplicate the config-operator-1-ethereum-package.yaml and change the private and bls keys and the address.

To start Telemetry and the Explorer, run the usual commands:

```bash
make telemetry_full_start
make run_explorer
```

To spam transactions install spamoor:
```bash
make install_spamoor
```

and run the following make targets:

```bash
make spamoor_send_transactions  \\
  COUNT=<TOTAL_TX_TO_EXECUTE> \\
  TX_PER_BLOCK=<LIMIT_OF_TXS_TO_SEND_PER_BLOCK> \\
  TX_CONSUME_GAS=<HOW_MUCH_GAS_TO_USE_PER_TX> \\
  NUM_WALLETS=<NUMBER_OF_WALLETS_FROM_WHICH_TO_SEND_TXS> \\
  TIP_FEE=<TIP_FEE_IN_GWEI>
```

For Example:
```bash
make spamoor_send_transactions COUNT=1000 TX_CONSUME_GAS=150000 TX_PER_BLOCK=50 NUM_WALLETS=100 TIP_FEE=2
```

## Network status

You can check the network status using Grafana and el_forkmon explorer.
Run `make ethereum_package_inspect` to see the exposed ports for the enabled services. You can check for example the Grafana panels under `Dashboards -> Ethereum Metrics Exporter Overview` which provides useful information about the network.

## Changing Network Params

To adjust network params you have to modify `network_params.yaml`.

> [!NOTE]
> We are using a hardcoded input to deploy Eigen and Aligned contracts using the output from anvil.

## How to make transactions compete and see bumping in the Aggregator and Batcher logs

To increment the gas price and make transactions compete with aligned transactions we need to:

1. **Exceed the block `gasLimit` (30 million):** This is achieved by ensuring the total gas consumed per block is greater than `30,000,000`. Calculate it as: `TX_CONSUME_GAS * TX_PER_BLOCK` 
2. **Raise the `tipFee` slightly above the current gas price:** For instance, if the current `gasPrice` is `20 GWEI`, you can generate spam transactions with:

```bash
make spamoor_send_transactions COUNT=1000000000  TX_CONSUME_GAS=150000 TX_PER_BLOCK=210 NUM_WALLETS=1000 TIP_FEE=22
```

- Notes:
    - A transaction consuming `150000` of gas would be similar to a bridge swap.
    - We pass `2` gwei more to the `tipFee` that should be enough if not, you can increase it.

3. **Monitor Gas Price Updates:** After a few blocks, the `gasPrice` will adjust. The aligned batcher and aggregator will fetch the updated `gasPrice` and start competing in the mempool with their adjusted bump.
4. **Repeat as Needed:** Re-run the same command with the updated `TIP_FEE` to maintain competition:

```bash
make spamoor_send_transactions COUNT=1000000000 TX_CONSUME_GAS=150000 TX_PER_BLOCK=210 NUM_WALLETS=1000 TIP_FEE=<new_tip_fee>
```

## How to update preloaded contracts

1. First, deploy the new contracts in Anvil:

```bash
make anvil_deploy_aligned_contracts
```

That will generate the state output in `contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json`

2. Filter the contracts' state:

```bash
jq '.accounts | to_entries | map(select(.value.code != "0x")) | from_entries' contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json > contracts.json
```

3. Open the `network_params.yaml` file and replace the `additional_preloaded_contracts` section with the content of `contracts.json`.
