#!/bin/bash

# This script send periodically a burst of proof with a specific amount of repetitions
# the proofs always have a random address
# Params:
# PROOF_TYPE = sp1|groth16|plonk|risc0  (default sp1)
# RPC_URL (default localhost:8545)
# NETWORK   devnet|holesky-stage|holesky
# REPETITIONS (default 1)
# BURST_DELAY in secs (default 30)

if [ -z $BURST_DELAY ]; then
    echo "Using default burst delay 30"
    BURST_DELAY=30
fi

while true 
do
    . ./send_proof_with_random_address.sh &
    sleep $BURST_DELAY
done
