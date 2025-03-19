# Propose the Transaction for Whitelisting Operators using Multisig

If you want to Whitelist Operators, you can propose the whitelist operator transaction using the multisig wallet.

## Prerequisites

- You need to have deployed the contracts following the [Deploy Contracts Guide](./2_deploy_contracts.md).

## Propose transaction for Whitelist Operators

To propose the whitelist transaction you can follow the steps below:

1. Go to [Safe](https://app.safe.global/home)

2. Click on `New transaction` -> `Transaction Builder`

   ![New transaction](./images/5_b_1_whitelist_operator_1.png)

   ![Transaction Builder](./images/5_b_1_whitelist_operator_2.png)

3. Get the `registryCoordinator` address from ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/holesky/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json```

4. Paste the `registryCoordinator` address on `Enter Address or ENS Name`

   ![](./images/5_b_1_whitelist_operator_3.png)

5. As this is a Proxy contract, choose `Use Implementation ABI`

   ![Use Implementation ABI](./images/5_b_1_whitelist_operator_4.png)

If `Use Implementation ABI`, did not show up you will need to submit the call via raw calldata. Consult this alternative [guide](./5_b_1b_propose_whitelist_with_call_data.md)
   
6. In `Contract Method Selector` choose `add_multiple()` in the `_addresses(address[])` field, enter the operator addresses in the following format `[<OPERATOR_ADDRESS>, ..., <OPERATOR_ADDRESS>]` for example, `[0000000000000000000000000000000000000009, 0000000000000000000000000000000000000003]`

   ![Choose the add_multiple()](./images/5_b_1_whitelist_operator_5.png)

7. Click on `+ Add new transaction`

   You should see the new transaction to be executed

   ![alt text](./images/5_b_1_whitelist_operator_6.png)

8. Click on `Create batch` to create the transaction.

   ![alt text](./images/5_b_1_whitelist_operator_7.png)

9.  Simulate the transaction by clicking on `Simulate`

   ![alt text](./images/5_b_1_whitelist_operator_8.png)

10. If everything is correct, click on `Send batch` to send the transaction.

11. Simulate the transaction, and if everything is correct, click on `Sign`.

   ![alt text](./images/5_b_1_whitelist_operator_9.png)

12. Wait for the transaction to be executed. You can check the transaction status on the `Transactions` tab.

13. If the transaction is correctly created, you have to wait until the required Multisig member signs the transaction to send it. For this, you can follow [the following guide](./5_b_2_approve_whitelist.md)
