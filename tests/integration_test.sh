echo Starting anvil
make anvil-start &> /dev/null &
export ANVIL_PID=$!

sleep 3

echo Registering Operator
make operator-full-registration #&> /dev/null

echo Starting Aggregator
make aggregator-start &> /dev/null &
export AGGREGATOR_PID=$!

sleep 5

echo Starting Operator
make operator-start &> /dev/null &
export OPERATOR_PID=$!

sleep 15

echo "Sending Task 1" 
make send-plonk_bls12_381-proof &> /dev/null

echo "Sending Task 2"
make send-plonk_bn254-proof &> /dev/null

sleep 10

echo "Verifying Tasks sent & accepted"

go test tests/integration_test.go -v

echo "DONE"

kill ${ANVIL_PID}
kill ${AGGREGATOR_PID}
kill ${OPERATOR_PID}
