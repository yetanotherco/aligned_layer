#!/bin/bash

# Reference: https://github.com/iden3/snarkjs?tab=readme-ov-file#10-compile-the-circuit

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1 ; pwd -P )

cd "$parent_path" || exit 1


# 23a. Calculate the witness and generate the proof in one step
snarkjs groth16 fullprove input.json circuit_js/circuit.wasm circuit_final.zkey proof.json public.json
# 24. Verify the proof
snarkjs groth16 verify verification_key.json public.json proof.json
