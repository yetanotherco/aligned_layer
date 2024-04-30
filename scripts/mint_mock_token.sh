#!/bin/bash

# check that OPERATOR_ADDRESS is not empty
if [[ "$OPERATOR_ADDRESS" -eq "" ]]; then
  echo "OPERATOR_ADDRESS is empty, using default value 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
  OPERATOR_ADDRESS="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
fi;

# Check first argument
if [[ "$1" -eq "" ]]; then
  echo "Please provide the amount of tokens to mint"
  exit 1
fi;

# Get mock token address from deployment output using jq
mock_token_address=$(cat "contracts/script/output/devnet/strategy_deployment_output.json" | jq -r '.erc20Mock')

if [[ "$mock_token_address" -eq "" ]]; then
  echo "Mock token address is empty, please deploy the contracts first"
  exit 1
fi;

echo "Minting $1 tokens to $OPERATOR_ADDRESS"
echo "Mock token address: $mock_token_address"

# Ethereum sender address - anvil address 1
private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Mint tokens
cast send "$mock_token_address" \
    "mint(address, uint256)" \
    $OPERATOR_ADDRESS "$1" \
    --private-key $private_key \
    --rpc-url "http://localhost:8545"

