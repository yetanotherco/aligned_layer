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

cd ./batcher/aligned

while true
do
    echo "Generating proof $counter"
    random_addr=$(python3 ./send_infinite_sp1_tasks/generate_address.py)
    echo "Random address: $random_addr"

    aligned submit \
    --proving_system SP1 \
    --proof test_files/sp1/sp1_fibonacci.proof \
    --vm_program test_files/sp1/sp1_fibonacci-elf \
    --proof_generator_addr "$random_addr"

    sleep "$timer"
    counter=$((counter + 1))
done
