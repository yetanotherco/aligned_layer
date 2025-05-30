# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ "$ETHERSCAN_API_KEY" = "0x0" ]; then
    forge script ./script/StateTransitionDeployer.s.sol \
        $PROGRAM_ID \
        $INITIAL_STATE_ROOT \
        $ALIGNED_PROOF_AGGREGATOR_ADDRESS \
        $OWNER_ADDRESS \
        --rpc-url $RPC_URL \
        --private-key $PRIVATE_KEY \
        --broadcast \
        --slow \
        --sig "run(bytes32,bytes32,address,address)" \
        --via-ir
else 
    forge script ./script/StateTransitionDeployer.s.sol \
        $PROGRAM_ID \
        $INITIAL_STATE_ROOT \
        $ALIGNED_PROOF_AGGREGATOR_ADDRESS \
        $OWNER_ADDRESS \
        --rpc-url $RPC_URL \
        --private-key $PRIVATE_KEY \
        --broadcast \
        --verify \
        --etherscan-api-key $ETHERSCAN_API_KEY \
        --slow \
        --sig "run(bytes32,bytes32,address,address)" \
        --via-ir
fi

