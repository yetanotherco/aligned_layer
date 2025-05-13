#!/bin/bash

RPC_URL="http://localhost:8545"
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts/anvil/
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --dump-state state/eigenlayer-deployed-anvil-state.json &

cd ../../
# At this point we are in contracts/

cd eigenlayer_contracts/eigenlayer-contracts

sleep 1

# Deploy the contracts
forge script script/deploy/local/deploy_from_scratch.slashing.s.sol \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --sig "run(string memory configFile)" -- local/deploy_from_scratch.slashing.anvil.config.json

# Whitelist strategy into strategyManager
strategy_manager=$(jq -r '.addresses.strategyManager' script/output/devnet/SLASHING_deploy_from_scratch_deployment_data.json)
strategy=$(jq -r '.addresses.strategy' script/output/devnet/SLASHING_deploy_from_scratch_deployment_data.json)
echo "Whitelisting strategy ($strategy) into strategy manager ($strategy_manager)"
cast send "$strategy_manager" \
  "addStrategiesToDepositWhitelist(address[])" "[$strategy]" \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \

# Copy the deployment data to Aligned output directory
cp script/output/devnet/SLASHING_deploy_from_scratch_deployment_data.json ../../script/output/devnet/eigenlayer_deployment_output.json

# Restore the submodule repository
git restore script/output/devnet/SLASHING_deploy_from_scratch_deployment_data.json

# Kill the anvil process to save state
pkill anvil

# Anvil adds a block state, making the code to fail. We don't care about this, just the accounts and the deployed code
cd "$parent_path"

jq 'del(.block)' state/eigenlayer-deployed-anvil-state.json > state/eigenlayer-deployed-anvil-state-tmp.json

cp -f state/eigenlayer-deployed-anvil-state-tmp.json state/eigenlayer-deployed-anvil-state.json

rm state/eigenlayer-deployed-anvil-state-tmp.json
