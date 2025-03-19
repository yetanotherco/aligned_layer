# Pause Contracts with a Multisig

> [!WARNING]  
> Safe Multisig Wallet is not currently supported in Holesky Testnet.
> For this reason, we deployed EigenLayer contracts in Sepolia to test the upgrade on AlignedLayer Contracts.

> [!NOTE]
> EigenLayer Sepolia contracts information is available in `contracts/script/output/sepolia/eigenlayer_deployment_output.json`.

> [!NOTE]
> You can find a guide on how to Deploy the contracts [here](./2_deploy_contracts.md).

This guide is for pausing contracts deployed using a Multisig wallet. If you deployed the contract without a Multisig wallet, you can follow the [Pause Contracts](./4_a_pause_contracts.md) tutorial.

In this guide we pause and unpause the AlignedLayerServiceManager and BatcherPaymentService contracts using a multisig wallet.

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

## Steps for Pausing

1. Propose the transaction with the multisig following the [Propose Pause Guide](./4_b_1_propose_pause.md).

2. After the pause is proposed, multisig participants can approve the pause following the [Approve Pause Guide](./4_b_2_approve_pause.md).

## Steps for Unpausing

1. Propose the transaction with the multisig following the [Propose Unpause Guide](./4_b_3_propose_unpause.md).

2. After the unpause is proposed, multisig participants can approve the unpause following the [Approve Unpause Guide](./4_b_4_approve_unpause.md).
