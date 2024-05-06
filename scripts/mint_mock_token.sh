#!/bin/bash

# check that OPERATOR_ADDRESS is not empty
if [ "$OPERATOR_ADDRESS" = "" ]; then
  echo "OPERATOR_ADDRESS is empty, using default value 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
  OPERATOR_ADDRESS="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
fi;

# Check that the script received 2 arguments
if [ "$#" != 2 ]; then
  echo "Usage: $0 <config_file> <amount>"
  exit 1
fi;

# Get mock token address from deployment output using jq
mock_token_address=$(cat "contracts/script/output/devnet/strategy_deployment_output.json" | jq -r '.erc20Mock')
operator_address=$(cat "$1" | yq -r '.operator.address')

if [[ "$mock_token_address" -eq "" ]]; then
  echo "Mock token address is empty, please deploy the contracts first"
  exit 1
fi;

echo "Minting $2 tokens to $operator_address"
echo "Mock token address: $mock_token_address"
echo "Amount: $2"

# Ethereum sender address - anvil address 1
private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Mint tokens
cast send "$mock_token_address" \
    "mint(address, uint256)" \
    "$operator_address" "$2" \
    --private-key $private_key \
    --rpc-url "http://localhost:8545"

echo Tokens minted