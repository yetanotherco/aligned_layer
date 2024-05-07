echo "\nStarting anvil"
make anvil-start & #> /dev/null &
export ANVIL_PID=$!

sleep 3

echo "\nStarting Aggregator"
make aggregator-start & #> /dev/null &
export AGGREGATOR_PID=$!

sleep 3

#TODO fix this locally works the second time
echo "\nRegistering Operator"
make operator-full-registration #&> /dev/null
sleep 3
echo "\nStarting Operator"
make operator-start & #> /dev/null &
export OPERATOR_PID=$!

sleep 15

echo "\nSending Task 1" 
make send-plonk_bls12_381-proof #> /dev/null

echo "\nSending Task 2"
make send-plonk_bn254-proof #> /dev/null

sleep 10

echo "\nVerifying Tasks sent & accepted"

go test tests/verify_test.go -v
passed=$?

## Crashes the CI, only run locally
if [ "$LOCAL" != "" ]; then
    echo "\nStopping Anvil"
    kill ${ANVIL_PID}
    echo "\nStopping Aggregator"
    kill ${AGGREGATOR_PID}
    echo "\nStopping Operator"
    kill ${OPERATOR_PID}
fi

exit $passed
