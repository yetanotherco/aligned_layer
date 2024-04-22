# Aligned Layer

> [!CAUTION]
> To be used in testnet only.

Basic repo demoing a Stark/Snark verifier AVS middleware with full EigenLayer integration. 

## The Project 

Aligned Layer works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduces the proving overhead and adds verifier overhead, now become economically feasable to verify thanks to Aligned Layer. 

Full documentation and examples will be added soon

## Dependencies

You will need [go](https://go.dev/doc/install), [foundry](https://book.getfoundry.sh/getting-started/installation) and [zap-pretty](https://github.com/maoueh/zap-pretty) to run the examples below.

To install
```bash
make deps
```

To install foundry
```bash
make install-foundry
```
Then follow the command line instructions
Make sure to run `foundryup`

## Run using make

Start anvil with every relevant contract deployed with:

```bash
make anvil-start
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and AlignedLayer contracts already deployed (but no operator registered).


## Dev notes

When changing EigenLayer contracts, the anvil state needs to be updated with:

```bash
make anvil-deploy-eigen-contracts
```

When changing AlignedLayer contracts, the anvil state needs to be updated with:

```bash
make anvil-deploy-aligned-contracts
```

To redeploy all contracts, run:
```bash
make anvil-deploy-all
```

To start the aggregator, run:
```bash
make aggregator-start
```

To run aggregator tests, run:
```bash
make aggregator-test
```

To run dummy operator to test aggregator SubmitTaskResponse endpoint, run:
```bash
make aggregator-send-dummy-responses
```

## Notes on project creation / devnet deployment

Eigenlayer middleware was installed as a submodule with:

```
mkdir contracts
cd contacts
forge init . --no-commit
forge install Layr-Labs/eigenlayer-middleware@mainnet
```

Then to solve the issue https://github.com/Layr-Labs/eigenlayer-middleware/issues/229, we changed it to:

```forge install yetanotherco/eigenlayer-middleware@yac-mainnet --no-commit```

As soon as it gets fixed in mainnet we can revert it.

Base version of middleware used is ```7229f2b```

The script to initialize the devnet can be found on  ```contracts/scripts/anvil```

The addresses of the relevant contracts after running the anvil script is dumped on ```contracts/script/output/devnet```.

The state is backuped on ```contracts/scripts/anvil/state```

Eigenlayer contract deployment is almost the same as the EigenLayer contract deployment on mainnet. Changes are described on the file.

### Aggregator

Current aggregator implementation is WIP. The RPC method `Aggregator.SubmitTaskResponse` expects a `SignedTaskResponse` as body and returns 0 if args.TaskResponse is not empty, and 1 otherwise.

Check `common/types/signed_task_response.go` for specification on `SignedTaskResponse`.
