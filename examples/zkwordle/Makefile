deploy_verifier:
	@./contracts/deploy.sh

generate_proof:
	@cd wordle/script && cargo run -r

PROOF_GEN_ADDR=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

submit_proof:
	@aligned submit \
    --proving_system SP1 \
    --proof wordle/script/proof-with-io.json \
    --vm_program wordle/program/elf/riscv32im-succinct-zkvm-elf \
    --proof_generator_addr $(PROOF_GEN_ADDR) \
    --conn wss://batcher.alignedlayer.com

CONTRACT_ADDRESS=0x3b32Ec6DfdfE8dDb048028328e5750ab6E108BE6
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
ALIGNED_VERIFICATION_DATA=./aligned_verification_data/a0e7197421a011a7c0671db9adf433fa6c344568ac4ef361c58abfa5714d6508_160.json

encode_verification_data:
	@python3 encode_verification_data.py \
		--aligned-verification-data $(ALIGNED_VERIFICATION_DATA)

verify_proof_submission:
	@cast send \
		--rpc-url https://rpc.alignedlayer.com \
		--private-key 2d3049d91bf27fcc829b37b14e2a9c021d8f9b9752612a818a8952cc6d9ad32a \
		$(CONTRACT_ADDRESS) \
		$(shell python3 encode_verification_data.py --aligned-verification-data $(ALIGNED_VERIFICATION_DATA))

