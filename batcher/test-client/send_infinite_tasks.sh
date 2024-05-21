#!/bin/bash

counter=1

while true 
do
    echo "Generating proof $counter != 0"

    go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go $counter

    ./batcher/test-client/target/debug/test-client --proving_system Groth16Bn254 --proof task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.proof --public_input task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.pub --vk task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.vk
    
    sleep 1
    counter=$((counter + 1))
done
