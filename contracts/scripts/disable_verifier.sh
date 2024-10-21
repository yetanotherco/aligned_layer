#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

# Check if the number of arguments is correct
if [ "$#" -ne 1 ]; then
    echo "Usage: disable_verifier.sh <VERIFIER_ID>"
    exit 1
fi

VERIFIER_ID=$1

# Read the service manager address from the JSON file
SERVICE_MANAGER=$(jq -r '.addresses.alignedLayerServiceManager' "$OUTPUT_PATH")

# Check if the servide manager address is empty
if [ -z "$SERVICE_MANAGER" ]; then
    echo "Service manager address is empty"
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

# Call the disableVerifier function on the contract
cast send \
    --private-key=$PRIVATE_KEY \
    --rpc-url=$RPC_URL \
    $SERVICE_MANAGER "disableVerifier(uint8)" \
    $VERIFIER_ID
