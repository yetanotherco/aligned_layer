#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

source scripts/.env

# Deploy Proof Aggregation Service Contract
forge script script/deploy/AlignedProofAggregationServiceDeployer.s.sol \
    $PROOF_AGGREGATION_SERVICE_CONFIG_PATH \
    $PROOF_AGGREGATION_SERVICE_OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --sig "run(string memory batcherConfigPath, string memory outputPath)"
