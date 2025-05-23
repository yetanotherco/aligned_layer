#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
# At this point we are in tests/integration
cd "$parent_path"

# Start anvil chain in the background and dump its state to a json file upon exit
anvil --load-state state/sp1-deployed-anvil-state.json --dump-state state/alignedlayer-deployed-anvil-state.json &

cd ../../

sleep 1

# Deploy the contracts
forge script script/deploy/AlignedLayerDeployer.s.sol \
    ./script/output/devnet/eigenlayer_deployment_output.json \
    ./script/deploy/config/devnet/aligned.devnet.config.json \
    ./script/output/devnet/alignedlayer_deployment_output.json \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string memory existingDeploymentInfoPath, string memory deployConfigPath, string memory outputPath)"


ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS=$(jq -r '.addresses.alignedLayerServiceManager' ./script/output/devnet/alignedlayer_deployment_output.json)

# Can't deploy on another script, current open issue: https://github.com/foundry-rs/foundry/issues/7952
forge script ../examples/verify/script/VerifyBatchInclusionCallerDeployer.s.sol \
    "$ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS" \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(address _targetContract)"

output_path=./script/output/devnet/batcher_deployment_output.json

# Deploy Batcher Payments Contract
forge script script/deploy/BatcherPaymentServiceDeployer.s.sol \
    ./script/deploy/config/devnet/batcher-payment-service.devnet.config.json \
    $output_path \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string batcherConfigPath, string outputPath)"

# Extract the batcher payment service values from the output
batcher_payment_service_proxy=$(jq -r '.addresses.batcherPaymentService' $output_path)
batcher_payment_service_implementation=$(jq -r '.addresses.batcherPaymentServiceImplementation' $output_path)

# Give initial funds to ServiceManager for the Batcher
cast send $ALIGNED_LAYER_SERVICE_MANAGER_ADDRESS "depositToBatcher(address)()" $batcher_payment_service_proxy --value 1ether --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" --rpc-url "http://localhost:8545"

# Use the extracted value to replace the  batcher payment service values in alignedlayer_deployment_output.json and save it to a temporary file
jq --arg batcher_payment_service_proxy "$batcher_payment_service_proxy" '.addresses.batcherPaymentService = $batcher_payment_service_proxy' "script/output/devnet/alignedlayer_deployment_output.json" > "script/output/devnet/alignedlayer_deployment_output.temp1.json"
jq --arg batcher_payment_service_implementation "$batcher_payment_service_implementation" '.addresses.batcherPaymentServiceImplementation = $batcher_payment_service_implementation' "script/output/devnet/alignedlayer_deployment_output.temp1.json" > "script/output/devnet/alignedlayer_deployment_output.temp2.json"

# Replace the original file with the temporary file
mv "script/output/devnet/alignedlayer_deployment_output.temp2.json" "script/output/devnet/alignedlayer_deployment_output.json"

# Delete the temporary file
rm -f "script/output/devnet/alignedlayer_deployment_output.temp1.json"
rm -f "script/output/devnet/alignedlayer_deployment_output.temp2.json"


# Update Program IDs in anvil deployment
cd ..
make agg_mode_write_program_ids

# Copy new values to config file
jq '.programs_id.risc0AggregationProgramImageId = $input[0].risc0_chunk_aggregator_image_id | .programs_id.sp1AggregationProgramVKHash = $input[0].sp1_chunk_aggregator_vk_hash' \
  --slurpfile input aggregation_mode/programs_ids.json \
  contracts/script/deploy/config/devnet/proof-aggregator-service.devnet.config.json \
  > temp.json && mv temp.json contracts/script/deploy/config/devnet/proof-aggregator-service.devnet.config.json

jq '.programs_id.risc0AggregationProgramImageId = $input[0].risc0_chunk_aggregator_image_id | .programs_id.sp1AggregationProgramVKHash = $input[0].sp1_chunk_aggregator_vk_hash' \
  --slurpfile input aggregation_mode/programs_ids.json \
  contracts/script/deploy/config/devnet/proof-aggregator-service.devnet.mock.config.json \
  > temp.json && mv temp.json contracts/script/deploy/config/devnet/proof-aggregator-service.devnet.mock.config.json

cd contracts

# Deploy proof aggregation service contract with SP1 Verifier
forge script script/deploy/AlignedProofAggregationServiceDeployer.s.sol \
    ./script/deploy/config/devnet/proof-aggregator-service.devnet.config.json \
    ./script/output/devnet/proof_aggregation_service_deployment_output.json \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string configPath, string outputPath)" \
    --via-ir

# Deploy proof aggregation service contract with Mocked Verifier
forge script script/deploy/AlignedProofAggregationServiceDeployer.s.sol \
    ./script/deploy/config/devnet/proof-aggregator-service.devnet.mock.config.json \
    ./script/output/devnet/proof_aggregation_service_deployment_output.mock.json \
    --rpc-url "http://localhost:8545" \
    --private-key "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" \
    --broadcast \
    --sig "run(string configPath, string outputPath)" \
    --via-ir

# Kill the anvil process to save state
pkill anvil
