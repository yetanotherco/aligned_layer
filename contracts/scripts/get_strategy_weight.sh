#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ "$#" -ne 1 ]; then
  echo "Error: 1 arguments is required, STRATEGY_INDEX"
  exit 1
fi

STRATEGY_INDEX=$1

if [ -z "$OUTPUT_PATH" ]; then
    echo "OUTPUT_PATH env var is not set"
    exit 1
fi

if  [ -z "$RPC_URL" ]; then
    echo "RPC_URL env var is not set"
    exit 1
fi

STAKE_REGISTRY=$(jq -r '.addresses.stakeRegistry' "$OUTPUT_PATH")

## Using in this cast call:

#     struct StrategyParams {
#         IStrategy strategy; (iface -> address)
#         uint96 multiplier;
#     }

#     /// @notice Returns the strategy and weight multiplier for the `index`'th strategy in the quorum `quorumNumber`
#     function strategyParamsByIndex(
#         uint8 quorumNumber, 
#         uint256 index
#     ) public view returns (StrategyParams memory)
#

QUORUM_NUMER=0x0 #Aligned has only 1 quorum for now

cast call $STAKE_REGISTRY "strategyParamsByIndex(uint8,uint256)((address,uint96))" $QUORUM_NUMER $STRATEGY_INDEX --rpc-url $RPC_URL

# Expected output:
# (strategy_address, multiplier)
# example:
# (0xc5a5C42992dECbae36851359345FE25997F5C42d, 1000000000000000000 [1e18])
