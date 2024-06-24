#!/bin/bash

# Check that BATCHER_ADDRESS is not empty
if [[ "$BATCHER_ADDRESS" -eq "" ]]; then
  echo "BATCHER_ADDRESS is empty, using default value 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
  BATCHER_ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
  BATCHER_PRIVATE_KEY=ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
fi;

aligned_service_manager_address=$(cat "contracts/script/output/devnet/alignedlayer_deployment_output.json" | jq -r '.addresses.alignedLayerServiceManager')

# Amount of ETH to send (in Ether)
amount_in_eth="100ether"

# Send Ether transaction
cast send --from $BATCHER_ADDRESS \
    --value $amount_in_eth \
    --private-key $BATCHER_PRIVATE_KEY \
    --rpc-url "http://localhost:8545" \
    "$aligned_service_manager_address"
