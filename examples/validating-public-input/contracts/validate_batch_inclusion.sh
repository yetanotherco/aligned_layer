#!/bin/bash

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1 ; pwd -P )

# At this point we are in aligned-example
cd "$parent_path" || exit 1

# check if the number of arguments is correct
if [ "$#" -ne 3 ]; then
    echo "Usage: verify_batch_inclusion.sh <FIBONACCI_VALIDATOR_ADDRESS> <DATA_FILE_NAME> <VERIFIER_ID>"
    exit 1
fi

FIBONACCI_VALIDATOR_ADDRESS=$1
DATA_FILE_NAME=$2
VERIFIER_ID=$3

if [ -z "$RPC_URL" ]; then
    echo "RPC_URL is not set. Please set it in .env"
    exit 1
fi

if [ -z "$PRIVATE_KEY" ]; then
    echo "PRIVATE_KEY is not set. Please set it in .env"
    exit 1
fi

proof_commitment=$(jq -r '.proof_commitment' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
pub_input_commitment=$(jq -r '.pub_input_commitment' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
program_id_commitment=$(jq -r '.program_id_commitment' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
proof_generator_addr=$(jq -r '.proof_generator_addr' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
batch_merkle_root=$(jq -r '.batch_merkle_root' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
merkle_proof=$(jq -r '.merkle_proof' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
verification_data_batch_index=$(jq -r '.verification_data_batch_index' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")
pub_input=$(jq -r '.pub_input' "../aligned-integration/batch_inclusion_data/$DATA_FILE_NAME")

cast send --rpc-url $RPC_URL $FIBONACCI_VALIDATOR_ADDRESS \
	"verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256, bytes, string)" \
    $proof_commitment \
    $pub_input_commitment \
    $program_id_commitment \
    $proof_generator_addr \
    $batch_merkle_root \
    $merkle_proof \
    $verification_data_batch_index \
    $pub_input \
    $VERIFIER_ID \
    --private-key $PRIVATE_KEY
