# Deploy New Implementation

To deploy a new implementation, you can follow the steps below.

## Prerequisites

1. Make sure you have set variables as specified in the [Deploy Contracts Guide](./2_deploy_contracts.md).

2. Set ```MULTISIG=true``` on the ```.env``` used to deploy. ```contracts/scripts/.env.mainnet``` or ```contracts/scripts/.env.hoodi``` or ```contracts/scripts/.env.sepolia```

## Deploy New Implementation for AlignedLayerServiceManager

1. Deploy the new implementation by running:

   For **Mainnet** deployment:

   ```bash
    make upgrade_aligned_contracts NETWORK=mainnet
   ```

    For **Hoodi** deployment:

    ```bash
     make upgrade_aligned_contracts NETWORK=hoodi
    ```

    For **Sepolia** deployment:

    ```bash
     make upgrade_aligned_contracts NETWORK=sepolia
    ```

   If the new implementation is correctly deployed, the terminal will show the following message:

   ```html
   The new aligned layer service manager implementation is <new_aligned_layer_service_manager_implementation>

   You can propose the upgrade transaction with the multisig using this calldata
   <calldata>
   ```

   Also, the ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json``` file will be updated with the new implementation address depending on the network you are deploying.

2. Create a PR with the new address for the AlignedLayerServiceManagerImplementation.

## Deploy New Implementation for BatcherPaymentService

1. Deploy the new implementation by running:

    For **Mainnet** deployment:

    ```bash
    make upgrade_batcher_payment_service NETWORK=mainnet
    ```

    For **Hoodi** deployment:

     ```bash
     make upgrade_batcher_payment_service NETWORK=hoodi
     ```

    For **Sepolia** deployment:

     ```bash  
     make upgrade_batcher_payment_service NETWORK=sepolia
     ```

   If the new implementation is correctly deployed, the script will show the following message:

   ```html
   The new Batcher Payment Service Implementation is <new_batcher_payment_service_implementation>

   You can propose the upgrade transaction with the multisig using this calldata
   <calldata>
   ```

   Also, the ```contracts/script/output/mainnet/alignedlayer_deployment_output.json``` or ```contracts/script/output/hoodi/alignedlayer_deployment_output.json``` or ```contracts/script/output/sepolia/alignedlayer_deployment_output.json``` file will be updated with the new implementation address depending on the network you are deploying.

2. Create a PR with the new address for the BatcherPaymentServiceImplementation.

## Next Steps

Once you have deployed the new implementation of the contract you want to upgrade, you need to propose the upgrade transaction to the mulstisig, following this [guide](./3_b_2_propose_upgrade.md).
