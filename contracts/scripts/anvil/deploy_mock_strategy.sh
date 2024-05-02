#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/eigenlayer-deployed-anvil-state.json --dump-state state/strategy-deployed-anvil-state.json &

cd ../../

sleep 1

# Deploy the contracts
forge script script/deploy/MockStrategyDeployer.s.sol --rpc-url "http://localhost:8545" --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" --broadcast

# Kill the anvil process to save state
pkill anvil

# Anvil adds a block state, making the code to fail. We don't care about this, just the accounts and the deployed code
cd "$parent_path"

jq 'del(.block)' state/strategy-deployed-anvil-state.json > state/strategy-deployed-anvil-state-tmp.json

cp -f state/strategy-deployed-anvil-state-tmp.json state/strategy-deployed-anvil-state.json

rm state/strategy-deployed-anvil-state-tmp.json

cd ../../

# Extract the erc20MockStrategy value from strategy_deployment_output.json
erc20MockStrategy=$(jq -r '.erc20MockStrategy' "script/output/devnet/strategy_deployment_output.json")

# Use the extracted value to replace the 0_strategy value in aligned.devnet.config.json and save it to a temporary file
jq --arg erc20MockStrategy "$erc20MockStrategy" '.strategyWeights[0][0]."0_strategy" = $erc20MockStrategy' "script/deploy/config/devnet/aligned.devnet.config.json" | sed -r 's/1E\+([0-9]+)/1e+\1/g' > "script/deploy/config/devnet/aligned.devnet.config.temp.json"

# Replace the original file with the temporary file
mv "script/deploy/config/devnet/aligned.devnet.config.temp.json" "script/deploy/config/devnet/aligned.devnet.config.json"

# Delete the temporary file
rm -f "script/deploy/config/devnet/aligned.devnet.config.temp.json"
