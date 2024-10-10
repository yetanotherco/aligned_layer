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

# Deploy new Batcher Payments implementation, but don't upgrade yet 
forge_output=$(forge script script/upgrade/BatcherPaymentServiceUpgrader.s.sol \
    "./script/output/devnet/alignedlayer_deployment_output.json" \
    --rpc-url "http://localhost:8545" \
    --private-key "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356" \
    --broadcast \
    --legacy \
    --verify \
    --sig "run(string batcherConfigPath)")

echo "$forge_output"

data=$(cast calldata "upgradeToAndCall(address, bytes)" $batcher_payment_service_implementation "0x")

# Extract the batcher payment service values from the output
batcher_payment_service_proxy=$(echo "$forge_output" | awk '/0: address/ {print $3}')
batcher_payment_service_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

MULTISIG=false # hardcoding non-multisig for devnet.
if [ "$MULTISIG" = false ]; then
  echo "Executing upgrade transaction"
  cast send $batcher_payment_service_proxy $data \
    --rpc-url "http://localhost:8545" \
    --private-key "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"
else
  echo "You can propose the upgrade transaction with the multisig using this calldata"
  echo $data
fi

pkill anvil

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg batcher_payment_service_implementation "$batcher_payment_service_implementation" '.addresses.batcherPaymentServiceImplementation = $batcher_payment_service_implementation' "./script/output/devnet/alignedlayer_deployment_output.json" > "./script/output/devnet/alignedlayer_deployment_output.temp.json"

# Replace the original file with the temporary file
mv "./script/output/devnet/alignedlayer_deployment_output.temp.json" "./script/output/devnet/alignedlayer_deployment_output.json"

# Delete the temporary file
rm -f "./script/output/devnet/alignedlayer_deployment_output.temp.json"
