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

# Deploy Batcher Payments Contract
forge_output=$(forge script script/upgrade/BatcherPaymentServiceUpgradeAddTypeHash.s.sol \
    "./script/output/devnet/alignedlayer_deployment_output.json" \
    "./script/deploy/config/devnet/batcher-payment-service.devnet.config.json" \
    --rpc-url "http://localhost:8545" \
    --private-key "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356" \
    --broadcast \
    --legacy \
    --verify \
    --sig "run(string alignedLayerDeploymentFilePath,string batcherPaymentServiceConfigFilePath)")

echo "$forge_output"

pkill anvil

# Extract the batcher payment service values from the output
batcher_payment_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg batcher_payment_service_implementation "$batcher_payment_service_implementation" '.addresses.batcherPaymentServiceImplementation = $batcher_payment_service_implementation' "./script/output/devnet/alignedlayer_deployment_output.json" > "./script/output/devnet/alignedlayer_deployment_output.temp.json"

# Replace the original file with the temporary file
mv "./script/output/devnet/alignedlayer_deployment_output.temp.json" "./script/output/devnet/alignedlayer_deployment_output.json"

# Delete the temporary file
rm -f "./script/output/devnet/alignedlayer_deployment_output.temp.json"
