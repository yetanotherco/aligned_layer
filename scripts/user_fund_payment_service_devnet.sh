#!/bin/bash

# Check that OPERATOR_ADDRESS is not empty
if [[ "$USER_ADDRESS" -eq "" ]]; then
  echo "USER_ADDRESS is empty, using default value 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
  USER_ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
  USER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
fi;

cd contracts

batcher_payment_service_address=$(jq -r '.addresses.batcherPaymentService' ./script/output/devnet/alignedlayer_deployment_output.json)

# Amount of ETH to send (in Ether)
amount_in_eth="100ether"

# Send Ether transaction
cast send --from $USER_ADDRESS \
    --value $amount_in_eth \
    --private-key $USER_PRIVATE_KEY \
    --rpc-url "http://localhost:8545" \
    "$batcher_payment_service_address" \
    --gas-price $(cast gas-price --rpc-url "http://localhost:8545")
