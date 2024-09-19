#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"

cd ../../contracts

# Save the output to a variable to later extract the address of the new deployed contract
forge_output=$(forge script script/upgrade/AlignedLayerUpgradeAddAggregator.s.sol \
    $EXISTING_DEPLOYMENT_INFO_PATH \
    $OUTPUT_PATH \
    $DEPLOY_CONFIG_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory eigenLayerDeploymentFilePath, string memory alignedLayerDeploymentFilePath, string memory alignedConfigFilePath)")

echo "$forge_output"

# Extract the alignedLayerServiceManagerImplementation value from the output
new_aligned_layer_service_manager_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the alignedLayerServiceManagerImplementation value in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg new_aligned_layer_service_manager_implementation "$new_aligned_layer_service_manager_implementation" '.addresses.alignedLayerServiceManagerImplementation = $new_aligned_layer_service_manager_implementation' $OUTPUT_PATH > "script/output/holesky/alignedlayer_deployment_output.temp.json"

# Write aggregator addres to deployment output file
ALIGNED_LAYER_AGGREGATOR_ADDRESS=$(jq -r '.permissions.aggregator' $DEPLOY_CONFIG_PATH)
jq --arg alignedLayerAggregator "$ALIGNED_LAYER_AGGREGATOR_ADDRESS" '.permissions += {"alignedLayerAggregator": $alignedLayerAggregator}' "script/output/holesky/alignedlayer_deployment_output.temp.json" > "script/output/holesky/alignedlayer_deployment_output.temp2.json"

# Replace the original file with the temporary file
mv "script/output/holesky/alignedlayer_deployment_output.temp2.json" $OUTPUT_PATH

# Delete the temporary file
rm -f "script/output/holesky/alignedlayer_deployment_output.temp.json"
