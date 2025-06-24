#!/bin/bash

# Reference: https://github.com/iden3/snarkjs?tab=readme-ov-file#10-compile-the-circuit

# 10. Compile the circuit
circom --r1cs --wasm --c --sym --inspect circuit.circom
# 11. View information about the circuit
snarkjs r1cs info circuit.r1cs
# 12. Print the constraints [omitted]
# 13. Export r1cs to json
snarkjs r1cs export json circuit.r1cs circuit.r1cs.json
# 14. Calculate the witness
snarkjs wtns calculate circuit_js/circuit.wasm input.json witness.wtns
# 15. Setup. This generates the reference zkey without phase 2 contributions.
#IMPORTANT: Do not use this zkey in production, as it's not safe. It requires at least one contribution.
snarkjs groth16 setup circuit.r1cs pot14_final.ptau circuit_0000.zkey
# 16. Contribute to the phase 2 ceremony
snarkjs zkey contribute circuit_0000.zkey circuit_0001.zkey --name="1st Contributor Name" -v -e="Random entropy"
# 17. Provide a second contribution
snarkjs zkey contribute circuit_0001.zkey circuit_0002.zkey --name="Second contribution Name" -v -e="Another random entropy"
# 18. Provide a third contribution using third-party software
snarkjs zkey export bellman circuit_0002.zkey  challenge_phase2_0003
snarkjs zkey bellman contribute bn128 challenge_phase2_0003 response_phase2_0003 -e="some random text"
snarkjs zkey import bellman circuit_0002.zkey response_phase2_0003 circuit_0003.zkey -n="Third contribution name"
# 19. Verify the latest zkey. Expected output: [INFO]  snarkJS: ZKey Ok!
snarkjs zkey verify circuit.r1cs pot14_final.ptau circuit_0003.zkey
# 20. Apply a random beacon
snarkjs zkey beacon circuit_0003.zkey circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
# 21. Verify the final zkey. Expected output: [INFO]  snarkJS: ZKey Ok!
snarkjs zkey verify circuit.r1cs pot14_final.ptau circuit_final.zkey
# 22. Export the verification key
snarkjs zkey export verificationkey circuit_final.zkey verification_key.json
# 23a. Calculate the witness and generate the proof in one step
snarkjs groth16 fullprove input.json circuit_js/circuit.wasm circuit_final.zkey proof.json public.json
# 24. Verify the proof
snarkjs groth16 verify verification_key.json public.json proof.json
