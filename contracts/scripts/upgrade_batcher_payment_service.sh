#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

source scripts/.env

# Deploy Batcher Payments Contract
forge_output=$(forge script script/upgrade/BatcherPaymentServiceUpgrader.s.sol \
    $OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --legacy \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string batcherConfigPath)")

echo "$forge_output"

# Extract the batcher payment service values from the output
batcher_payment_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg batcher_payment_service_implementation "$batcher_payment_service_implementation" '.addresses.batcherPaymentServiceImplementation = $batcher_payment_service_implementation' $OUTPUT_PATH > "$OUTPUT_PATH.temp"

# Replace the original file with the temporary file
mv "$OUTPUT_PATH.temp" $OUTPUT_PATH

# Delete the temporary file
rm -f "$OUTPUT_PATH.temp"


