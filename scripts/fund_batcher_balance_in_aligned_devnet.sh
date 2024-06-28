#!/bin/bash

cd contracts

BATCHER_ADDRESS=$(jq -r '.address.batcherWallet' ./script/deploy/config/devnet/batcher-payments.devnet.config.json)
BATCHER_PRIVATE_KEY=$(jq -r '.address.batcherPrivateKey' ./script/deploy/config/devnet/batcher-payments.devnet.config.json)
alignedLayerServiceManager=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

# Amount of ETH to send (in Ether)
amount_in_eth="100ether"

# Send Ether transaction
cast send --from $BATCHER_ADDRESS \
    --value $amount_in_eth \
    --private-key $BATCHER_PRIVATE_KEY \
    --rpc-url "http://localhost:8545" \
    "$alignedLayerServiceManager"
