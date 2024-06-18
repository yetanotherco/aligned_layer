#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1 ; pwd -P )

# At this point we are in examples/verify/scripts
cd "$parent_path" || exit 1

# At this point we are in examples/verify dir
cd ../ || exit 1

source .env

ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' "$ALIGNED_DEPLOYMENT_OUTPUT")

forge script script/VerifyBatchInclusionCallerDeployer.s.sol \
    "$ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS" \
    --rpc-url "$RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    --broadcast \
    --slow \
    --legacy \
    --sig "run(address _targetContract)"
