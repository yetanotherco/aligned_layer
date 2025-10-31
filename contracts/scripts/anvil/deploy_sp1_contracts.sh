#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start an empty anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/risc0-deployed-anvil-state.json --dump-state state/sp1-deployed-anvil-state.json &

# cd to /contracts
cd ../../

sleep 1

export CHAINS='DEVNET'
export RPC_DEVNET='http://localhost:8545'

# Anvil account #2
export OWNER='0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC'
export VERIFIER_PRIVATE_KEY="0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a"

# Deploy Groth16 SP1 verifier gateway
forge script script/deploy/SP1VerifierGatewayGroth16Deployer.s.sol:SP1VerifierGatewayScript \
    --rpc-url $RPC_DEVNET \
    --private-key $VERIFIER_PRIVATE_KEY \
    --broadcast

# Deploy Groth16 SP1 verifier
forge script ./script/deploy/SP1VerifierGroth16Deployer.s.sol:SP1VerifierScript \
    --rpc-url $RPC_DEVNET \
    --private-key $VERIFIER_PRIVATE_KEY \
    --broadcast


# Kill the anvil process to save state
pkill anvil
