#!/bin/bash

# Check if the number of arguments is correct
if [ "$#" -ne 2 ]; then
    echo "Usage: add_operator_to_whitelist.sh <ALIGNED_DEPLOYMENT_OUTPUT> <OPERATOR_ADDRESS>"
    exit 1
fi

JSON_PATH=$1
OPERATOR_ADDRESS=$2

# Read the registry coordinator address from the JSON file
REGISTRY_COORDINATOR=$(jq -r '.addresses.registryCoordinator' $JSON_PATH)

# Check if the registry coordinator address is empty
if [ -z "$REGISTRY_COORDINATOR" ]; then
    echo "Registry coordinator address is empty"
    exit 1
fi

# Check if the Ethereum RPC URL is empty
if [ -z "$ETH_RPC_URL" ]; then
    echo "Ethereum RPC URL is empty"
    exit 1
fi

# Check if the private key is empty
if [ -z "$ETH_PRIVATE_KEY" ]; then
    echo "Private key is empty"
    exit 1
fi

# Call the add function on the contract
cast send \
  --rpc-url=$ETH_RPC_URL \
  --private-key=$ETH_PRIVATE_KEY \
  $REGISTRY_COORDINATOR 'add(address)' \
  $OPERATOR_ADDRESS
