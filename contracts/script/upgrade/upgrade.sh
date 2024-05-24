#!/bin/bash

anvil --load-state scripts/anvil/state/alignedlayer-deployed-anvil-state.json --dump-state scripts/anvil/state/alignedlayer-upgraded-anvil-state.json &

sleep 2

forge script script/upgrade/AlignedLayerUpgrader.s.sol \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \

pkill anvil
