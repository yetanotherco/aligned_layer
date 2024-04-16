# Aligned Layer

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

To start anvil with saved state:
```bash
make anvil-start
```