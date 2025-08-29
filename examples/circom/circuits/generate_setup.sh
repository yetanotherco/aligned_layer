#!/bin/bash

# Reference: https://github.com/iden3/snarkjs?tab=readme-ov-file#guide

# 1. Start a new powers of tau ceremony
snarkjs powersoftau new bn128 14 pot14_0000.ptau -v
# 2. Contribute to the ceremony
snarkjs powersoftau contribute pot14_0000.ptau pot14_0001.ptau --name="First contribution" -v -e="random text"
# 3. Provide a second contribution
snarkjs powersoftau contribute pot14_0001.ptau pot14_0002.ptau --name="Second contribution" -v -e="some random text"
# 4. Provide a third contribution using third-party software
snarkjs powersoftau export challenge pot14_0002.ptau challenge_0003
snarkjs powersoftau challenge contribute bn128 challenge_0003 response_0003 -e="some random text"
snarkjs powersoftau import response pot14_0002.ptau response_0003 pot14_0003.ptau -n="Third contribution name"
# 5. Verify the protocol so far. Expected output: [INFO]  snarkJS: Powers Of tau file OK!
snarkjs powersoftau verify pot14_0003.ptau
# 6. Apply a random beacon
snarkjs powersoftau beacon pot14_0003.ptau pot14_beacon.ptau 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon"
# 7. Prepare phase 2
snarkjs powersoftau prepare phase2 pot14_beacon.ptau pot14_final.ptau -v
# 8. Verify the final ptau. Expected output: [INFO]  snarkJS: Powers Of tau file OK!
snarkjs powersoftau verify pot14_final.ptau
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
