
make anvil-start &> /dev/null &

export ANVIL_PID=$!

go test tests/integration_test.go -v

kill ${ANVIL_PID}
