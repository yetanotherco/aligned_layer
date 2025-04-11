#!/bin/bash

if [ -z "$NETWORK" ]; then
    echo "NETWORK is not set. Setting it to devnet"
    NETWORK="devnet"
fi

if [ -z "$RPC_URL" ]; then
    echo "RPC_URL is not set. Setting it to localhost:8545"
    RPC_URL="http://localhost:8545"
fi

if [ -z $PROOF_TYPE ]; then
    echo "Proof type not provided, using SP1 default"
    PROOF_TYPE="sp1" #sp1|groth16|plonk|risc0
fi

echo "Sending $PROOF_TYPE proof to the batcher"

if [[ $PROOF_TYPE == "sp1" ]]; then
	cd batcher/aligned/
    cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci_4_1_3.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci_4_1_3.elf \
        --random_address \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "groth16" ]]; then
    cd batcher/aligned/ 
    cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
        --random_address \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "plonk" ]]; then
    cd batcher/aligned/ 
    cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
		--random_address \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "risc0" ]]; then
    cd batcher/aligned/ 
    cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/no_public_inputs/risc_zero_no_pub_input_2_0.proof \
        --vm_program ../../scripts/test_files/risc_zero/no_public_inputs/no_pub_input_id_2_0.bin \
		--random_address \
		--rpc_url $RPC_URL \
		--network $NETWORK

else
    echo "Incorrect proof type provided $1"
    exit 1
fi
