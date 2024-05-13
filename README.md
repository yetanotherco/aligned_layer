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
make install_foundry
```

Then follow the command line instructions
Make sure to run `foundryup`

To install eigenlayer-cli

```bash
make install-eigenlayer_cli
```

## How to use Devnet

### Start anvil

Start anvil with every relevant contract deployed with:

```bash
make anvil_start
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and AlignedLayer contracts already deployed (but no operator registered).

### Start aggregator

Make sure to set config file variables to correct value at `config-files/config.yaml`.

To start the aggregator with a default configuration, run:

```bash
make aggregator_start
```

To use some custom configuration, set the `CONFIG_FILE` parameter with the path to your configuration file:

```bash
make aggregator_start CONFIG_FILE=<path_to_your_config> 
```

To run dummy operator to test aggregator SubmitTaskResponse endpoint, run:

```bash
make aggregator_send_dummy_responses
```

Make sure to have aggregator running on another terminal.
That command sends one dummy response to the aggregator with a task index of 0.

If you use task sender to send a task, you will see response changes from 1 to 0,
since the aggregator will have a task index of 0.

### Start operator

First make sure to build SP1 with:

```bash
make build_sp1_macos # or make build_sp1_linux on linux
```

To do the full process (register with EigenLayer, deposit into strategy and register with AligendLayer), run:

```bash
make operator_full_registration CONFIG_FILE=<path_to_config_file>
```

Once the registration process is done, start the operator with

```bash
make operator_start CONFIG_FILE=<path_to_config_file>
```

If `CONFIG_FILE` is not provided, it will use the default config file at `config-files/config.yaml`.

To use the default configuration file, just run:

```bash
make build_sp1_macos # or make build_sp1_linux on linux
make operator_full_registration
make operator_start
```

### Send task

### Sending a Task Using the TaskSender CLI

To send a task to the ServiceManager using the TaskSender CLI with a specific proving system, you can use one of the following commands depending on the proving system you wish to use:

For BLS12_381

```bash
  make send_plonk_bls12_381_proof
```

For BN254

```bash
  make send_plonk_bn254_proof
```

This will send a dummy task to the ServiceManager and an event will be emitted.
You should see logs from the operator with the received task's index.
Make sure to have the ServiceManager deployed and anvil running on another terminal or background.

The plonk proofs are located at `task_sender/test_examples`.

You can also send a task with a da by running:

```bash
  make send_plonk_bls12_381_proof DA_SOLUTION=<calldata|eigen|celestia>
```

This also works for any other proof type.

### Sending a task to be stored in Celestia

