#!/bin/bash

# Params:
# PROOF_TYPE = sp1|gnark_groth16|gnark_plonk|risc0|circom_groth16  (default sp1)
# RPC_URL (default localhost:8545)
# NETWORK   devnet|holesky-stage|holesky
# REPETITIONS (default 1)

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
    PROOF_TYPE="sp1" #sp1|gnark_groth16|gnark_plonk|risc0|circom_groth16
fi

if [ -z $REPETITIONS ]; then
    echo "REPETITIONS not provided, using 1 as default"
    REPETITIONS=1
fi

echo "Sending $REPETITIONS $PROOF_TYPE proof/s to the batcher"
echo "Batcher in $NETWORK and endpoint at $RPC_URL"

if [[ $PROOF_TYPE == "sp1" ]]; then
    aligned submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf \
		--public_input ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.pub \
		--random_address \
    --repetitions $REPETITIONS \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "gnark_groth16" ]]; then
    aligned submit \
		--proving_system GnarkGroth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.vk \
    --random_address \
    --repetitions $REPETITIONS \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "gnark_plonk" ]]; then
    aligned submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_pub_input_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.vk \
		--random_address \
    --repetitions $REPETITIONS \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "risc0" ]]; then
	aligned submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_2_0.proof \
    --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_2_0.bin \
    --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_2_0.pub \
		--random_address \
    --repetitions $REPETITIONS \
		--rpc_url $RPC_URL \
		--network $NETWORK

elif [[ $PROOF_TYPE == "circom_groth16" ]]; then
    aligned submit \
    --proving_system CircomGroth16Bn256 \
    --proof ../../scripts/test_files/circom_groth16_bn256_script/proof.json \
    --public_input ../../scripts/test_files/circom_groth16_bn256_script/public.json \
    --vk ../../scripts/test_files/circom_groth16_bn256_script/verification_key.json \
    --random_address \
    --repetitions $REPETITIONS \
    --rpc_url $RPC_URL \
    --network $NETWORK

else
    echo "Incorrect proof type provided $1"
    exit 1
fi
