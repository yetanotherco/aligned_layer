# Deploying Aligned Contracts

This guide will walk you through the deployment of the Aligned Layer contracts.

Also, you will be able to deploy the Batcher Payment Service contract.

## Prerequisites

- You need to have installed `git` and `make`.

- Clone the repository

   ```sh
   git clone https://github.com/yetanotherco/aligned_layer.git
   ```

- Install foundry

    ```shell
    make install_foundry
    foundryup -i nightly-a428ba6ad8856611339a6319290aade3347d25d9
    ```
  
## AlignedServiceManager Contracts

1. You should have a keystore for the Aggregator, with the seedphrase saved on paper

2. If you don't have an API Key on Etherscan, create one following this [guide](https://docs.etherscan.io/getting-started/creating-an-account).

3. If you don't have it, create a wallet to deploy Aligned with ```cast wallet new```. and keep the ```Address``` and ```Private Key```values

4. Transfer around 35M gas to the ```Address```

5. ```CD``` into the Aligned repo

6. Set the ```PRIVATE_KEY``` and ```ETHERSCAN_API_KEY```on the ```.env``` used to deploy. ```contracts/scripts/.env.mainnet``` or ```contracts/scripts/.env.holesky``` or ```contracts/scripts/.env.sepolia```

7. Set `aggregator` value of ```contracts/script/deploy/config/mainnet/aligned.mainnet.config.json``` or ```contracts/script/deploy/config/holesky/aligned.holesky.config.json``` or ```contracts/script/deploy/config/sepolia/aligned.sepolia.config.json``` to the address from step 1.

8. Set `deployer` value of ```contracts/script/deploy/config/mainnet/aligned.mainnet.config.json``` or ```contracts/script/deploy/config/holesky/aligned.holesky.config.json``` or ```contracts/script/deploy/config/sepolia/aligned.sepolia.config.json``` to the address from step 3.

9. Set `owner`, `upgrader`, `churner`, `ejector` and `pauser` values of ```contracts/script/deploy/config/mainnet/aligned.mainnet.config.json``` or ```contracts/script/deploy/config/holesky/aligned.holesky.config.json``` or ```contracts/script/deploy/config/sepolia/aligned.sepolia.config.json``` to the multisig controlling this processes, or the owner of the deploying address, depending on what you want.

10. Deploy the contracts with the following command:

    For **Mainnet** deployment:

    ```bash
    make deploy_aligned_contracts NETWORK=mainnet
    ```

    For **Holesky** deployment:

    ```bash
    make deploy_aligned_contracts NETWORK=holesky
    ```

    For **Sepolia** deployment:

    ```bash
    make deploy_aligned_contracts NETWORK=sepolia
    ```

    If the deployment is correct, you will find the deployment information here:

    - Mainnet: `contracts/script/output/mainnet/alignedlayer_deployment_output.json`
    - Holesky: `contracts/script/output/holesky/alignedlayer_deployment_output.json`
    - Sepolia: `contracts/script/output/sepolia/alignedlayer_deployment_output.json`

11. Upgrade the new contracts addresses in docs/3_guides/7_contract_addresses.md

12. Create a PR with the new contract addresses and the updated docs.

## BatcherPaymentsService Contracts

1. You should have a keystore for the Batcher, with the Seedphrase saved on paper.

2. If you don't have an API Key on Etherscan, create one following this [guide](https://docs.etherscan.io/getting-started/creating-an-account).

3. If you don't have it, create a wallet to deploy BatcherPaymentsService with ```cast wallet new```. and keep the ```Address``` and ```Private Key```values

4. Transfer around 2.5M gas to the ```Address```

5. Set the ```PRIVATE_KEY``` and ```ETHERSCAN_API_KEY```on the ```.env``` used to deploy. ```contracts/scripts/.env.mainnet``` or ```contracts/scripts/.env.holesky``` or ```contracts/scripts/.env.sepolia```

6. Set `owner` value of `contracts/script/deploy/config/mainnet/batcher-payment-service.mainnet.config.json` or `contracts/script/deploy/config/holesky/batcher-payment-service.holesky.config.json` or `contracts/script/deploy/config/sepolia/batcher-payment-service.sepolia.config.json` to the multisig controlling this contract, or the owner of the deploying address, depending on what you want.

7. Set `batcherWallet` value of `contracts/script/deploy/config/mainnet/batcher-payment-service.mainnet.config.json` or `contracts/script/deploy/config/holesky/batcher-payment-service.holesky.config.json` or `contracts/script/deploy/config/sepolia/batcher-payment-service.sepolia.config.json` to the address from step 1.

8. Set `alignedLayerServiceManager` value of `contracts/script/deploy/config/mainnet/batcher-payment-service.mainnet.config.json` or `contracts/script/deploy/config/holesky/batcher-payment-service.holesky.config.json` or `contracts/script/deploy/config/sepolia/batcher-payment-service.sepolia.config.json` to the address of the AlignedServiceManager contract deployed on [AlignedServiceManager Contracts](#alignedservicemanager-contracts).

9. Deploy the contracts with the following command:

    For **Mainnet** deployment:

    ```bash
    make deploy_batcher_payment_service NETWORK=mainnet
    ```

    For **Holesky** deployment:

    ```bash
    make deploy_batcher_payment_service NETWORK=holesky
    ```

    For **Sepolia** deployment:

    ```bash
    make deploy_batcher_payment_service NETWORK=sepolia
    ```

    If the deployment is correct, you will find the deployment information here:

    - Mainnet: `contracts/script/output/mainnet/alignedlayer_deployment_output.json`
    - Holesky: `contracts/script/output/holesky/alignedlayer_deployment_output.json`
    - Sepolia: `contracts/script/output/sepolia/alignedlayer_deployment_output.json`

10. Upgrade the new contracts addresses in docs/3_guides/7_contract_addresses.md

11. Create a PR with the new contract addresses and the updated docs.
