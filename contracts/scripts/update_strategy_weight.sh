#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ "$#" -ne 2 ]; then
  echo "Error: 2 arguments are required, STRATEGY_INDICES and NEW_MULTIPLIERS"
  exit 1
fi

STRATEGY_INDICES=$1
NEW_MULTIPLIERS=$2


if [[ ! "$STRATEGY_INDICES" =~ ^\[[0-9]+(,[0-9]+)*\]$ ]]; then
  echo "The STRATEGY_INDICES doesn't match the required format: [0,1,...,n]"
  exit 1
fi

if [ -z "$NEW_MULTIPLIERS" ]; then
    echo "NEW_MULTIPLIERS env var is not set"
    exit 1
fi
if [[ ! "$NEW_MULTIPLIERS" =~ ^\[[0-9]+(,[0-9]+)*\]$ ]]; then
  echo "The NEW_MULTIPLIERS doesn't match the required format: [0,1,...,n]"
  exit 1
fi

count_elements() {
  local var="$1"
  # Remove brackets and count elements by splitting on commas
  echo "$var" | sed 's/[\[\]]//g' | awk -F',' '{print NF}'
}
count1=$(count_elements "$STRATEGY_INDICES")
count2=$(count_elements "$NEW_MULTIPLIERS")


if [[ $count1 -ne $count2 ]]; then
    echo "STRATEGY_INDICES and NEW_MULTIPLIERS have different numbers of elements:"
    echo "STRATEGY_INDICES: $STRATEGY_INDICES"
    echo "NEW_MULTIPLIERS: $NEW_MULTIPLIERS"
    exit 1
fi


if [ -z "$MULTISIG" ]; then
    echo "MULTISIG env var is not set"
    exit 1
fi
if [ "$MULTISIG" = false ]; then
  if [ -z "$PRIVATE_KEY" ]; then
      echo "PRIVATE_KEY env var is not set"
      exit 1
  fi
  if  [ -z "$RPC_URL" ]; then
    echo "RPC_URL env var is not set"
    exit 1
  fi
  if [ -z "$OUTPUT_PATH" ]; then
    echo "OUTPUT_PATH env var is not set"
    exit 1
  fi
  STAKE_REGISTRY=$(jq -r '.addresses.stakeRegistry' "$OUTPUT_PATH")
fi


## Using in this cast call:

# /**
#  * @notice This function is used for modifying the weights of strategies that are already in the
#  * mapping strategyParams for a specific
#  * @param quorumNumber is the quorum number to change the strategy for
#  * @param strategyIndices are the indices of the strategies to change
#  * @param newMultipliers are the new multipliers for the strategies
#  */
# function modifyStrategyParams(
#     uint8 quorumNumber,
#     uint256[] calldata strategyIndices,
#     uint96[] calldata newMultipliers
# ) external;

QUORUM_NUMBER=0 #Aligned has only 1 quorum for now

data=$(cast calldata "modifyStrategyParams(uint8, uint256[], uint96[])()" $QUORUM_NUMBER $STRATEGY_INDICES $NEW_MULTIPLIERS)

if [ "$MULTISIG" = false ]; then
  echo "Executing modify strategy params transaction"

  cast send $STAKE_REGISTRY $data \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY
else
  echo "You can propose the modify strategy params transaction with the multisig using this calldata:"
  echo $data
fi
