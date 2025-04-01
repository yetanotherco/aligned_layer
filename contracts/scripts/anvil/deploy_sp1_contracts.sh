#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --dump-state state/sp1-deployed-anvil-state.json &

# cd to /contracts
cd ../../

sleep 1

export CHAINS='TESTNET'
export RPC_TESTNET='http://localhost:8545'
export OWNER='0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC'

# Deploy Groth16 SP1 verifier gateway
forge script script/deploy/SP1VerifierGatewayGroth16Deployer.s.sol:SP1VerifierGatewayScript \
    --rpc-url "http://localhost:8545" \
    --private-key "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a" \
    --broadcast

# # Deploy Groth16 SP1 verifier
forge script ./script/deploy/SP1VerifierGroth16Deployer.s.sol:SP1VerifierScript \
    --rpc-url "http://localhost:8545" \
    --private-key "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a" \
    --broadcast 


# Kill the anvil process to save state
pkill anvil

# Anvil adds a block state, making the code to fail. We don't care about this, just the accounts and the deployed code
cd "$parent_path"

jq 'del(.block)' state/sp1-deployed-anvil-state.json > state/sp1-deployed-anvil-state-tmp.json

cp -f state/sp1-deployed-anvil-state-tmp.json state/sp1-deployed-anvil-state.json

rm state/sp1-deployed-anvil-state-tmp.json
