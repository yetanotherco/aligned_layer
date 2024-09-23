#!/bin/bash

if  [ -z "$BATCHER_PAYMENT_SERVICE" ]; then
    echo "BATCHER_PAYMENT_SERVICE env var is not set"
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

echo "Pausing batcher payment contract"
cast send $BATCHER_PAYMENT_SERVICE \
    "pause()()" \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY
