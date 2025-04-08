#!/bin/bash

RPC_URL="http://localhost:8545"

cd contracts

ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

sleep 1

# Deploy the contracts
forge script script/deploy/AlignedLayerDeployer.s.sol \
    ./script/output/devnet/eigenlayer_deployment_output.json \
    ./script/deploy/config/devnet/aligned.devnet.config.json \
    ./script/output/devnet/alignedlayer_deployment_output.json \
    --rpc-url $RPC_URL \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string memory existingDeploymentInfoPath, string memory deployConfigPath, string memory outputPath)"


ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

# Can't deploy on another script, current open issue: https://github.com/foundry-rs/foundry/issues/7952
forge script ../examples/verify/script/VerifyBatchInclusionCallerDeployer.s.sol \
    "$ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS" \
    --rpc-url $RPC_URL \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(address _targetContract)"

output_path=./script/output/devnet/batcher_deployment_output.json

# Deploy Batcher Payments Contract
forge script script/deploy/BatcherPaymentServiceDeployer.s.sol \
    ./script/deploy/config/devnet/batcher-payment-service.devnet.config.json \
    $output_path \
    --rpc-url $RPC_URL \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string batcherConfigPath, string outputPath)"

# Extract the batcher payment service values from the output
batcher_payment_service_proxy=$(jq -r '.addresses.batcherPaymentService' $output_path)
batcher_payment_service_implementation=$(jq -r '.addresses.batcherPaymentServiceImplementation' $output_path)

# Give initial funds to ServiceManager for the Batcher
cast send $ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS "depositToBatcher(address)()" $batcher_payment_service_proxy --value 1ether --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" --rpc-url $RPC_URL

cd ..
