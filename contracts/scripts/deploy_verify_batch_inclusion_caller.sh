#!/bin/bash
# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts/anvil
cd "$parent_path"

# At this point we are in contracts
cd ../

if [ -z "$ALIGNED_CONTRACT_ADDRESS" ]; then
    echo Missing exported ALIGNED_CONTRACT_ADDRESS variable
    exit 1
fi

forge script script/deploy/VerifyBatchInclusionCallerDeployer.s.sol \
    $ALIGNED_CONTRACT_ADDRESS \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --slow \
    --legacy \
    --sig "run(address _targetContract)"
