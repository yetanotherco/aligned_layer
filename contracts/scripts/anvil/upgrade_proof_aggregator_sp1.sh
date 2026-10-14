#!/bin/bash

# Updates the SP1 setup of the Aggregation Mode in the devnet anvil states:
# - Deploys the SP1 Groth16 verifier from SP1VerifierGroth16Deployer.s.sol and adds its route to the SP1 verifier gateway
#   (in both the sp1 and the alignedlayer states)
# - Allows the SP1 chunk aggregator program of programs_ids.json in the AlignedProofAggregationService
#   (in the alignedlayer state)

set -e

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$parent_path"

# cd to /contracts
cd ../../

export CHAINS='DEVNET'
export RPC_DEVNET='http://localhost:8545'

# Anvil account #2, owner of the SP1 verifier gateway
export VERIFIER_PRIVATE_KEY="0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a"
# Anvil account #7, owner of the AlignedProofAggregationService
OWNER_PRIVATE_KEY="0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"

SP1_PROVING_SYSTEM_ID=1
SP1_CHUNK_AGGREGATOR_VK_HASH=$(jq -r '.sp1_chunk_aggregator_vk_hash' ../aggregation_mode/proof_aggregator/programs_ids.json)
PROOF_AGGREGATOR_ADDRESS=$(jq -r '.addresses.alignedProofAggregationService' ./script/output/devnet/proof_aggregation_service_deployment_output.json)

for state in sp1-deployed-anvil-state.json alignedlayer-deployed-anvil-state.json; do
    state_path="scripts/anvil/state/$state"

    jq 'del(.block)' "$state_path" > "$state_path.tmp"
    mv "$state_path.tmp" "$state_path"

    anvil --load-state "$state_path" --dump-state "$state_path" &
    anvil_pid=$!
    sleep 2

    forge script ./script/deploy/SP1VerifierGroth16Deployer.s.sol:SP1VerifierScript \
        --rpc-url $RPC_DEVNET \
        --private-key $VERIFIER_PRIVATE_KEY \
        --broadcast

    if [ "$state" = "alignedlayer-deployed-anvil-state.json" ]; then
        cast send $PROOF_AGGREGATOR_ADDRESS \
            "allowVerifyingProgram(bytes32,uint8)" $SP1_CHUNK_AGGREGATOR_VK_HASH $SP1_PROVING_SYSTEM_ID \
            --rpc-url $RPC_DEVNET \
            --private-key $OWNER_PRIVATE_KEY
    fi

    # Stop anvil so it dumps its state
    kill $anvil_pid
    wait $anvil_pid || true
done
