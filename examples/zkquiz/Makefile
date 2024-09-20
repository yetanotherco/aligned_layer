deploy_verifier:
	@. ./contracts/.env && . ./contracts/deploy.sh

CONTRACT_ADDRESS=0xA828f1463074d26FB761c662F80194f5dB2d31D0
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
VERIFICATION_DATA=./aligned_verification_data/0a1fab5df88a71e48633cbdeedc8d1a234b790d15a8a2fd04cd6a03c1e05b5ef_212.json

answer_quiz:
	@cd quiz/script && cargo run -r -- \
		--keystore-path $(KEYSTORE_PATH) \
 		--rpc-url $(RPC_URL) \
  		--verifier-contract-address $(CONTRACT_ADDRESS)
