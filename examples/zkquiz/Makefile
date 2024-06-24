deploy_verifier:
	@./contracts/deploy.sh

generate_proof:
	@cd quiz/script && cargo run -r

PROOF_GEN_ADDR=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

submit_proof:
	@aligned submit \
    --proving_system SP1 \
    --proof quiz/script/proof-with-io.json \
    --vm_program quiz/program/elf/riscv32im-succinct-zkvm-elf \
    --proof_generator_addr $(PROOF_GEN_ADDR) \
    --conn wss://batcher.alignedlayer.com

CONTRACT_ADDRESS=0x41f1d1BD9a78Bf66EAeA2d6fb1e57d3dd415c839
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
ALIGNED_VERIFICATION_DATA=./aligned_verification_data/b43f689d980ecdc21b24d5feacc75b5e5b1de57922c6744ef6171a7189520522_37.json
PRIVATE_KEY=ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

encode_verification_data:
	@python3 encode_verification_data.py \
		--aligned-verification-data $(ALIGNED_VERIFICATION_DATA)

verify_proof_submission:
	@cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(PRIVATE_KEY) \
		$(CONTRACT_ADDRESS) \
		$(shell python3 encode_verification_data.py --aligned-verification-data $(ALIGNED_VERIFICATION_DATA))
