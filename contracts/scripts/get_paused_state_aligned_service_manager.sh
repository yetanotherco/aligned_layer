#!/bin/bash

if  [ -z "$ALIGNED_SERVICE_MANAGER" ]; then
    echo "ALIGNED_SERVICE_MANAGER env var is not set"
    exit 1
fi

if  [ -z "$PRIVATE_KEY" ]; then
    echo "PRIVATE_KEY env var is not set"
    exit 1
fi

if  [ -z "$RPC_URL" ]; then
    echo "RPC_URL env var is not set"
    exit 1
fi

number=$(cast call $ALIGNED_SERVICE_MANAGER "paused()()" --rpc-url $RPC_URL)
number=$((number))

echo Aligned Paused state: $number,

echo Aligned paused functions:

bit_position=0
while [ $number -gt 0 ]; do
    if [ $((number & 1)) -eq 1 ]; then
        echo $bit_position
    fi
    number=$((number >> 1))
    bit_position=$((bit_position + 1))
done
