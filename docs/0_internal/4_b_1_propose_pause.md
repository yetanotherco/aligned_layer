# Propose the Transaction for Pause using Multisig

If you want to pause the contracts, you can propose the pause transaction using the multisig wallet.

## Prerequisites

- You need to have deployed the contracts following the [Deploy Contracts Guide](./2_deploy_contracts.md).

## Propose transaction for Pause AlignedLayerServiceManager

To propose the pause transaction you can follow the steps below:

1. Go to [Safe](https://app.safe.global/home)

2. Click on `New transaction` -> `Transaction Builder`

   ![New transaction](./images/4_b_1_pause_1.png)

   ![Transaction Builder](./images/4_b_1_pause_2.png)

3. Get the `AlignedLayerServiceManager` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/holesky/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```

4. Paste the `AlignedLayerServiceManager` address on `Enter Address or ENS Name`

   ![Enter Address](./images/4_b_1_pause_3.png)

5. As this is a Proxy contract, choose `Use Implementation ABI`

   ![Use Implementation ABI](./images/4_b_1_pause_4.png)

6. In `contract method selector` choose `pauseAll()`

   ![Choose pause](./images/4_b_1_pause_5.png)

7. Click on `+ Add new transaction`

   You should see the new transaction to be executed

8. Click on `Create batch` to create the transaction.

9. Simulate the transaction by clicking on `Simulate`

10. If everything is correct, click on `Send batch` to send the transaction.

11. Simulate the transaction, and if everything is correct, click on `Sign`.

   ![Send batch](./images/4_b_1_pause_6.png)

> [!NOTE]
> In the `call` field, you will see `fallback`.

12. Wait for the transaction to be executed. You can check the transaction status on the `Transactions` tab.


## Propose transaction for Pause BatcherPaymentService

To propose the pause transaction you can follow the steps below:

1. Create the pause transaction on [Safe](https://app.safe.global/home)

2. Click on `New transaction` -> `Transaction Builder`

   ![New transaction](./images/4_b_1_pause_1.png)

   ![Transaction Builder](./images/4_b_1_pause_2.png)

3. Get the `BatcherPaymentService` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/holesky/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```

4. Paste the `BatcherPaymentService` address on `Enter Address or ENS Name`

   ![Enter Address](./images/4_b_1_pause_3.png)

5. As this is a Proxy contract, choose `Use Implementation ABI`

   ![Use Implementation ABI](./images/4_b_1_pause_4.png)

6. In `contract method selector` choose `pause()`

   ![Choose pause](./images/4_b_1_pause_8.png)

7. Then click on `+ Add new transaction`

   You should see the new transaction to be executed. Then click on `Create batch` to create the transaction.

   ![Add new transaction](./images/4_b_1_pause_9.png)

8. Review and confirm you are interacting with the correct `BatcherPaymentService` contract and you are calling the `pause` function.

   ![Review transaction](./images/4_b_1_pause_10.png)

9. Simulate the transaction by clicking on `Simulate`

10. If everything is correct, click on `Send batch` to send the transaction.

11. Review the transaction and click on `Sign` to sign the transaction.

   ![Send batch](./images/4_b_1_pause_11.png)

12. If the transaction is correctly created, you have to wait until the required Multisig member signs the transaction to send it. For this, you can follow [the following guide](./4_b_2_approve_pause.md)
