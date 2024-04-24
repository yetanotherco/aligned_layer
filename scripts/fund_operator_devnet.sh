#!/bin/bash

# Check that OPERATOR_ADDRESS is not empty
if [[ "$OPERATOR_ADDRESS" -eq "" ]]; then
  echo "OPERATOR_ADDRESS is empty"
  exit 1
fi;

# Ethereum sender address - anvil address 1
sender_address="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"

# Ethereum sender private key - anvil private key 1
sender_private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Recipient address
recipient_address=$OPERATOR_ADDRESS

# Amount of ETH to send (in Ether)
amount_in_eth="1ether"

# Send Ether transaction
cast send --from $sender_address \
    --value $amount_in_eth \
    --private-key $sender_private_key \
    --rpc-url "http://localhost:8545" \
    "$recipient_address"
