deploy_verifier:
	@./contracts/deploy.sh

generate_proof:
	@cd wordle/script && cargo run -r

submit_proof:
	@aligned submit \
    --proving_system SP1 \
    --proof wordle/script/proof-with-io.json \
    --vm_program wordle/program/elf/riscv32im-succinct-zkvm-elf \
    --conn wss://batcher.alignedlayer.com

CONTRACT_ADDRESS=0xcb589D7607624168C871477F623031B21972afc6
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
ALIGNED_VERIFICATION_DATA=./aligned_verification_data/d148e70e4009e9e4b31652acb3d6d5afafded10ed6322c6360d7d15e263183d0_75.json

encode_verification_data:
	@python3 encode_verification_data.py \
		--aligned-verification-data $(ALIGNED_VERIFICATION_DATA)
