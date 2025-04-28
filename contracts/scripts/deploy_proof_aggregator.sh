#!/bin/bash

# ENV VARIABLES
#
# PROOF_AGGREGATOR_DEPLOY_CONFIG_PATH: Path to the proof aggregator deploy config file
#   - Holesky Stage: ./script/deploy/config/holesky/proof-aggregator-service.holesky.config.stage.json
#   - Holesky Prod: ./script/deploy/config/holesky/proof-aggregator-service.holesky.config.json
#
# PROOF_AGGREGATOR_OUTPUT_PATH: Path to the proof aggregator output file
#   - Holesky Stage: ./script/output/holesky/proof_aggregation_service_deployment_output.stage.json
#   - Holesky Prod: ./script/output/holesky/proof_aggregation_service_deployment_output.json
#
# RPC_URL: The RPC URL to connect to the Ethereum network
#
# PRIVATE_KEY: The private key to use for the deployment
#
# ETHERSCAN_API_KEY: The Etherscan API key to use for verification
#

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts
cd "$parent_path"

# At this point we are in contracts
cd ../

# Deploy proof aggregation service contract
forge script script/deploy/AlignedProofAggregationServiceDeployer.s.sol \
    $PROOF_AGGREGATOR_DEPLOY_CONFIG_PATH \
    $PROOF_AGGREGATOR_OUTPUT_PATH \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --slow \
    --sig "run(string configPath, string outputPath)" \
    --via-ir
