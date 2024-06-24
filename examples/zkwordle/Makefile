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

CONTRACT_ADDRESS=0xcb589D7607624168C871477F623031B21972afc6
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
ALIGNED_VERIFICATION_DATA=./aligned_verification_data/da80e461d506fe1d0167385e017a5a01966656a4f85191cabe96aa3668093308_175.json

encode_verification_data:
	@python3 encode_verification_data.py \
		--aligned-verification-data $(ALIGNED_VERIFICATION_DATA)
