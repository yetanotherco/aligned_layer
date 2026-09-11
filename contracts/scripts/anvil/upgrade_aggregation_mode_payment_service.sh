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

# Deploy new Aggregation Mode Payment Service implementation, but don't upgrade yet
forge_output=$(forge script script/upgrade/AggregationModePaymentServiceUpgrader.s.sol \
    "./script/output/devnet/proof_aggregation_service_deployment_output.json" \
    --rpc-url "http://localhost:8545" \
    --private-key "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356" \
    --broadcast \
    --legacy \
    --sig "run(string memory alignedLayerDeploymentFilePath)")

echo "$forge_output"

# Extract the aggregation mode payment service values from the output
aggregation_mode_payment_service_proxy=$(echo "$forge_output" | awk '/0: address/ {print $3}')
aggregation_mode_payment_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

data=$(cast calldata "upgradeTo(address)" $aggregation_mode_payment_service_implementation)

MULTISIG=false # hardcoding non-multisig for devnet
if [ "$MULTISIG" = false ]; then
  echo "Executing upgrade transaction"
  cast send $aggregation_mode_payment_service_proxy $data \
    --rpc-url "http://localhost:8545" \
    --private-key "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"
else
  echo "You can propose the upgrade transaction with the multisig using this calldata"
  echo $data
fi

pkill anvil

# Use the extracted value to replace the aggregation mode payment service values and save it to a temporary file
jq --arg aggregation_mode_payment_service_implementation "$aggregation_mode_payment_service_implementation" \
   '.addresses.aggregationModePaymentServiceImplementation = $aggregation_mode_payment_service_implementation' \
   "./script/output/devnet/proof_aggregation_service_deployment_output.json" > \
   "./script/output/devnet/proof_aggregation_service_deployment_output.temp.json"

# Replace the original file with the temporary file
mv "./script/output/devnet/proof_aggregation_service_deployment_output.temp.json" \
   "./script/output/devnet/proof_aggregation_service_deployment_output.json"

# Delete the temporary file
rm -f "./script/output/devnet/proof_aggregation_service_deployment_output.temp.json"
