# Propose the Transaction for Upgrade using Multisig

After deploying a new implementation candidate for the upgrade, you need to propose the upgrade transaction using the multisig wallet.

## Propose transaction for AlignedLayerServiceManager

To propose the upgrade transaction you can follow the steps below:

1. Go to [Safe](https://app.safe.global/home)

2. Click on `New transaction` -> `Transaction Builder`

   ![New transaction](./images/3_b_2_multisig_1.png)

   ![Transaction Builder](./images/3_b_2_multisig_2.png)

3. Get the `alignedLayerProxyAdmin` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```, and paste it on `Enter Address or ENS Name`

4. Once you paste the address, the ABI should be automatically filled.

5. Choose the method ```upgrade``` in ```Contract method selector```.

6. Get the ```alignedLayerServiceManager``` and ```alignedServiceManagerImplementation``` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```.

7. Paste ```alignedLayerServiceManager``` address in ```proxy (address)```

8. Paste ```alignedServiceManagerImplementation``` address in ```implementation (address)```

9. Click on `Create batch` to create the transaction.

      ![Create batch](./images/3_b_2_multisig_7.png)

10. Review and confirm the transaction.

      To make sure everything is fine, simulate the transaction by clicking on `Simulate batch`

      Once the simulation is successful, click on `Send Batch` to send the transaction.

      ![Simulate batch](./images/3_b_2_multisig_8.png)

11. Confirm the transaction checking the function being called is correct and the contract address is the one you deployed.

      If everything is correct, click on `Sign` to send the transaction.

      ![Confirm transaction](./images/3_b_2_multisig_9.png)

12. Now in your transactions, you should be able to see the newly created transaction.

      ![New transaction](./images/3_b_2_multisig_10.png)

13. If the transaction is correctly created, you have to wait until the required Multisig member signs the transaction to send it.

A guide on how to sign the transaction can be found [here](./3_b_3_approve_upgrade.md)

## Propose transaction for BatcherPaymentService

1. Go to [Safe](https://app.safe.global/home)

2. Click on `New transaction` -> `Transaction Builder`

   ![New transaction](./images/3_b_2_multisig_1.png)

   ![Transaction Builder](./images/3_b_2_multisig_2.png)

3. Get the `batcherPaymentService` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```, and paste it on `Enter Address or ENS Name`

4. Once you paste the address, the ABI should be automatically filled.

5. Choose the method ```upgradeTo``` in ```Contract method selector```.

6. Get the ```batcherPaymentServiceImplementation``` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```.

7. Paste ```batcherPaymentServiceImplementation``` address in ```newImplementation (address)```

8. Click on `Create batch` to create the transaction.

   ![Create batch](./images/3_b_2_multisig_7.png)

9. Review and confirm the transaction.

   To make sure everything is fine, simulate the transaction by clicking on `Simulate batch`

   Once the simulation is successful, click on `Send Batch` to send the transaction.

   ![Simulate batch](./images/3_b_2_multisig_8.png)

10. Confirm the transaction checking the function being called is correct and the contract address is the one you deployed.

    If everything is correct, click on `Sign` to send the transaction.

    ![Confirm transaction](./images/3_b_2_multisig_9.png)

11. Now in your transactions, you should be able to see the newly created transaction.

    ![New transaction](./images/3_b_2_multisig_10.png)

12. If the transaction is correctly created, you have to wait until the required Multisig member signs the transaction to send it.

A guide on how to sign the transaction can be found [here](./3_b_3_approve_upgrade.md)
