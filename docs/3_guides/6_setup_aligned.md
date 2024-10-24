# Aligned Infrastructure Deployment Guide

## Dependencies

Ensure you have the following installed:

- [Go](https://go.dev/doc/install)
- [Rust](https://www.rust-lang.org/tools/install)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [zap-pretty](https://github.com/maoueh/zap-pretty)
- [abigen](https://geth.ethereum.org/docs/tools/abigen)
- [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git)
- [jq](https://jqlang.github.io/jq/)
- [yq](https://github.com/mikefarah/yq)

To
install [Go](https://go.dev/doc/install),
[Rust](https://www.rust-lang.org/tools/install), [jq](https://jqlang.github.io/jq/)
and [yq](https://github.com/mikefarah/yq) go to the provided links and follow the instructions.

Install Go
dependencies ([zap-pretty](https://github.com/maoueh/zap-pretty), [abigen](https://geth.ethereum.org/docs/tools/abigen), [eigenlayer-cli](https://github.com/Layr-Labs/eigenlayer-cli.git)):

```bash
make go_deps
```

Install [Foundry](https://book.getfoundry.sh/getting-started/installation):

```bash
make install_foundry
foundryup
```

Install the necessary submodules and build all the FFIs for your OS:

```bash
make deps
```

If you want to rebuild the FFIs, you can use:

```bash
make build_all_ffi
```

### Booting Devnet with Default configs

Before starting, you need to set up an S3 bucket. More data storage will be tested in the future.

You need to fill the data in:

`batcher/aligned-batcher/.env`

And you can use this file as an example of how to fill it:

`batcher/aligned-batcher/.env.example`

After having the env setup, run in different terminals the following commands to boot Aligned locally:

## Anvil

To start anvil, a local Ethereum devnet with all necessary contracts already deployed and ready to be interacted with,
run:

```bash
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

To test the upgrade script for ServiceManager in the local devnet, run:

```bash
make anvil_upgrade_aligned_contracts
```

To test the upgrade script for RegistryCoordintator in the local devnet, run:

```bash
make anvil_upgrade_registry_coordinator
```

Note that when upgrading the contracts, you must also:

1. Re-generate the Go smart contract bindings:

   ```bash
   make bindings
   ```

2. Rebuild Aggregator and Operator Go binaries:

   ```bash
   make build_binaries
   ```

</details>

---

## Aggregator

To start the [Aggregator](../2_architecture/components/5_aggregator.md):

```bash
make aggregator_start
```

<details>
<summary>To start the aggregator with a custom configuration:</summary>

```bash
make aggregator_start CONFIG_FILE=<path_to_config_file>
```

</details>

---

## Operator

To start an [Operator](../2_architecture/components/4_operator.md)
(note it also registers it):

```bash
make operator_register_and_start
```

If you need to start again the operator, and it's already registered, you can use:

```bash
make operator_start
```

<details>
<summary>More information about Operator registration:</summary>

Operator needs to register in both EigenLayer and Aligned. Then it can start verifying proofs.

### Register into EigenLayer

To register an operator in EigenLayer Devnet with the default configuration, run:

```bash
make operator_register_with_eigen_layer
```

To register an operator in EigenLayer with a custom configuration, run:

```bash
make operator_register_with_eigen_layer CONFIG_FILE=<path_to_config_file>
```

### Register into Aligned

To register an operator in Aligned with the default configuration, run:

```bash
make operator_register_with_aligned_layer
```

To register an operator in Aligned with a custom configuration, run:

```bash
make operator_register_with_aligned_layer CONFIG_FILE=<path_to_config_file>
```

### Full Registration in Anvil with one command

To register an operator in EigenLayer and Aligned and deposit strategy tokens in EigenLayer with the default
configuration, run:

```bash
make operator_full_registration
```

To register an operator in EigenLayer and Aligned and deposit strategy tokens in EigenLayer with a custom configuration,
run:

```bash
make operator_full_registration CONFIG_FILE=<path_to_config_file>
```

### Deposit Strategy Tokens in Anvil local devnet

There is an ERC20 token deployed in the Anvil chain to use as a strategy token with EigenLayer.

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

### Deposit Strategy tokens in Holesky/Mainnet

EigenLayer strategies are available in [eigenlayer-strategies](https://holesky.eigenlayer.xyz/restake).

For Holesky, we are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To get HolETH and swap it for different strategies, you can use the
following [guide](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/testnet/obtaining-testnet-eth-and-liquid-staking-tokens-lsts).

### Config

There is a default configuration for devnet purposes in `config-files/config.yaml`.
Also, there are three different configurations for the operator
in `config-files/devnet/operator-1.yaml`, `config-files/devnet/operator-2.yaml`
and `config-files/devnet/operator-3.yaml`.

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
  enable_metrics: <true|false>
  metrics_ip_port_address: <ip:port>
  max_batch_size: <max_batch_size_in_bytes>
# Operators variables needed for register it in EigenLayer
el_delegation_manager_address: <el_delegation_manager_address> # This is the address of the EigenLayer delegationManager
private_key_store_path: <path_to_bls_private_key_store>
bls_private_key_store_path: <bls_private_key_store_password>
signer_type: local_keystore
chain_id: <chain_id>
```

Changing operator keys:

Operator keys can be changed if needed.

{% hint style="warning" %}
When creating a new wallet keystore and private key please use strong passwords for your own protection.
{% endhint %}

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

</details>

---

## Batcher

To start the [Batcher](../2_architecture/components/1_batcher.md):

```bash
make batcher_start
```

If you are testing locally, you can run this instead:
```bash
make batcher_start_local
```

<details>
<summary>More information about Batcher configuration:</summary>

To run the batcher, you will need to set environment variables in a `.env` file in the same directory as the
batcher (`batcher/aligned-batcher/`).

The necessary environment variables are:

| Variable Name         | Description                                                                                                                    |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| AWS_SECRET_ACCESS_KEY | Secret key to authenticate and authorize API requests to the AWS S3 Bucket.                                                    |
| AWS_REGION            | Geographical region where the AWS S3 Bucket will be accessed.                                                                  |
| AWS_ACCESS_KEY_ID     | Access key used in combination with the AWS_SECRET_ACCESS_KEY to authenticate and authorize API requests to the AWS S3 Bucket. |
| AWS_BUCKET_NAME       | Name of the AWS S3 Bucket.                                                                                                     |
| RUST_LOG              | Rust log level (info, debug, error, warn, etc.).                                                                               |

You can find an example `.env` file in [.env.example](../../batcher/aligned-batcher/.env.example)

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
  max_proof_size: <max_proof_size_in_bytes>
  max_batch_size: <max_batch_size_in_bytes>
  eth_ws_reconnects: <eth_ws_reconnects_amount>
  pre_verification_is_enabled: <true|false>

## ECDSA Configurations
ecdsa:
  private_key_store_path: <path_to_ecdsa_private_key_store>
  private_key_store_password: <ecdsa_private_key_store_password>
```

### Run

```bash
make batcher_start
```

or

```bash
make batcher_start_local
```

The latter version sets up a [localstack](https://www.localstack.cloud/) to act as a replacement for S3,
so you don't need to interact with (and give money to) AWS for your tests.

</details>

---

## Send test proofs

Next, you can use some of the send proofs make targets.
All these proofs are pre-generated and for testing purposes,
feel free to generate your own tests to submit to Aligned.

<details>
<summary>SP1</summary>

Send an individual proof:

```bash
make batcher_send_sp1_task
```

Send a burst of 15 proofs:

```bash
make batcher_send_sp1_burst
```

Send proofs indefinitely:

```bash
make batcher_send_infinite_sp1
```

</details>

<details>
<summary>Risc0</summary>

Send an individual proof:

```bash
make batcher_send_risc0_task
```

Send a burst of 15 proofs:

```bash
make batcher_send_risc0_burst
```

</details>

<details>
<summary>Plonk</summary>

Send an individual BN254 proof:

```bash
make batcher_send_plonk_bn254_task
```

Send a burst of 15 BN254 proofs:

```bash
make batcher_send_plonk_bn254_burst
```

Send an individual BLS12-381 proof:

```bash
make batcher_send_plonk_bls12_381_task
```

Send a burst of 15 BLS12-381 proofs:

```bash
make batcher_send_plonk_bls12_381_burst
```

</details>

<details>
<summary>Groth16</summary>

Send an individual BN254 proof:

```bash
make batcher_send_groth16_bn254_task
```

Send BN254 proofs indefinitely:

```bash
make batcher_send_infinite_groth16
```

Send BN254 proof bursts indefinitely:

```bash
make batcher_send_burst_groth16
```

</details>

<details>
<summary>Send a specific proof:</summary>

To install the Aligned client to send a specific proof, run:

```bash
make install_aligned_compiling
```

The SP1 and Risc0 proofs need the proof file and the vm program file.
The current SP1 version used in Aligned is
`v3.0.0` and the current Risc0 version used in Aligned is `v1.1.2`.
The GnarkPlonkBn254, GnarkPlonkBls12_381 and Groth16Bn254 proofs need the proof file, the public input file and the
verification key file.

```bash
aligned submit \
--proving_system <SP1|GnarkPlonkBn254|GnarkPlonkBls12_381|Groth16Bn254|Risc0> \
--proof <proof_file> \
--vm_program <vm_program_file> \
--pub_input <pub_input_file> \
--proof_generator_addr [proof_generator_addr] \
--batch_inclusion_data_directory_path [batch_inclusion_data_directory_path] \
--keystore_path [path_to_ecdsa_keystore] \
--batcher_url wss://batcher.alignedlayer.com \
--rpc_url https://ethereum-holesky-rpc.publicnode.com 
```

</details>

## Explorer

If you also want to start the explorer for the devnet, to clearly visualize your submitted and verified batches, see how
to run it using the following documentation:

### Minimum Requirements

- [Erlang 26](https://github.com/asdf-vm/asdf-erlang)
- [Elixir 1.16.2](https://elixir-ko.github.io/install.html), compiled with OTP 26
- [Docker](https://docs.docker.com/get-docker/)
- [NodeJS](https://nodejs.org/en/download/package-manager)
  - Tested with node 20 and 22
- [pnpm](https://pnpm.io/installation)

### DB Setup

To set up the explorer, an installation of the DB is necessary.

First, you'll need to install docker if you don't have it already.
You can follow the instructions [here](https://docs.docker.com/get-docker/).

The explorer uses a PostgreSQL database. To build and start the DB using docker, run:

```bash
make explorer_build_db
```

<details>

<summary>
  (Optional) The steps to manually execute the database are as follows...
</summary>

- Run the database container, opening port `5432`:

```bash
make explorer_run_db
```

- Configure the database with ecto running `ecto.create` and `ecto.migrate`:

```bash
make explorer_ecto_setup_db
```

- Start the explorer:

```bash
make run_explorer
```

</details>

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

Then you'll be requested to enter the file name of the dump you want to recover already positioned in the `/explorer`
directory.

This will update your database with the dumped database data.

<details>
<summary>Extra Explorer script to fetch past batches</summary>

If you want to fetch past batches that for any reason were not inserted into the DB, you will first need to make sure
you have the ELIXIR_HOSTNAME .env variable configured.
You can get the hostname of your elixir by running :

```bash
elixir -e 'IO.puts(:inet.gethostname() |> elem(1))'
```

Then you can run:

```bash
make explorer_fetch_old_batches
```

You can modify which blocks are being fetched by modify the parameters the `explorer_fetch_old_batches.sh` is being
received

</details>

### Running the Explorer

To run the explorer for the local devnet, you'll need to have the devnet running and the DB already setup.

Additionally, you'll need to have the `.env` file in the `/explorer` directory of the project.
A base example of the `.env` file can be found in `/explorer/.env.dev`.

Use the following command to start the Explorer:

```bash
make run_explorer
```

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.
You can access to a tasks' information by visiting `localhost:4000/batches/:merkle_root`.

<details>
<summary>There's an additional Explorer script to fetch past operators and restake</summary>

If you want to fetch past operators, strategies and restake, you will need to run:

```bash
make explorer_fetch_old_operators_strategies_restakes
```

This will run the script `explorer_fetch_old_operators_strategies_restakes.sh` that will fetch the operators, strategies
and restake which will later insert into the DB.

</details>

### Run with custom env / other devnets

Create a `.env` file in the `/explorer` directory of the project.
The `.env` file needs to contain the following variables:

| Variable              | Description                                                                                     |
| --------------------- | ----------------------------------------------------------------------------------------------- |
| `RPC_URL`             | The RPC URL of the network you want to connect to.                                              |
| `ENVIRONMENT`         | The environment you want to run the application in. It can be `devnet`, `holesky` or `mainnet`. |
| `ALIGNED_CONFIG_FILE` | The config file containing Aligned contracts' deployment information                            |
| `PHX_HOST`            | The host URL where the Phoenix server will be running.                                          |
| `DB_NAME`             | The name of the postgres database.                                                              |
| `DB_USER`             | The username of the postgres database.                                                          |
| `DB_PASS`             | The password of the postgres database.                                                          |
| `DB_HOST`             | The host URL where the postgres database will be running.                                       |
| `ELIXIR_HOSTNAME`     | The hostname of your running elixir.                                                            |
| `DEBUG_ERRORS`        | If you want to enable phoenix errors on your browser instead of a 500 page, set this to `true`. |
| `TRACKER_API_URL`     | The URL of the aligned version each operator is running.                                        |

Then you can run the explorer with this env file config by entering the following command:

```bash
make run_explorer
```

This will start the explorer with the configuration set in the `.env` file on port 4000.
Visit [`localhost:4000`](http://localhost:4000) from your browser.

## Metrics

### Aggregator Metrics

Aggregator metrics are exposed on the `/metrics` endpoint.

If you are using the default config, you can access the metrics on `http://localhost:9091/metrics`.

To run Prometheus and Grafana, run:

```bash
make run_metrics
```

Then you can access Grafana on `http://localhost:3000` with the default credentials `admin:admin`.

If you want to install Prometheus and Grafana manually, you can follow the instructions below.

To install Prometheus, you can follow the instructions on
the [official website](https://prometheus.io/docs/prometheus/latest/getting_started/).

To install Grafana, you can follow the instructions on
the [official website](https://grafana.com/docs/grafana/latest/setup-grafana/installation/).

## Notes on project creation

EigenLayer middleware was installed as a submodule with:

```sh
mkdir contracts
cd contacts
forge init . --no-commit
forge install Layr-Labs/eigenlayer-middleware@mainnet
```

Then, to solve the issue<https://github.com/Layr-Labs/eigenlayer-middleware/issues/229>, we changed it to:

`forge install yetanotherco/eigenlayer-middleware@yac-mainnet --no-commit`

As soon as it gets fixed in mainnet, we can revert it.

Base version of middleware used is `7229f2b`.

The script to initialize the devnet can be found on `contracts/scripts/anvil`.

The addresses of the relevant contracts after running the anvil script are dumped
on `contracts/script/output/devnet`.

The state is backed up on `contracts/scripts/anvil/state`.

EigenLayer contract deployment is almost the same as the EigenLayer contract deployment on mainnet.
Changes are described in the file.

## Running Fuzzers:

Fuzzing for the operator can be done by executing the following make commands from the root directory of the project.

macOS:

```
make operator_verification_data_fuzz_macos
```

Linux:

```
operator_verification_data_fuzz_linux
```
