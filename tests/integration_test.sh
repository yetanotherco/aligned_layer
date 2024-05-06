echo starting anvil
make anvil-start & #&> /dev/null &
export ANVIL_PID=$!

sleep 3

echo registering operator
make operator-full-registration #&> /dev/null
# make operator-get-eth
# make operator-register-with-eigen-layer 
# ./scripts/mint_mock_token.sh
# make operator-deposit-into-mock-strategy 
# make operator-register-with-aligned-layer



# echo starting aggregator
make aggregator-start & #&> /dev/null &
export AGGREGATOR_PID=$!

sleep 5

echo starting operator
make operator-start &> /dev/null &
export OPERATOR_PID=$!

sleep 60

echo sending task
make send-plonk_bls12_381-proof &> /dev/null

echo sending task
make send-plonk_bn254-proof &> /dev/null

sleep 10

go test tests/integration_test.go -v

# kill ${ANVIL_PID}
# kill ${AGGREGATOR_PID}
# kill ${OPERATOR_PID}
