#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/eigenlayer-deployed-anvil-state.json --dump-state state/risc0-deployed-anvil-state.json &

# cd to /contracts
cd ../../

sleep 1

export RPC_DEVNET='http://localhost:8545'

# Anvil account #2
export DEPLOYER_PRIVATE_KEY='0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a'

forge script script/deploy/Risc0Groth16VerifierDeployer.s.sol:Risc0VerifierRouterDeployer \
    --rpc-url $RPC_DEVNET \
    --private-key $DEPLOYER_PRIVATE_KEY \
    --broadcast \
    --via-ir

# Kill the anvil process to save state
pkill anvil
