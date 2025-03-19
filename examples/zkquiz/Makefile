SHELL := /bin/bash

deploy_verifier:
	@. ./contracts/.env && . ./contracts/deploy.sh

deploy_verifier_devnet:
	. ./contracts/.devnet.env && . ./contracts/deploy.sh

CONTRACT_ADDRESS=0x1adFb00CC74Ff26bB05419953006c66B1abFCD45
STAGE_CONTRACT_ADDRESS=0xc4da6fcfa317eaf166b09ef276c0bdf43648a65f
RPC_URL=https://ethereum-holesky-rpc.publicnode.com

answer_quiz:
	@cd quiz/script && cargo run -r -- \
		--keystore-path $(KEYSTORE_PATH) \
 		--rpc-url $(RPC_URL) \
  		--verifier-contract-address $(CONTRACT_ADDRESS)

answer_quiz_stage:
	@cd quiz/script && cargo run -r -- \
		--keystore-path $(KEYSTORE_PATH) \
		--rpc-url $(RPC_URL) \
		--network holesky-stage \
		--verifier-contract-address $(STAGE_CONTRACT_ADDRESS)

answer_quiz_local: 
	@cd quiz/script && cargo run -r -- \
		--keystore-path ../../../../config-files/devnet/keys/operator-3.ecdsa.key.json \
 		--rpc-url http://localhost:8545 \
		--batcher-url ws://localhost:8080 \
		--network devnet \
  		--verifier-contract-address $(CONTRACT_ADDRESS)

compile_elf:
	cd quiz/program && cargo prove build
