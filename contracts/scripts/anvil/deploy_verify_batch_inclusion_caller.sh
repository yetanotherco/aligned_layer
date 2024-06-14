#!/bin/bash
# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in contracts/scripts/anvil
cd "$parent_path"

# At this point we are in contracts
cd ../../
  

# Deploy the contracts
forge script script/deploy/VerifyBatchInclusionCallerDeployer.s.sol \
    "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8" \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --slow \
    --sig "run(address _targetContract)"
