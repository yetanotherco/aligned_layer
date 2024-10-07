#!/bin/bash

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Load environment variables from .env file
if [ -f "$SCRIPT_DIR/.env" ]; then
    source "$SCRIPT_DIR/.env"
fi

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
go run scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go $x

# Set default values for RPC and BATCHER if they are not set
RPC=${RPC:-http://localhost:8545}
BATCHER_CONN=${BATCHER_CONN:-ws://localhost:8080}
if [ -z "$NETWORK" ]; then
    echo "NETWORK is not set. Setting it to devnet"
    NETWORK="devnet"
fi

cmd=(
    ./batcher/target/release/aligned
    submit
    --proving_system Groth16Bn254
    --repetitions "$repetitions"
    --proof "scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.proof"
    --public_input "scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.pub"
    --vk "scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.vk"
    --proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657
    --rpc_url "$RPC"
    --batcher_url "$BATCHER_CONN"
    --network "$NETWORK"
)

# If PRIVATE_KEY is set then add private key argument
if [ -n "$PRIVATE_KEY" ]; then
    cmd+=(--private_key "$PRIVATE_KEY")
fi

# Execute the command
"${cmd[@]}"
