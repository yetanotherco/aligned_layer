#!/bin/bash

counter=1
timer=3
if [ -z "$1" ]; then
    echo "Using default timer value: 3 seconds"
elif ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be a number."
    exit 1
else
    timer=$1
    echo "Using timer value: $timer seconds"
fi

# Set default values for RPC and BATCHER if they are not set
RPC=${RPC:-http://localhost:8545}
if [ -z "$NETWORK" ]; then
    echo "NETWORK is not set. Setting it to devnet"
    NETWORK="devnet"
fi


while true 
do
    echo "Generating proof $counter != 0"

    go run scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go $counter

    cd ./batcher/aligned && cargo run --release -- submit \
    --proving_system Groth16Bn254 \
    --proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.proof \
    --public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.pub \
    --vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.vk \
    --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
    --repetitions "2" \
    --rpc_url "$RPC" \
    --network "$NETWORK"

    cd ../..

    sleep $timer
    counter=$((counter + 1))
done
