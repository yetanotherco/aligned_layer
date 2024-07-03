# Register as an Aligned operator in testnet

> **IMPORTANT:** 
> You must be [whitelisted](https://docs.google.com/forms/d/e/1FAIpQLSdH9sgfTz4v33lAvwj6BvYJGAeIshQia3FXz36PFfF-WQAWEQ/viewform) to become an Aligned operator.

## Supported Verifiers

The following is the list of the verifiers currently supported by Aligned:

- :white_check_mark: gnark - Groth16 (with BN254)
- :white_check_mark: gnark - Plonk (with BN254 and BLS12-381)
- :white_check_mark: SP1 [(v1.0.8-testnet)](https://github.com/succinctlabs/sp1/releases/tag/v1.0.8-testnet)
- :white_check_mark: Risc0 [(v1.0.1)](https://github.com/risc0/risc0/releases/tag/v1.0.1)

The following proof systems are going to be added soon:

- :black_square_button: Kimchi
- :black_square_button: Halo2 - Plonk/KZG
- :black_square_button: Halo2 - Plonk/IPA

## Requirements

This guide assumes you are already [registered as an operator with EigenLayer](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation).

## Hardware Requirements

Minimum hardware requirements:

| Component     | Specification     |
| ------------- | ----------------- |
| **CPU**       | 16 cores          |
| **Memory**    | 32 GB RAM         |
| **Bandwidth** | 1 Gbps            |
| **Storage**   | 256 GB disk space |

## Step 1 - Clone the repo

To start with, clone the Aligned repository and move inside it

```bash
git clone https://github.com/yetanotherco/aligned_layer.git
cd aligned_layer
```

## Step 2 - Building the Operator

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

To see the operator version, run:

```bash
./operator/build/aligned-operator --version
```

This will display the current version of the operator binary.

## Step 3 - Update the configuration for your specific Operator

Update the following placeholders in `./config-files/config-operator.yaml`:

- `"<operator_address>"`
- `"<earnings_receiver_address>"`
- `"<ecdsa_key_store_location_path>"`
- `"<ecdsa_key_store_password>"`
- `"<bls_key_store_location_path>"`
- `"<bls_key_store_password>"`

`"<ecdsa_key_store_location_path>"` and `"<bls_key_store_location_path>"` are the paths to your keys generated with the EigenLayer CLI, `"<operator_address>"` and `"<earnings_receiver_address>"` can be found in the `operator.yaml` file created in the EigenLayer registration process.
The keys are stored by default in the `~/.eigenlayer/operator_keys/` directory, so for example `<ecdsa_key_store_location_path>` could be `/path/to/home/.eigenlayer/operator_keys/some_key.ecdsa.key.json` and for `<bls_key_store_location_path>` it could be `/path/to/home/.eigenlayer/operator_keys/some_key.bls.key.json`.

## Step 4 - Deposit Strategy Tokens

We are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To do so there are 2 options, either doing it through EigenLayer's website, and following their guide, or running the commands specified by us below.

You will need to stake a minimum of a 1000 Wei in WETH. We recommend to stake a maximum amount of 10 WETH. If you are staking more than 10 WETH please unstake any surplus over 10.

### Option 1

EigenLayer's guide can be found [here](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/liquid-restaking/restake-lsts).

### Option 2

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
  </summary>
</details>

If you don't have Holesky Eth, these are some useful faucets:

- [Google Cloud for Web3 Holesky Faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)
- [Holesky PoW Faucet](https://holesky-faucet.pk910.de/)

## Step 5 - Start the operator

```bash
./operator/build/aligned-operator start --config ./config-files/config-operator.yaml
```

## Unregistering the operator

To unregister the Aligned operator, run:

```bash
cast send --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> 0x3aD77134c986193c9ef98e55e800B71e72835b62 'deregisterOperator(bytes)' 0x00
 ```

 `<private_key>` is the one specified in the output when generating your keys with the EigenLayer CLI.
