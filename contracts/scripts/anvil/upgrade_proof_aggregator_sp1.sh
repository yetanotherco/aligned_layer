#!/bin/bash

# Updates the SP1 setup of the Aggregation Mode in the devnet:
# - Deploys the SP1 Groth16 verifier from SP1VerifierGroth16Deployer.s.sol and adds its route to the SP1 verifier gateway
#   (in both the sp1 and the alignedlayer anvil states)
# - Allows the SP1 chunk aggregator program of programs_ids.json in both AlignedProofAggregationService deployments,
#   the one with the real verifiers and the mock one (in the alignedlayer anvil state)
# - Copies the contracts of the alignedlayer anvil state to the ethereum-package preloaded contracts (network_params.yaml)
#
# It must run once per verifier version, since the gateway does not allow adding the same route twice.

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
# Anvil account #7, owner of the AlignedProofAggregationService deployments
OWNER_PRIVATE_KEY="0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356"

SP1_PROVING_SYSTEM_ID=1
SP1_CHUNK_AGGREGATOR_VK_HASH=$(jq -r '.sp1_chunk_aggregator_vk_hash' ../aggregation_mode/proof_aggregator/programs_ids.json)
PROOF_AGGREGATOR_ADDRESS=$(jq -r '.addresses.alignedProofAggregationService' ./script/output/devnet/proof_aggregation_service_deployment_output.json)
PROOF_AGGREGATOR_MOCK_ADDRESS=$(jq -r '.addresses.alignedProofAggregationService' ./script/output/devnet/proof_aggregation_service_deployment_output.mock.json)

# Newer anvil versions can't parse the transactions of states dumped by older ones.
# The accounts already hold the whole chain state, so the transactions are dropped.
# The block headers are kept so the chain does not go back to block 0.
strip_transactions() {
    jq '.blocks |= map(.transactions = []) | .transactions = []' "$1" > "$2"
}

if cast block-number --rpc-url $RPC_DEVNET > /dev/null 2>&1; then
    echo "There is a node running on $RPC_DEVNET, stop it before running this script"
    exit 1
fi

# The states are updated in temporary files, the committed ones are only replaced once every state is updated
upgraded_states_dir=$(mktemp -d)
anvil_pid=""
cleanup() {
    if [ -n "$anvil_pid" ]; then
        kill $anvil_pid 2> /dev/null
        wait $anvil_pid 2> /dev/null
    fi
    rm -rf "$upgraded_states_dir"
}
trap cleanup EXIT

for state in sp1-deployed-anvil-state.json alignedlayer-deployed-anvil-state.json; do
    strip_transactions "scripts/anvil/state/$state" "$upgraded_states_dir/$state.initial"

    anvil --load-state "$upgraded_states_dir/$state.initial" --dump-state "$upgraded_states_dir/$state.dump" &
    anvil_pid=$!
    until cast block-number --rpc-url $RPC_DEVNET > /dev/null 2>&1; do
        sleep 1
    done

    forge script ./script/deploy/SP1VerifierGroth16Deployer.s.sol:SP1VerifierScript \
        --rpc-url $RPC_DEVNET \
        --private-key $VERIFIER_PRIVATE_KEY \
        --broadcast

    if [ "$state" = "alignedlayer-deployed-anvil-state.json" ]; then
        for proof_aggregator_address in $PROOF_AGGREGATOR_ADDRESS $PROOF_AGGREGATOR_MOCK_ADDRESS; do
            cast send $proof_aggregator_address \
                "allowVerifyingProgram(bytes32,uint8)" $SP1_CHUNK_AGGREGATOR_VK_HASH $SP1_PROVING_SYSTEM_ID \
                --rpc-url $RPC_DEVNET \
                --private-key $OWNER_PRIVATE_KEY
        done
    fi

    # Stop anvil so it dumps its state
    kill $anvil_pid
    wait $anvil_pid || true
    anvil_pid=""

    strip_transactions "$upgraded_states_dir/$state.dump" "$upgraded_states_dir/$state"
done

for state in sp1-deployed-anvil-state.json alignedlayer-deployed-anvil-state.json; do
    mv "$upgraded_states_dir/$state" "scripts/anvil/state/$state"
done

# Update the ethereum-package preloaded contracts with the contracts of the alignedlayer state
# (see docs/0_internal/ethereum_package.md).
# The system contracts that anvil adds for the hardforks (e.g. the EIP-2935 block hashes one) are skipped,
# since the genesis of the ethereum-package already includes them.
SYSTEM_CONTRACTS='[
    "0x000f3df6d732807ef1319fb7b8bb8522d0beac02",
    "0x0000f90827f1c53a10cb7a02335b175320002935",
    "0x00000961ef480eb55e80d19ad83579a64c007002",
    "0x0000bbddc7ce488642fb579f8b00f3a590007251"
]'
preloaded_contracts="$upgraded_states_dir/preloaded_contracts.json"
jq --argjson system_contracts "$SYSTEM_CONTRACTS" \
    '.accounts | to_entries | map(select(.value.code != "0x" and (.key | ascii_downcase | IN($system_contracts[]) | not))) | from_entries' \
    scripts/anvil/state/alignedlayer-deployed-anvil-state.json \
    | sed 's/^/     /' | sed "1s/^     {/    '{/" | sed "\$s/}\$/}'/" > "$preloaded_contracts"

awk -v preloaded_contracts="$preloaded_contracts" '
    skipping && /^     }'"'"'$/ { skipping = 0; next }
    skipping { next }
    { print }
    /^  additional_preloaded_contracts:$/ {
        while ((getline line < preloaded_contracts) > 0) print line
        skipping = 1
    }
' ../network_params.yaml > "$upgraded_states_dir/network_params.yaml"
mv "$upgraded_states_dir/network_params.yaml" ../network_params.yaml
