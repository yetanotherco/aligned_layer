#!/bin/bash

if [ -z "$MULTISIG" ]; then
  echo "Missing MULTISIG env variable"
  exit 1
fi

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

cd "$parent_path"

cd ../

# Save the output to a variable to later extract the address of the new deployed contract
forge_output=$(forge script script/upgrade/RegistryCoordinatorUpgrader.s.sol \
    $EXISTING_DEPLOYMENT_INFO_PATH \
    $OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory eigenLayerDeploymentFilePath, string memory alignedLayerDeploymentFilePath, )")

echo "$forge_output"

# Extract the alignedLayerServiceManagerImplementation value from the output
registry_coordinator=$(echo "$forge_output" | awk '/0: address/ {print $3}')
new_registry_coordinator_implementation=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the alignedLayerServiceManagerImplementation value in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg new_registry_coordinator_implementation "$new_registry_coordinator_implementation" '.addresses.registryCoordinatorImplementation = $new_registry_coordinator_implementation' $OUTPUT_PATH > "script/output/holesky/alignedlayer_deployment_output.temp.json"

# Replace the original file with the temporary file
mv "script/output/holesky/alignedlayer_deployment_output.temp.json" $OUTPUT_PATH

# Delete the temporary file
rm -f "script/output/holesky/alignedlayer_deployment_output.temp.json"

data=$(cast calldata "upgrade(address, address)" $registry_coordinator $new_registry_coordinator_implementation)

if [ "$MULTISIG" = false ]; then
  echo "Executing upgrade transaction"
  proxy_admin=$(jq -r '.addresses.alignedLayerProxyAdmin' $OUTPUT_PATH)
  cast send $batcher_payment_service_proxy $data \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY
else
  echo "You can propose the upgrade transaction with the multisig using this calldata"
  echo $data
fi
