#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

# Check if the number of arguments is correct
if [ "$#" -ne 1 ]; then
    echo "Usage: add_operator_to_whitelist.sh <OPERATOR_ADDRESS>"
    exit 1
fi

OPERATOR_ADDRESS=$1

# Read the registry coordinator address from the JSON file
REGISTRY_COORDINATOR=$(jq -r '.addresses.registryCoordinator' "$OUTPUT_PATH")

# Check if the registry coordinator address is empty
if [ -z "$REGISTRY_COORDINATOR" ]; then
    echo "Registry coordinator address is empty"
    exit 1
fi

# Check if the Ethereum RPC URL is empty
if [ -z "$RPC_URL" ]; then
    echo "Ethereum RPC URL is empty"
    exit 1
fi

# Check if the private key is empty
if [ -z "$PRIVATE_KEY" ]; then
    echo "Private key is empty"
    exit 1
fi

# Call the add function on the contract
cast send \
  --rpc-url=$RPC_URL \
  --private-key=$PRIVATE_KEY \
  $REGISTRY_COORDINATOR 'remove(address)' \
  $OPERATOR_ADDRESS
