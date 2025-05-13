#!/bin/bash

# check that OPERATOR_ADDRESS is not empty
if [[ -z "$OPERATOR_ADDRESS" ]]; then
  echo "OPERATOR_ADDRESS is empty, using default value 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
  OPERATOR_ADDRESS="0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
fi;

# Check that the script received 2 arguments
if [[ "$#" -ne 2 ]]; then
  echo "Usage: $0 <config_file> <amount>"
  exit 1
fi;

mock_strategy_address=$(cat "contracts/script/output/devnet/eigenlayer_deployment_output.json" | jq -r '.addresses.strategies.WETH')
mock_token_address=$(cast call "$mock_strategy_address" "underlyingToken()")

operator_address=$(cat "$1" | yq -r '.operator.address')

if [[ -z  "$mock_token_address" ]]; then
  echo "Mock token address is empty, please deploy the contracts first"
  exit 1
fi;


# Remove 0x prefix from mock token address
mock_token_address=$(echo "$mock_token_address" | sed 's/^0x//')

stripped=$(echo "$mock_token_address" | sed 's/^0*//')

# Add back a single leading zero if the original string had any leading zeros
if [[ "$mock_token_address" =~ ^0+ ]]; then
    mock_token_address="0$stripped"
else
    mock_token_address="$stripped"
fi

echo "Minting $2 tokens to $operator_address"
echo "Mock token address: $mock_token_address"

# Ethereum sender address - anvil address 1
private_key="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# Mint tokens
# The deployment `contracts/eigenlayer_contracts/eigenlayer-contracts/script/deploy/local/deploy_from_scratch.slashing.s.sol`
# send tokens to `executorMultisig` which is `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266` (Anvil address 1)
# We need to send tokens from `executorMultisig` to `operator_address`
cast send "$mock_token_address" \
    "transfer(address recipient, uint256 amount)(bool)" \
    "$operator_address" "$2" \
    --private-key $private_key \
    --rpc-url "http://localhost:8545"
