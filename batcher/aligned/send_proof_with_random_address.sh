#!/bin/bash

PROOF_TYPE="sp1" #sp1|groth16|plonk|risc0

RPC_URL=${RPC_URL:-http://localhost:8545}
if [ -z "$NETWORK" ]; then
    echo "NETWORK is not set. Setting it to devnet"
    NETWORK="devnet"
fi

if [ -z $1 ]; then
    echo "Proof type not provided, using SP1 default"
else
    PROOF_TYPE=$1
fi

echo "Sending $PROOF_TYPE proof to the batcher"


if [ $PROOF_TYPE == "sp1" ]; then
    echo "TOD SP1"

elif [ $PROOF_TYPE == "groth16" ]; then
    cd batcher/aligned/ 
    cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
        --random_address \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [ $PROOF_TYPE == "plonk" ]; then
    echo "TOD plonk"

elif [ $PROOF_TYPE == "risc0" ]; then
    echo "TODO risc0 "

fi
