#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/eigenlayer-deployed-anvil-state.json --dump-state state/alignedlayer-deployed-anvil-state.json &

cd ../../

ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

sleep 1

# Deploy the contracts
forge script script/deploy/AlignedLayerDeployer.s.sol \
    ./script/output/devnet/eigenlayer_deployment_output.json \
    ./script/deploy/config/devnet/aligned.devnet.config.json \
    ./script/output/devnet/alignedlayer_deployment_output.json \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string memory existingDeploymentInfoPath, string memory deployConfigPath, string memory outputPath)"

# Can't deploy on another script, current open issue: https://github.com/foundry-rs/foundry/issues/7952
forge script script/deploy/VerifyBatchInclusionCallerDeployer.s.sol \
    $ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --slow \
    --sig "run(address _targetContract)"


BATCHER_WALLET=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
# using anvil prefunded:
# (1) 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 (10000.000000000000000000 ETH)
# (1) 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d

# Deploy the contracts
forge script script/deploy/BatcherPaymentsDeployer.s.sol \
    $ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS \
    $BATCHER_WALLET \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(address existingDeploymentInfoPath, address deployConfigPath)"

# Kill the anvil process to save state
pkill anvil
