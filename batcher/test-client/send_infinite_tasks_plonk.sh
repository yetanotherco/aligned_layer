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


while true 
do
    echo "Generating proof $counter != 0"

    go run task_sender/test_examples/gnark_plonk_bn254_infinite_script/cmd/main.go $counter

    ./batcher/test-client/target/debug/test-client --proving_system GnarkPlonkBn254 --proof task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}plonk.proof --public_input task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}_plonk.pub --vk task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}_plonk.vk
    
    sleep $timer
    counter=$((counter + 1))
done
