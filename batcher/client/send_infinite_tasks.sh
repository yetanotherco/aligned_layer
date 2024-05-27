#!/bin/bash

counter=1
timer=3
proving_system="PlonkBn254"

if [ -z "$1" ]; then
    echo "Using default timer value: 3 seconds"
elif ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be a number."
    exit 1
else
    timer=$1
    echo "Using timer value: $timer seconds"
fi

if [ -z "$2" ]; then
    echo "Using default proving system: PlonkBn254"
elif [[ "$2" != "Groth16" && "$2" != "PlonkBn254" ]]; then
    echo "Error: Proving system must be either 'Groth16' or 'PlonkBn254'."
    exit 1
else
    proving_system=$2
    echo "Using proving system: $proving_system"
fi

while true 
do
    echo "Generating proof $counter != 0"

    if [ "$proving_system" == "Groth16" ]; then
        go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go $counter

        ./batcher/client/target/release/batcher-client --proving_system Groth16Bn254 --proof task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.proof --public_input task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.pub --vk task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${counter}_groth16.vk
    else
        go run task_sender/test_examples/gnark_plonk_bn254_infinite_script/cmd/main.go $counter

        ./batcher/client/target/release/batcher-client --proving_system GnarkPlonkBn254 --proof task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}_plonk.proof --public_input task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}_plonk.pub --vk task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs/ineq_${counter}_plonk.vk
    fi
    
    sleep $timer
    counter=$((counter + 1))
done
