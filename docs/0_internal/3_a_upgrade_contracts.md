# Upgrade Contracts

This guide is for upgrading contracts with a normal wallet.

If you deployed the contract using a Multisig wallet, you can follow the [Upgrade Contracts with Multisig](./3_b_upgrade_contracts_with_multisig.md) tutorial.

## Prerequisites

- To upgrade any of the contracts, you need to have set the `.env`, `DEPLOY_CONFIG_PATH` and `BATCHER_PAYMENT_SERVICE_CONFIG_PATH`

- You need to have installed git and make.

- Clone the repository

   ```sh
   git clone https://github.com/yetanotherco/aligned_layer.git
   ```

- Install foundry

    ```sh
    make install_foundry
    foundryup -i nightly-a428ba6ad8856611339a6319290aade3347d25d9
    ```

- Add the following variables to the `.env` file:

    ```makefile
    MULTISIG=false
    ```

## Upgrade Service Manager

To upgrade the Service Manager Contract, run:

```bash
make upgrade_aligned_contracts
```

## Upgrade Registry Coordinator

To upgrade the Registry Coordinator, run:

```bash
make upgrade_registry_coordinator
```

## Upgrade Batcher Payment Service

To upgrade the Batcher Payment Service, run:

```bash
make upgrade_batcher_payment_service
```
