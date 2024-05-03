echo starting anvil
make anvil-start & #&> /dev/null &
export ANVIL_PID=$!

echo registering operator
make operator-full-registration #&> /dev/null

# echo starting aggregator
make aggregator-start & #&> /dev/null &
export AGGREGATOR_PID=$!

sleep 5

echo starting operator
make operator-start &> /dev/null &
export OPERATOR_PID=$!

sleep 3

echo sending task
make send-plonk_bls12_381-proof &> /dev/null

echo sending task
make send-plonk_bn254-proof &> /dev/null

#uncomment to run locally:
# sleep 3600

# kill ${ANVIL_PID}
# kill ${AGGREGATOR_PID}
# kill ${OPERATOR_PID}
