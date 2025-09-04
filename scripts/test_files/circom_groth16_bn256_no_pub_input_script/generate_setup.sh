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