First, you will need to install the celestia-node CLI. Refer to [this resource](https://docs.celestia.org/nodes/celestia-node#installing-from-source)
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

Try sending a task with:

```bash
make send_plonk_bls12_381_proof DA_SOLUTION=celestia
```

You will get an error like `...Message: rpc error: code = NotFound desc = account <account_id> not found`. This means you don't have funds in your account.

To get funds in your account, access [this](https://faucet.celestia-arabica-11.com/) faucet and enter your account_id.

Finally, run:

```bash
make send_plonk_bls12_381_proof DA_SOLUTION=celestia
 ```

## Developing workflows in testnet

### Upgrade contracts

When changing EigenLayer contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_eigen_contracts
```

You will also need to redeploy the MockStrategy & MockERC20 contracts:

```bash
make anvil_deploy_mock_strategy
```

When changing AlignedLayer contracts, the anvil state needs to be updated with:

```bash
make anvil_deploy_aligned_contracts
```

Also make sure to re-generate the Go smart contract bindings:

```bash
make bindings
```

### Operator registration step by step (WIP Guide)

When not using the default address, get eth with:

```bash
make operator_get_eth
```

Update the config in:

```operator/config/devnet/config.yaml```
```operator/config/devnet/operator.yaml```

To register with EigenLayer, run:

```bash
make operator_register_with_eigen_layer
```

To get mock tokens (DEVNET ONLY), run:

```bash
make operator_mint_mock_tokens
```

To deposit into strategy, and register with AlignedLayer, run:

```bash
make operator_deposit_and_register
```

To just deposit into the strategy run:

```bash
export STRATEGY_ADDRESS=<strategy_address> && make operator_deposit_into_strategy
```

To deposit into mock strategy (DEVNET ONLY), run:

```bash
make operator_deposit_into_mock_strategy
```

To just register an operator with AlignedLayer, run:

```bash
make operator_register_with_aligned_layer
```

## Testnet/Mainnet Deployment

To deploy the contracts to Testnet/Mainnet, you will need to set environment variables
in a .env file in the same directory as the deployment script (`contracts/scripts/`).
The variables are as follows:

| Variable                      | Description                                                           |
|-------------------------------|-----------------------------------------------------------------------|
| RPC_URL                       | The RPC URL of the network you want to deploy to.                     |
| PRIVATE_KEY                   | The private key of the account you want to deploy the contracts with. |
| EXISTING_DEPLOYMENT_INFO_PATH | The path to the file containing the deployment info about EigenLayer. |
| DEPLOY_CONFIG_PATH            | The path to the deployment config file.                               |
| OUTPUT_PATH                   | The path to the file where the deployment info will be saved.         |

Then run the following command:

```bash
make deploy_aligned_contracts
```

To get the existing deployment info about EigenLayer, you can download it
from [EigenLayer repo](https://github.com/Layr-Labs/eigenlayer-contracts/tree/dev/script/configs).

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

## Notes on project creation / devnet deployment

Eigenlayer middleware was installed as a submodule with:

```sh
mkdir contracts
cd contacts
forge init . --no-commit
forge install Layr-Labs/eigenlayer-middleware@mainnet
```

Then to solve the issue <https://github.com/Layr-Labs/eigenlayer-middleware/issues/229>, we changed it to:

```forge install yetanotherco/eigenlayer-middleware@yac-mainnet --no-commit```

As soon as it gets fixed in mainnet we can revert it.

Base version of middleware used is ```7229f2b```

The script to initialize the devnet can be found on  ```contracts/scripts/anvil```

The addresses of the relevant contracts after running the anvil script is dumped on ```contracts/script/output/devnet```.

The state is backuped on ```contracts/scripts/anvil/state```

Eigenlayer contract deployment is almost the same as the EigenLayer contract deployment on mainnet. Changes are described on the file.

### Strategies

The strategy contract is a contract where operators deposit restaked tokens.
For test purposes, we have a dummy strategy contract that takes a Mock ERC20 token.

### Aggregator

Current aggregator implementation is WIP. The RPC method `Aggregator.SubmitTaskResponse` expects a `SignedTaskResponse`
as body and returns 0 if args.TaskIndex exists, and 1 otherwise.

Check `common/types/signed_task_response.go` for specification on `SignedTaskResponse`.

### Operator

The following section is instructions on how to create an operator from scratch.
You can find more details on the [EigenLayer documentation](https://docs.eigenlayer.xyz/eigenlayer/operator-guides/operator-installation#create-and-list-keys).

To create an operator, you will need to generate keys, generate a config, and register with EigenLayer.

To generate the operator keys, run:

```bash
make operator_generate_keys
```

This will output key paths & address, make sure to store them for following steps.

To generate a new operator config, run the command

```bash
make operator_generate_config
```

Then follow the instructions to populate the file

You will then need to populate two additional values, which are _metadata_url_ and _el_delegation_manager_address_

To get the Delegation Manager Address of the last devnet deployment you can run:

```bash
make get_delegation_manager_address
```

For the metadata URL you can either use our example URL:
`https://yetanotherco.github.io/operator_metadata/`

Or Deploy your metadata to your own sever (can be GitHub Pages)

You can get devnet Ether for gas by running:

```bash
make operator_get_eth
```

Make sure to set `OPERATOR_ADDRESS` enviroment variable to your own address before running command.
This will send 1 eth to that address

Then you can register with EigenLayer by running:

```bash
make operator_register_with_eigen_layer
```

### Config File

In `config-files/config.yaml` you can find the configuration file for the project.

There is a section for operator, aggregator, and keys. Also, there are common variables for the project.

There are also three other configuration files in the `config-files` directory for operators. They have their own keys and addresses.

## FAQ

### What is the objective of Aligned Layer?

Aligned Layer’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain the zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t know what that future will look like, but we are certain it will be in Ethereum. The question we want to share is: If we are certain zero-knowledge proofs are the future of Ethereum but we are not certain which of the many possible zero-knowledge futures will win. How can we build an infrastructure for Ethereum to be compatible with any future zero-knowledge proving system?

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and help it increase its capabilities.

### What are the use cases of Aligned Layer?

Among the possible use cases of Aligned Layer we have:

Soft finality for Rollups and Appchains, fast bridging, new settlement layers (use Aligned + EigenDA) for Rollups and Intent based systems, P2P protocols based on SNARKs such as payment systems and social networks, alternative L1s interoperable with Ethereum, Verifiable Machine Learning, cheap verification and interoperability for Identity Protocols, ZK Oracles, new credential protocols such as zkTLS based systems, ZK Coprocessor, encrypted Mempools using SNARKs to show the correctness of the encryption, protocols against misinformation and fake news, and on-chain gaming.

### Why build on top of Ethereum?

Ethereum is the most decentralized and biggest source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned Layer is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

### Why not do this directly on top of Ethereum?

In order to do this we would have to aggregate all the proofs into a single proof. This is not a good solution considering that we would need some way to wrap proofs (for example, by means of recursion), which involves complex operations such as field emulation, bitwise, and/or elliptic curve operations.

### Why not make Aligned Layer a ZK L1?

An L1 would not have the security properties of Ethereum consensus, and bootstrapping a new decentralized network is not only expensive but might be an impossible task. Zero-knowledge proofs are a nascent technology, and change is a constant. The best solution for today may not be the best for tomorrow; modifying L1s is extremely costly, especially as time progresses.

### Why not a ZK L2?

An L2 needs to use the EVM to settle in Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.

The EVM is not designed for ZK Verification, so most verifications are expensive.

To solve this, for pairing-based cryptography, Ethereum has added a precompile for verifications using the curve BN254.

But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast Starks need efficient hashing for fields. Which is the best field? Mersenne’s? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?

The amount of progress in the field is big, and nobody can predict the endgame.

Even more, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems.

Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or one hash not created yet.

Aligned Layer solves all of this. No matter how or what you want to prove, it can be verified efficiently here while still inheriting the security of Ethereum as other L2s.

### Why EigenLayer?

We believe Ethereum is the best settlement layer, and zero-knowledge will play a key role in helping it be THE settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs, but how can we build such a decentralized network that will help Ethereum? Creating a new L1 doesn’t benefit Ethereum because using it will add new trust assumptions to the Ethereum protocols relying on it. So, if we must have:

1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in Ethereum
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### Will you aggregate proofs?

Proof aggregation can also be supported by proving the verification of many of these different verifications. This will likely not be an urgent feature, but it will be needed in the future with more demand.

### How does it compare to the Polygon aggregation layer?

Aligned Layer is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned Layer, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.
