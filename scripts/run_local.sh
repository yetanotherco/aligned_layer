#!/bin/bash

echo "Starting local execution"

echo "Starting anvil"
make anvil_start > /dev/null 2>&1 & 
export ANVIL_PID=$!

sleep 1

echo "Starting Aggregator"
make aggregator_start > /dev/null 2>&1 & 

export AGGREGATOR_PID=$!

sleep 3

echo "Registering and Starting Operator"
make operator_register_and_start > /dev/null 2>&1 &

export OPERATOR_PID=$!

sleep 2

echo "Starting Batcher"
make batcher_start & #> /dev/null 2>&1 &
export BATCHER_PID=$!

sleep 50

make batcher_send_burst_groth16

echo "Ready"
read  -n 1 -p "Press anything to stop execution of Anvil, Aggregator, Operator and Batcher" wait
echo "Stopping execution"

echo "Stopping Operator"
kill ${OPERATOR_PID}

echo "Stopping Aggregator"
kill ${AGGREGATOR_PID}

echo "Stopping Anvil"
kill ${ANVIL_PID}

echo "Stopping Batcher"
kill ${BATCHER_PID}

exit 0
