# Aligned Infrastructure Deployment Guide

## Dependencies

Ensure you have the following installed:

- [Go](https://go.dev/doc/install)
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [jq](https://jqlang.github.io/jq/)
- [yq](https://github.com/mikefarah/yq)

After installing foundryup, you need to install a specific Foundry version:

```shell
foundryup -i nightly-a428ba6ad8856611339a6319290aade3347d25d9
```

Then run:

```shell
make deps
```

This will:

- Initialize git submodules
- Install: `eigenlayer-cli`, `zap-pretty` and `abigen`
- Build ffis for your os.

## Contracts and eth node

To start anvil, a local Ethereum devnet with all necessary contracts already deployed and ready to be interacted with, run:

```shell
make anvil_start_with_block_time
```

<details>
<summary>More information on deploying the smart contracts on anvil:</summary>

### EigenLayer Contracts

If EigenLayer contracts change, the anvil state needs to be updated with:

```bash
make anvil_deploy_eigen_contracts
```

You will also need to redeploy the MockStrategy & MockERC20 contracts:

```bash
make anvil_deploy_mock_strategy
```

### Aligned Contracts

When changing Aligned contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_aligned_contracts
```

Note that when changing the contracts, you must also re-generate the Go smart contract bindings:

```bash
make bindings
```

</details>

## Aggregator

To start the [Aggregator](../2_architecture/components/5_aggregator.md):

```bash
make aggregator_start ENVIRONMENT=devnet
```

or with a custom config:

```bash
make aggregator_start ENVIRONMENT=devnet CONFIG_FILE=<path_to_config_file>
```

## Operator

To setup an [Operator](../2_architecture/components/4_operator.md) run:

```bash
make operator_register_and_start ENVIRONMENT=devnet
```

or with a custom config:

```bash
make operator_register_and_start ENVIRONMENT=devnet CONFIG_FILE=<path_to_config_file>
```

Different configs for operators can be found in `config-files/config-operator`.

<details>
<summary>More information about Operator registration:</summary>

If you wish to only register an operator you can run:

```bash
make operator_full_registration CONFIG_FILE<path_to_config_file>
```

and to start it once it has been registered:

```bash
make operator_start ENVIRONMENT=devnet CONFIG_FILE=<path_to_config_file>
```

</details>

## Batcher

To start the [Batcher](../2_architecture/components/1_batcher.md) locally:

```bash
make batcher_start_local
```

This starts a [localstack](https://www.localstack.cloud/) to act as a replacement for S3.

If you want to use the batcher under a real `S3` connection you'll need to specify the environment variables under `batcher/aligned-batcher/.env` and then run:

```bash
make batcher_start
```

---

# Other components

Aligned also counts with 2 external components, which are not necessary for Aligned to work, but are useful for observability.

## Explorer

### Dependencies

Ensure you have the following installed:

- [Erlang 26](https://github.com/asdf-vm/asdf-erlang)
- [Elixir 1.16.2](https://elixir-ko.github.io/install.html), compiled with OTP 26
- [Docker](https://docs.docker.com/get-docker/)
- [NodeJS](https://nodejs.org/en/download/package-manager)
  - Tested with node 20 and 22
- [pnpm](https://pnpm.io/installation)

After installing the necessary deps, setup the environment variables by running:

```shell
make explorer_create_env
```

Then start the explorer:

```shell
make explorer_build_db
make run_explorer
```

This will:

- Start a postgres docker container
- Run ecto setup
- Start the explorer on http://localhost:4000.

If you want to run the explorer without docker run:

```shell
make run_explorer_without_docker_db
```

<details>
<summary>Clean, dump and recover DB</summary>

To clear the DB, you can run:

```bash
make explorer_clean_db
```

If you need to dump the data from the DB, you can run:

```bash
make explorer_dump_db
```

This will create a `dump.$date.sql` SQL script on the `explorer` directory with all the existing data.

Data can be recovered from a `dump.$date.sql` using the following command:

```bash
make explorer_recover_db
```

Then you'll be requested to enter the file name of the dump you want to recover already positioned in the `/explorer` directory.

This will update your database with the dumped database data.

</details>

### Fetching batches and operators data

If you want to fetch past batches that for any reason were not inserted into the DB, you will first need to make sure you have the `ELIXIR_HOSTNAME` in the `.env` file.

You can get the hostname of your elixir by running:

```bash
elixir -e 'IO.puts(:inet.gethostname() |> elem(1))'
```

Then you can run:

```bash
make explorer_fetch_old_batches FROM_BLOCK=<FROM_BLOCK> TO_BLOCK=<TO_BLOCK>
```

To get operators strategies and restakes data:

```bash
make explorer_fetch_old_operators_strategies_restakes FROM_BLOCK=<FROM_BLOCK>
```

## Metrics/Telemetry

The Metrics and Telemetry are used to view more in-depth information about the network. With it, you can visualize all sort of cumulative and historical metrics of the network, of the individual components and their behaviors. Tese services are not necessary to run aligned, though you will see warnings in the rest of components as they won't be able to connect and send their status.

### Dependencies

Ensure you have the following installed:

- [Go](https://go.dev/doc/install)
- [Erlang 26](https://github.com/asdf-vm/asdf-erlang)
- [Elixir 1.16.2](https://elixir-ko.github.io/install.html), compiled with OTP 26
- [Docker](https://docs.docker.com/get-docker/)

### Metrics service

To run Prometheus and Grafana, run:

```bash
make run_metrics
```

This will start containers for Prometheus and Grafana. You can access Grafana on `http://localhost:3000` with the default credentials `admin:admin`.

Alternately, you can access the raw scrapped metrics collected with Prometheus on `http://localhost:9091/metrics`.

### Telemetry service

To setup the telemetry service run:

If it is your first time first you'll need to execute the following commands:

```bash
make telemetry_create_env
make telemetry_build_db
```

Then, to start the service:

```bash
make telemetry_full_start
```

This will:

- Start OpenJaeger container for the traces: available at `http://localhost:16686/`
- Start telemetry server: available at `http://localhost:4001/`

---

## Send test proofs

To send proofs quickly you can run any of the targets that have the prefix `batcher_send` for example:

Send a single plonk proof:

```shell
make batcher_send_plonk_bn254_task
```

Send a burst of `<N>` risc0 proofs:

```shell
make batcher_send_risc0_burst BURST_SIZE=<N>
```

Send an infinite stream of groth_16 proofs:

```shell
make batcher_send_burst_groth16 BURST_SIZE=2
```

Feel free to explore the rest of targets.

---
