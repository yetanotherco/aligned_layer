#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ -z "$OUTPUT_PATH" ]; then
    echo "OUTPUT_PATH env var is not set"
    exit 1
fi

if  [ -z "$RPC_URL" ]; then
    echo "RPC_URL env var is not set"
    exit 1
fi

ALIGNED_SERVICE_MANAGER=$(jq -r '.addresses.alignedLayerServiceManager' "$OUTPUT_PATH")

## Using in this cast call:

# /**
#     * @notice Returns the list of strategies that the AVS supports for restaking
#     * @dev This function is intended to be called off-chain
#     * @dev No guarantee is made on uniqueness of each element in the returned array.
#     *      The off-chain service should do that validation separately
#     */
# function getRestakeableStrategies() external view returns (address[] memory) {

cast call $ALIGNED_SERVICE_MANAGER "getRestakeableStrategies()(address[])" --rpc-url $RPC_URL

# Expected output:
# [addresses]
# example:
# [0xc5a5C42992dECbae36851359345FE25997F5C42d, 0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9]
