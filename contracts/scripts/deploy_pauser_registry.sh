#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

# Deploy the contracts
forge script script/deploy/PauserRegistryDeployer.s.sol \
    $EXISTING_DEPLOYMENT_INFO_PATH \
    $DEPLOY_CONFIG_PATH \
    $OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --sig "run(string memory existingDeploymentInfoPath, string memory deployConfigPath, string memory outputPath)" \
    --slow
