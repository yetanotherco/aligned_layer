#!/bin/bash

echo "Starting local execution"

echo "\nStarting anvil"
make anvil_start > /dev/null 2>&1 & 
export ANVIL_PID=$!

sleep 1

echo "\nStarting Aggregator"
make aggregator_start > /dev/null 2>&1 & 

export AGGREGATOR_PID=$!

sleep 3

echo "\nRegistering Operator"
make operator_full_registration
sleep 3
echo "\nStarting Operator"
make operator_start > /dev/null 2>&1 & 
export OPERATOR_PID=$!

sleep 2

echo "\nSending 1 task"
make send_plonk_bls12_381_proof > /dev/null 2>&1

echo "Ready"
read  -n 1 -p "Press anything to stop execution of Anvil & Aggregator & Operator" wait
echo "Stopping execution"

echo "\nStopping Operator"
kill ${OPERATOR_PID}

echo "\nStopping Aggregator"
kill ${AGGREGATOR_PID}

echo "\nStopping Anvil"
kill ${ANVIL_PID}

exit 0