#!/bin/bash

# Check that USER_PRIVATE_KEY is not empty
if [[ "$USER_PRIVATE_KEY" -eq "" ]]; then
  echo "USER_PRIVATE_KEY is empty, using default value 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
  USER_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
fi;

if [ $# -lt 1 ]; then
    echo "Usage: $0 <x> <repetitions?>"
    exit 1
fi

x="$1"

if [ $# -eq 2 ]; then
    repetitions="$2"
else
    repetitions=1
fi

echo "Generating proof $x != 0"
go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go $x

./batcher/target/release/aligned submit \
  --proving_system Groth16Bn254 \
  --repetitions $repetitions \
  --proof task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.proof \
  --public_input task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.pub \
  --vk task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.vk \
  --private_key $USER_PRIVATE_KEY \
  --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657
