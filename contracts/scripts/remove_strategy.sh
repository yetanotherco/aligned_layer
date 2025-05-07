#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ "$#" -ne 1 ]; then
  echo "Error: 1 arguments is required, INDICES_TO_REMOVE"
  exit 1
fi

INDICES_TO_REMOVE=$1

if [[ ! "$INDICES_TO_REMOVE" =~ ^\[[0-9]+(,[0-9]+)*\]$ ]]; then
  echo "The INDICES_TO_REMOVE doesn't match the required format: [0,1,...,n]"
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
    #  * @notice This function is used for removing strategies and their associated weights from the
    #  * mapping strategyParams for a specific @param quorumNumber.
    #  * @dev higher indices should be *first* in the list of @param indicesToRemove, since otherwise
    #  * the removal of lower index entries will cause a shift in the indices of the other strategiesToRemove
    #  */
    # function removeStrategies(uint8 quorumNumber, uint256[] calldata indicesToRemove) external;

QUORUM_NUMBER=0 #Aligned has only 1 quorum for now

data=$(cast calldata "removeStrategies(uint8, uint256[])()" $QUORUM_NUMBER $INDICES_TO_REMOVE)

if [ "$MULTISIG" = false ]; then
  echo "Executing remove strategies transaction"

  cast send $STAKE_REGISTRY $data \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY
else
  echo "You can propose the remove strategies transaction with the multisig using this calldata:"
  echo $data
fi
