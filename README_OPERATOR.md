
# Register as an Aligned operator in testnet

## Requirements

> [!NOTE]
> You must be whitelisted to become an Aligned operator.

This guide assumes you are already [registered as an operator with EigenLayer](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation).

## Hardware Requirements

Minimum hardware requirements:

| Component     | Specification     |
|---------------|-------------------|
| **CPU**       | 16 cores          |
| **Memory**    | 32 GB RAM         |
| **Bandwidth** | 1 Gbps            |
| **Storage**   | 256 GB disk space |

## Building from Source (Recommended)

We recommend building from source whenever possible. If using the docker image, these steps can be skipped.

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

To build the operator binary, run:

```bash
make build_operator
```

To update the operator, run:

```bash
git pull
make build_operator
```

This will recreate the binaries. You can then proceed to restart the operator.

## Configuration

## When building from source

Update the following placeholders in `./config-files/config-operator.yaml`:

- `"<operator_address>"`
- `"<earnings_receiver_address>"`
- `"<ecdsa_key_store_location_path>"`
- `"<ecdsa_key_store_password>"`
- `"<bls_key_store_location_path>"`
- `"<bls_key_store_password>"`

`"<ecdsa_key_store_location_path>"` and `"<bls_key_store_location_path>"` are the paths to your keys generated with the EigenLayer CLI, `"<operator_address>"` and `"<earnings_receiver_address>"` can be found in the `operator.yaml` file created in the EigenLayer registration process.
The keys are stored by default in the `~/.eigenlayer/operator_keys/` directory, so for example `<ecdsa_key_store_location_path>` could be `/path/to/home/.eigenlayer/operator_keys/some_key.ecdsa.key.json` and for `<bls_key_store_location_path>` it could be `/path/to/home/.eigenlayer/operator_keys/some_key.bls.key.json`.


## When using docker

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

## Deposit Strategy Tokens

We are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To do so there are 2 options, either doing it through EigenLayer's website, and following their guide, or running the commands specified by us below.

You will need to stake a minimum of a 1000 Wei in WETH. We recommend to stake a maximum amount of 10 WETH. If you are staking more than 10 WETH please unstake any surplus over 10.

## Option 1:
EigenLayer's guide can be found [here](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/liquid-restaking/restake-lsts).

## Option 2:
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

## Start the operator

## From Source (Recommended)

```
./operator/build/aligned-operator start --config ./config-files/config-operator.yaml
```

## Using Docker

Ensure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

Then run:

```bash
make operator_start_docker
```

## Unregister the operator from Aligned

To unregister the Aligned operator, run:

```bash
cast send --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> 0x3aD77134c986193c9ef98e55e800B71e72835b62 'deregisterOperator(bytes)' 0x00
 ```

 `<private_key>` is the one specified in the output when generating your keys with the EigenLayer CLI.
