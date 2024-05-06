echo "\nStarting anvil"
make anvil-start & #> /dev/null &
export ANVIL_PID=$!

sleep 3

echo "\nRegistering Operator"
make operator-full-registration #&> /dev/null

echo "\nStarting Aggregator"
make aggregator-start & #> /dev/null &
export AGGREGATOR_PID=$!

sleep 5

echo "\nStarting Operator"
make operator-start & #> /dev/null &
export OPERATOR_PID=$!

sleep 15

# echo "\nSending Task 1" 
# make send-plonk_bls12_381-proof #> /dev/null

# echo "\nSending Task 2"
# make send-plonk_bn254-proof #> /dev/null

sleep 10

echo "\nVerifying Tasks sent & accepted"

output=$(go test tests/verify_test.go -v)
echo $output

## Crashes the CI, only run locally
if [ "$LOCAL" != "" ]; then
    echo "\nStopping Anvil"
    kill ${ANVIL_PID}
    echo "\nStopping Aggregator"
    kill ${AGGREGATOR_PID}
    echo "\nStopping Operator"
    kill ${OPERATOR_PID}
fi

