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
    echo "Using burst value: $burst"
fi

counter=1
while true
do
  # Run in backaground to be able to run onece per second, and not wait for the previous one to finish
  ./aligned-batcher/aligned-batcher-client/generate_proof_and_send.sh $counter $burst &
  sleep 1
  counter=$((counter + 1))
done

