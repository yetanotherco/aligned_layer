#!/bin/bash

RPC_URL="http://localhost:59139"

sleep 1

cd contracts

# Deploy the contracts
forge script script/deploy/EigenLayerDeployer.s.sol \
    --rpc-url $RPC_URL \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --slow \
    --sig "run(string memory configFile)" -- eigen.devnet.config.json

cd ..
