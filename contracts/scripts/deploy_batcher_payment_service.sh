#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

source scripts/.env

# Deploy Batcher Payments Contract
forge script script/deploy/BatcherPaymentServiceDeployer.s.sol \
    $BATCHER_PAYMENT_SERVICE_CONFIG_PATH \
    $BATCHER_PAYMENT_SERVICE_OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --legacy \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory batcherConfigPath, string memory outputPath)"

# Extract the batcher payment service values from the output
batcher_payment_service_proxy=$(jq -r '.addresses.batcherPaymentService' $BATCHER_PAYMENT_SERVICE_OUTPUT_PATH)
batcher_payment_service_implementation=$(jq -r '.addresses.batcherPaymentServiceImplementation' $BATCHER_PAYMENT_SERVICE_OUTPUT_PATH)

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg batcher_payment_service_proxy "$batcher_payment_service_proxy" '.addresses.batcherPaymentService = $batcher_payment_service_proxy' $OUTPUT_PATH > $OUTPUT_PATH.temp2
jq --arg batcher_payment_service_implementation "$batcher_payment_service_implementation" '.addresses.batcherPaymentServiceImplementation = $batcher_payment_service_implementation' $OUTPUT_PATH.temp2 > $OUTPUT_PATH.temp

# Replace the original file with the temporary file
mv $OUTPUT_PATH.temp $OUTPUT_PATH

# Delete the temporary file
rm -f $OUTPUT_PATH.temp
rm -f $OUTPUT_PATH.temp2
