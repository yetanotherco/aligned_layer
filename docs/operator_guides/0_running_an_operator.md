# Register as an Aligned operator in testnet

> **CURRENT VERSION:**
> Aligned Operator [v0.14.0](https://github.com/yetanotherco/aligned_layer/releases/tag/v0.14.0)

> **IMPORTANT:** 
> You must be [whitelisted](https://docs.google.com/forms/d/e/1FAIpQLSdH9sgfTz4v33lAvwj6BvYJGAeIshQia3FXz36PFfF-WQAWEQ/viewform) to become an Aligned operator.

## Requirements

This guide assumes you are already [registered as an operator with EigenLayer](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation).

## Hardware Requirements

Minimum hardware requirements:

| Component     | Specification     |
|---------------|-------------------|
| **CPU**       | 16 cores          |
| **Memory**    | 32 GB RAM         |
| **Bandwidth** | 1 Gbps            |
| **Storage**   | 256 GB disk space |

## Supported Strategies

The list of supported strategies can be found [here](../3_guides/7_contract_addresses.md).

## Step 1 - Clone the repo

To start with, clone the Aligned repository and move inside it

```bash
git clone https://github.com/yetanotherco/aligned_layer.git --branch v0.14.0
cd aligned_layer
```

## Step 2 - Building the Operator

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

To build the operator binary for **Testnet**, run:

```bash
make build_operator ENVIRONMENT=testnet
```

To build the operator binary for **Mainnet**, run:

```bash
make build_operator ENVIRONMENT=mainnet
```

### Upgrading the Operator

If you want to upgrade the operator in **Testnet**, run:

```bash
make update_operator ENVIRONMENT=testnet
```

If you want to upgrade the operator in **Mainnet**, run:

```bash
make update_operator ENVIRONMENT=mainnet
```

This will recreate the binaries. You can then proceed to restart the operator.

You can find the latest version of the operator [here](https://github.com/yetanotherco/aligned_layer/releases).

### Checking the Operator Version

To see the operator version, run:

```bash
./operator/build/aligned-operator --version
```

This will display the current version of the operator binary.

## Step 3 - Update the configuration for your specific Operator

Locate the appropiate `operator_config_file`:

- Mainnet: `./config-files/config-operator-mainnet.yaml`.
- Holesky: `./config-files/config-operator-holesky.yaml`.

Update the following placeholders:

- `"<operator_address>"`
- `"<earnings_receiver_address>"`
- `"<ecdsa_key_store_location_path>"`
- `"<ecdsa_key_store_password>"`
- `"<bls_key_store_location_path>"`
- `"<bls_key_store_password>"`

`"<ecdsa_key_store_location_path>"` and `"<bls_key_store_location_path>"` are the paths to your keys generated with the EigenLayer CLI, `"<operator_address>"` and `"<earnings_receiver_address>"` can be found in the `operator.yaml` file created in the EigenLayer registration process.

The keys are stored by default in the `~/.eigenlayer/operator_keys/` directory, so for example `<ecdsa_key_store_location_path>` could be `/path/to/home/.eigenlayer/operator_keys/some_key.ecdsa.key.json` and for `<bls_key_store_location_path>` it could be `/path/to/home/.eigenlayer/operator_keys/some_key.bls.key.json`.

{% hint style="danger" %}

Don't keep the Operator Key in the Aligned Operator Node. If you already registered, don't use it. If you need to register, delete it after step 4.

{% endhint %}

The ECDSA key is only used for registration and funding of the operator and is not needed afterwards. It is recommended that you remove it after you're done, as well as the `ecdsa` section in the config file, or better yet for that data to never make it to the server (e.g., you run the registration from a machine without listening ports).  
If you run the registration on the server, it's recommended to do this part on a RAM filesystem to ease secure removal, and only after removing the `ecdsa` section move the config file to persistent storage.

If you run on a different computer, you will need to copy the BLS key store to the server.

Two RPCs are used, one as the main one, and the other one as a fallback in case one node is working unreliably. 

Default configurations is set up to use the same public node in both scenarios. 

{% hint style="danger" %}

PUBLIC NODES SHOULDN'T BE USED AS THE MAIN RPC. We recommend not using public nodes at all. 

FALLBACK AND MAIN RPCs SHOULD BE DIFFERENT. 

{% endhint %}

Most of the actions will pass through the main RPC unless there is a problem with it. Events are fetched from both nodes.

```yaml
eth_rpc_url: "https://<RPC_1>" 
eth_rpc_url_fallback: "https://<RPC_2>"
eth_ws_url: "wss://<RPC_1>"
eth_ws_url_fallback: "wss://<RPC_2>"
```

## Step 4 - Register Operator on AlignedLayer

Then you must register as an Operator on AlignedLayer. To do this, you must run:

- Mainnet:

```bash
make operator_register_with_aligned_layer CONFIG_FILE=./config-files/config-operator-mainnet.yaml
```

- Holesky:

```bash
make operator_register_with_aligned_layer CONFIG_FILE=./config-files/config-operator-holesky.yaml
```

{% hint style="danger" %}
If you are going to run the server in this machine, 
delete the operator key
{% endhint %}

## Step 5 - Start the operator

- Mainnet:

```bash
./operator/build/aligned-operator start --config ./config-files/config-operator-mainnet.yaml
```

- Holesky:

```bash
./operator/build/aligned-operator start --config ./config-files/config-operator-holesky.yaml
```

### Run Operator using Systemd

To manage the Operator process on Linux systems, we recommend use systemd with the following configuration:

You should create a user and a group in order to run the Operator and set the service unit to use that. In the provided service unit, we assume you have already created a user called `aligned`

```toml
# aligned-operator.service

[Unit]
Description=Aligned Operator
After=network.target

[Service]
Type=simple
User=aligned
ExecStart=<path_to_aligned_layer_repository>/operator/build/aligned-operator start --config <path_to_operator_config_file>
Restart=always
RestartSec=1
StartLimitBurst=100

[Install]
WantedBy=multi-user.target
```

{% hint style="info" %}
`aligned-operator.service` is just an arbitrary name. You can name your service as you wish, following the format `<service-name>.service`.
{% endhint %}

Once you have configured the `aligned-operator.service` file, you need to run the following commands:

```shell
sudo cp aligned-operator.service /etc/systemd/system/aligned-operator.service
sudo systemctl enable --now aligned-operator.service
```

{% hint style="warning" %}
All paths must be absolute.
{% endhint %}

Those commands will link the service to systemd directory and then, will start the Operator service.

Also, if the server running the operator goes down, systemd will start automatically the Operator on server startup.

#### Restart operator

If you want to restart the operator, you can use the following command:

```shell
sudo systemctl restart aligned-operator.service
```

#### Get Operators logs

Once you are running your operator using systemd, you can get its logs using journalctl as follows:

```shell
journalctl -xfeu aligned-operator.service
```

## Unregistering the operator

To unregister the Aligned operator, run:

- Mainnet:

```bash
cast send --rpc-url https://ethereum-rpc.publicnode.com --private-key <private_key> 0xA8CC0749b4409c3c47012323E625aEcBA92f64b9 'deregisterOperator(bytes)' 0x00
 ```

- Holesky:

```bash
cast send --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> 0x3aD77134c986193c9ef98e55e800B71e72835b62 'deregisterOperator(bytes)' 0x00
 ```

 `<private_key>` is the one specified in the output when generating your keys with the EigenLayer CLI.


##   Deposit Strategy Tokens in Testnet

We are using [WETH](https://holesky.eigenlayer.xyz/restake/WETH) as the strategy token.

To do so, there are two options, either doing it through EigenLayer's website, and following their guide, or running the commands specified by us below.

You will need to stake a minimum of 1000 WEI in WETH. We recommend to stake a maximum amount of 10 WETH. If you are staking more than 10 WETH please unstake any surplus over 10.

### Option 1

EigenLayer's guide can be found [here](https://docs.eigenlayer.xyz/eigenlayer/restaking-guides/restaking-user-guide/liquid-restaking/restake-lsts).

### Option 2

If you have ETH and need to convert it to WETH you can use the following command, that will convert 1 ETH to WETH.
Make sure to have [foundry](https://book.getfoundry.sh/getting-started/installation) already installed.
Change the parameter in ```---value``` if you want to wrap a different amount:

```bash
cast send 0x94373a4919B3240D86eA41593D5eBa789FEF3848 --rpc-url https://ethereum-holesky-rpc.publicnode.com --private-key <private_key> --value 1ether
```

Here `<private_key>` is the placeholder for the ECDSA key specified in the output when generating your keys with the EigenLayer CLI.

Finally, to end the staking process, you need to deposit into the WETH strategy,
as shown in the EigenLayer guide.

<details>
  <summary>An alternative using the CLI</summary>

Run the following command to deposit one WETH

  ```bash
  ./operator/build/aligned-operator deposit-into-strategy --config ./config-files/config-operator.yaml --strategy-address 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9 --amount 1000000000000000000
  ```

</details>

If you don't have Holesky ETH, these are some useful faucets:

- [Google Cloud for Web3 Holesky Faucet](https://cloud.google.com/application/web3/faucet/ethereum/holesky)
- [Holesky PoW Faucet](https://holesky-faucet.pk910.de/)

