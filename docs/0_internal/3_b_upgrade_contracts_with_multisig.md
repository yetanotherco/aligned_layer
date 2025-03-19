# Upgrade Contracts using a Multisig

> [!WARNING]  
> Safe Multisig Wallet is not currently supported in Holesky Testnet.
> For this reason, we deployed EigenLayer contracts in Sepolia to test the upgrade on AlignedLayer Contracts.

> [!NOTE]
> EigenLayer Sepolia contracts information is available in `contracts/script/output/sepolia/eigenlayer_deployment_output.json`.

> [!NOTE]
> You can find a guide on how to Deploy the contracts [here](./2_deploy_contracts.md).


This guide is for upgrading contracts deployed using a Multisig wallet. If you deployed the contract without a Multisig wallet, you can follow the [Upgrade Contracts](./3_a_upgrade_contracts.md) tutorial.

In this guide we make an upgrade of Aligned Layer Service Manager contract using a multisig wallet.

## Prerequisites

- You need to have installed
  - git
  - make 
  - [jq](https://jqlang.github.io/jq/download/)

- Clone the repository

   ```
   git clone https://github.com/yetanotherco/aligned_layer.git
   ```

- Install foundry

    ```shell
    make install_foundry
    foundryup -i nightly-a428ba6ad8856611339a6319290aade3347d25d9
    ```

## Steps

1. Deploy the new implementation following the [Deploy New Implementation Guide](./3_b_1_deploy_new_impl.md).

2. Once you have deployed the new implementation, you can propose the upgrade transaction with the multisig following the [Propose Upgrade Guide](./3_b_2_propose_upgrade.md).

3. After the upgrade is proposed, multisig participants can approve the upgrade following the [Approve Upgrade Guide](./3_b_3_approve_upgrade.md).

Finishing the approval process, the contract will be upgraded to the new implementation.
