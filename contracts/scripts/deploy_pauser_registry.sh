#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

# Deploy the contracts
forge_output=$(forge script script/deploy/PauserRegistryDeployer.s.sol \
    $EXISTING_DEPLOYMENT_INFO_PATH \
    $DEPLOY_CONFIG_PATH \
    $OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --sig "run(string memory existingDeploymentInfoPath, string memory deployConfigPath, string memory outputPath)" \
    --slow)

echo "$forge_output"

# Extract the pauser registry and Pauser addresses values from the output

pauser_registry=$(echo "$forge_output" | awk '/0: address/ {print $3}')
pauser=$(echo "$forge_output" | awk '/1: address/ {print $3}')

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg pauser_registry "$pauser_registry" '.addresses.pauserRegistry = $pauser_registry' "script/output/holesky/alignedlayer_deployment_output.json" > "script/output/holesky/alignedlayer_deployment_output.temp.temp.json"
jq --arg pauser "$pauser" '.permissions.alignedLayerPauser = $pauser' "script/output/holesky/alignedlayer_deployment_output.temp.temp.json" > "script/output/holesky/alignedlayer_deployment_output.temp.json"


# Replace the original file with the temporary file
mv "script/output/holesky/alignedlayer_deployment_output.temp.json" "script/output/holesky/alignedlayer_deployment_output.json"

# Delete the temporary file
rm -f "script/output/holesky/alignedlayer_deployment_output.temp.json"
rm -f "script/output/holesky/alignedlayer_deployment_output.temp.temp.json"
