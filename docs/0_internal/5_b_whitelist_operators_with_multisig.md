# Whitelist Operators with a Multisig

> [!WARNING]  
> Safe Multisig Wallet is not currently supported in Holesky Testnet.
> For this reason, we deployed EigenLayer contracts in Sepolia to test the upgrade on AlignedLayer Contracts.

> [!NOTE]
> EigenLayer Sepolia contracts information is available in `contracts/script/output/sepolia/eigenlayer_deployment_output.json`.

> [!NOTE]
> You can find a guide on how to Deploy the contracts [here](./2_deploy_contracts.md).

This guide is for Whitelisting Operators using a Multisig wallet. If you deployed the contract without a Multisig wallet, you can follow the [Whitelist Operators](./5_a_whitelist_operators.md) tutorial.

In this guide we add and remove Operators from Aligned's Whitelist using a multisig wallet.

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
    foundryup -v nightly-a428ba6ad8856611339a6319290aade3347d25d9
    ```

## Steps for Whitelisting an Operator

1. Propose the transaction with the multisig following the [Propose Whitelist Guide](./5_b_1_propose_whitelist.md).

2. After the Whitelist is proposed, multisig participants can approve the whitelist transaction following the [Approve Whitelist Guide](./5_b_2_approve_whitelist.md).

## Steps for Removing an Operator

1. Propose the transaction with the multisig following the [Propose Remove Whitelist Guide](./5_b_3_propose_remove_whitelist.md).

2. After the unpause is proposed, multisig participants can approve the remove following the [Approve Remove Whitelist Guide](./5_b_4_approve_remove_whitelist.md).
