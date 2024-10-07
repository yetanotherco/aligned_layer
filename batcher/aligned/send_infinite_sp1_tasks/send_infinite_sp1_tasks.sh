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

RPC=${RPC:-http://localhost:8545}
BATCHER_CONN=${BATCHER_CONN:-ws://localhost:8080}
if [ -z "$NETWORK" ]; then
    echo "NETWORK is not set. Setting it to devnet"
    NETWORK="devnet"
fi

cd ./batcher/aligned

while true
do
    echo "Generating proof $counter"
    random_address=$(openssl rand -hex 20)
    echo "Random address: $random_addr"

    aligned submit \
        --proving_system SP1 \
        --proof ../../scripts/test_files/sp1/sp1_fibonacci.proof \
        --vm_program ../../scripts/test_files/sp1/sp1_fibonacci.elf \
        --proof_generator_addr "$random_address" \
        --network "$NETWORK" \
        --batcher_url "$BATCHER_CONN" \
        --repetitions "2" \
        --rpc_url "$RPC"

    sleep "$timer"
    counter=$((counter + 1))
done

