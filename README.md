# Aligned Layer

> [!CAUTION]
> To be used in testnet only.

Basic repo demoing a Stark/Snark verifier AVS middleware with full EigenLayer integration. 

## The Project 

Aligned Layer works with EigenLayer to leverage ethereum consensus mechanism for ZK proof verification. Working outside the EVM, this allows for cheap verification of any proving system. This enables the usage of cutting edge algorithms, that may use new techniques to prove even faster. Even more, proving systems that reduces the proving overhead and adds verifier overhead, now become economically feasable to verify thanks to Aligned Layer. 

Full documentation and examples will be added soon

## Dependencies
You will need [go](https://go.dev/doc/install) [foundry](https://book.getfoundry.sh/getting-started/installation) and [zap-pretty](https://github.com/maoueh/zap-pretty) to run the examples below.

To install
```bash
make deps
```

## Run using make

To deploy EigenLayer contracts to local anvil testnet and save state
```bash
make anvil-deploy-eigen-contracts
```

Start anvil with saved state in a separate terminal:
```bash
make anvil-start
```

The above command starts a local anvil chain from a [saved state](./tests/integration/eigenlayer-and-shared-avs-contracts-deployed-anvil-state.json) with EigenLayer and Aligned Layer contracts already deployed (but no operator registered).