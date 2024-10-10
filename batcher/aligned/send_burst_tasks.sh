#!/bin/bash

counter=1
burst=8

if [ -z "$1" ]; then
    echo "Using default burst value: 10"
elif ! [[ "$1" =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be a number."
    exit 1
else
    burst=$1
    echo "Using burst value: $burst"
fi

if [ -z "$2" ]; then
    echo "Using default counter start value: 1"
    counter=1
elif ! [[ "$2" =~ ^[0-9]+$ ]]; then
    echo "Error: Second argument must be a number."
    exit 1
else
    counter=$2
    echo "Starting counter from: $counter"
fi

while true
do
    # Run in backaground to be able to run onece per second, and not wait for the previous one to finish
    . ./batcher/aligned/generate_proof_and_send.sh $counter $burst &
    sleep 3
    counter=$((counter + 1))
done
