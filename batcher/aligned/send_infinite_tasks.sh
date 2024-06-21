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

    go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go $counter

    cd ./batcher/aligned && cargo run --release -- submit --proving_system Groth16Bn254 --proof ../../task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.proof --public_input ../../task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.pub --vk ../../task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.vk --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657
    cd ../..

    sleep $timer
    counter=$((counter + 1))
done
