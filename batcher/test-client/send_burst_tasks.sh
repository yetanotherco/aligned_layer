#!/bin/bash

counter=1
burst=5
if [ -z "$1" ]; then
    echo "Using default burst value: 10"
elif ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be a number."
    exit 1
else
    burst=$1
    echo "Using burst value: $timer"
fi



echo "Generating proof $counter != 0"
go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go $counter

iter=1
while [ $iter -le $burst ]
do
    echo "iter: $iter"
    ./batcher/test-client/target/debug/test-client --proving_system Groth16Bn254 --proof task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.proof --public_input task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.pub --vk task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.vk
    iter=$((iter + 1))
    # counter=$((counter + 1))
done
