#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"

cd ../../

jq 'del(.block)' scripts/anvil/state/alignedlayer-deployed-anvil-state.json > scripts/anvil/state/alignedlayer-deployed-anvil-state-tmp.json

cp -f scripts/anvil/state/alignedlayer-deployed-anvil-state-tmp.json scripts/anvil/state/alignedlayer-deployed-anvil-state.json

rm scripts/anvil/state/alignedlayer-deployed-anvil-state-tmp.json

anvil --load-state scripts/anvil/state/alignedlayer-deployed-anvil-state.json --dump-state scripts/anvil/state/alignedlayer-deployed-anvil-state.json &

sleep 2

# Save the output to a variable to later extract the address of the new deployed contract
forge_output=$(forge script script/upgrade/AlignedLayerUpgradeDisabledVerifiers.s.sol \
    "./script/output/devnet/eigenlayer_deployment_output.json" \
    "./script/output/devnet/alignedlayer_deployment_output.json" \
    "./script/deploy/config/devnet/aligned.devnet.config.json" \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string memory eigenLayerDeploymentFilePath, string memory alignedLayerDeploymentFilePath, string memory alignedConfigFilePath)")

echo "$forge_output"

pkill anvil

# Extract the alignedLayerServiceManagerImplementation value from the output
new_aligned_layer_service_manager_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the alignedLayerServiceManagerImplementation value in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg new_aligned_layer_service_manager_implementation "$new_aligned_layer_service_manager_implementation" '.addresses.alignedLayerServiceManagerImplementation = $new_aligned_layer_service_manager_implementation' "script/output/devnet/alignedlayer_deployment_output.json" > "script/output/devnet/alignedlayer_deployment_output.temp.json"

mv "script/output/devnet/alignedlayer_deployment_output.temp.json" "script/output/devnet/alignedlayer_deployment_output.json"
rm -f "script/output/devnet/alignedlayer_deployment_output.temp.json"
