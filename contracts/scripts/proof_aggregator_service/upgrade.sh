#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"

cd ../

# Save the output to a variable to later extract the address of the new deployed contract
forge_output=$(forge script script/upgrade/ProofAggregatorServiceUpgrader.s.sol \
    $EXISTING_DEPLOYMENT_INFO_PATH \
    $OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory alignedLayerDeploymentFilePath)")

echo "$forge_output"

# Extract the proof aggregator service values from the output
proof_aggregator_service_proxy=$(echo "$forge_output" | awk '/0: address/ {print $3}')
proof_aggregator_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg  proof_aggregator_service_implementation "$proof_aggregator_service_implementation" '.addresses.alignedProofAggregationServiceImplementation = $proof_aggregator_service_implementation' $OUTPUT_PATH > "$OUTPUT_PATH.temp"

# Replace the original file with the temporary file
mv "$OUTPUT_PATH.temp" $OUTPUT_PATH

# Delete the temporary file
rm -f "$OUTPUT_PATH.temp"

data=$(cast calldata "upgradeTo(address)" $batcher_payment_service_implementation)

echo "The new Proof Aggregator Service Implementation is $batcher_payment_service_implementation"
