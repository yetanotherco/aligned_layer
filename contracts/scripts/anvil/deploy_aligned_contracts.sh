#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/eigenlayer-deployed-anvil-state.json --dump-state state/alignedlayer-deployed-anvil-state.json &

cd ../../

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


ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

# Can't deploy on another script, current open issue: https://github.com/foundry-rs/foundry/issues/7952
forge script ../examples/verify/script/VerifyBatchInclusionCallerDeployer.s.sol \
    "$ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS" \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(address _targetContract)"

# Kill the anvil process to save state
pkill anvil
