!/bin/bash

echo "Starting local execution"

echo "\nStarting anvil"
make anvil-start & #> /dev/null &
export ANVIL_PID=$!

sleep 1

echo "\nStarting Aggregator"
make aggregator-start & #> /dev/null &
export AGGREGATOR_PID=$!

sleep 3

echo "\nRegistering Operator"
make operator-full-registration #&> /dev/null
sleep 3
echo "\nStarting Operator"
make operator-start & #> /dev/null &
export OPERATOR_PID=$!

echo "\nSending 1 task"
make send-plonk_bls12_381-proof

echo "Ready"
read  -n 1 -p "Press anything to stop execution of Anvil & Aggregator & Operator" wait
echo "Stopping execution"

echo "\nStopping Operator"
kill ${OPERATOR_PID}

echo "\nStopping Aggregator"
kill ${AGGREGATOR_PID}

echo "\nStopping Anvil"
kill ${ANVIL_PID}
