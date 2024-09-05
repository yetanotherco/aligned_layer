#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1 ; pwd -P )

# At this point we are in examples/zkquiz
cd "$parent_path" || exit 1

if [ -z "$ALIGNED_SERVICE_MANAGER_ADDRESS" ]; then
    echo "ALIGNED_SERVICE_MANAGER_ADDRESS is not set. Please set it in .env"
    exit 1
fi

if [ -z "$BATCHER_PAYMENT_SERVICE_ADDRESS" ]; then
    echo "BATCHER_PAYMENT_SERVICE_ADDRESS is not set. Please set it in .env"
    exit 1
fi

if [ -z "$RPC_URL" ]; then
    echo "RPC_URL is not set. Please set it in .env"
    exit 1
fi

if [ -z "$PRIVATE_KEY" ]; then
    echo "PRIVATE_KEY is not set. Please set it in .env"
    exit 1
fi

forge install

forge script script/Deployer.s.sol \
    "$ALIGNED_SERVICE_MANAGER_ADDRESS" "$BATCHER_PAYMENT_SERVICE_ADDRESS" \
    --rpc-url "$RPC_URL" \
    --private-key "$PRIVATE_KEY" \
    --broadcast \
    --sig "run(address _alignedServiceManager, address _paymentService)"
