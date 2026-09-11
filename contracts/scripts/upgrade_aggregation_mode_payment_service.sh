#!/bin/bash

# ENV VARIABLES
#
# MULTISIG=true|false whether the contract is deployed under a multisig account
#
# PROOF_AGGREGATOR_OUTPUT_PATH: Path to the proof aggregator output file
#   - Holesky Stage: ./script/output/holesky/proof_aggregation_service_deployment_output.stage.json
#   - Holesky Prod: ./script/output/holesky/proof_aggregation_service_deployment_output.json
#
# RPC_URL: The RPC URL to connect to the Ethereum network
#
# PRIVATE_KEY: The private key to use for the deployment
#
# ETHERSCAN_API_KEY: The Etherscan API key to use for verification
#

if [ -z "$MULTISIG" ]; then
  echo "Missing MULTISIG env variable"
  exit 1
fi

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"

cd ../

# Save the output to a variable to later extract the address of the new deployed contract
forge_output=$(forge script script/upgrade/AggregationModePaymentServiceUpgrader.s.sol \
    $PROOF_AGGREGATOR_OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory alignedLayerDeploymentFilePath)")

echo "$forge_output"

# Extract the aggregation mode payment service values from the output
aggregation_mode_payment_service_proxy=$(echo "$forge_output" | awk '/0: address/ {print $3}')
aggregation_mode_payment_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the aggregation mode payment service values and save it to a temporary file
jq --arg aggregation_mode_payment_service_implementation "$aggregation_mode_payment_service_implementation" \
   '.addresses.aggregationModePaymentServiceImplementation = $aggregation_mode_payment_service_implementation' \
   $PROOF_AGGREGATOR_OUTPUT_PATH > "$PROOF_AGGREGATOR_OUTPUT_PATH.temp"

# Replace the original file with the temporary file
mv "$PROOF_AGGREGATOR_OUTPUT_PATH.temp" $PROOF_AGGREGATOR_OUTPUT_PATH

# Delete the temporary file
rm -f "$PROOF_AGGREGATOR_OUTPUT_PATH.temp"

echo "The new Aggregation Mode Payment Service Implementation is $aggregation_mode_payment_service_implementation"

data=$(cast calldata "upgradeTo(address)" $aggregation_mode_payment_service_implementation)

if [ "$MULTISIG" = false ]; then
  echo "Executing upgrade transaction"
  cast send $aggregation_mode_payment_service_proxy $data \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY
else
  echo "You can propose the upgrade transaction with the multisig using this calldata"
  echo $data
fi
