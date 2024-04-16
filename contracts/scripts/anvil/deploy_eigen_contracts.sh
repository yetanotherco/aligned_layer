#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --dump-state state/eigenlayer-deployed-anvil-state.json &

cd ../../lib/eigenlayer-middleware/lib/eigenlayer-contracts

# Backup contract output
mv script/output/devnet/M2_from_scratch_deployment_data.json script/output/devnet/M2_from_scratch_deployment_data.json.bak

sleep 1

# Deploy the contracts
forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url "http://localhost:8545" --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json

# Move the output to the expected location & restore backup
mv script/output/devnet/M2_from_scratch_deployment_data.json ../../../../script/output/devnet/eigenlayer_deployment_output.json
mv script/output/devnet/M2_from_scratch_deployment_data.json.bak script/output/devnet/M2_from_scratch_deployment_data.json

# Kill the anvil process to save state
pkill anvil

# Anvil adds a block state, making the code to fail. We don't care about this, just the accounts and the deployed code
cd "$parent_path"

jq 'del(.block)' state/eigenlayer-deployed-anvil-state.json > state/eigenlayer-deployed-anvil-state-tmp.json

cp -f state/eigenlayer-deployed-anvil-state-tmp.json state/eigenlayer-deployed-anvil-state.json

rm state/eigenlayer-deployed-anvil-state-tmp.json
