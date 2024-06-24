deploy_verifier:
	@./contracts/deploy.sh

generate_proof:
	@cd wordle/script && cargo run -r

submit_proof:
	@aligned submit \
    --proving_system SP1 \
    --proof wordle/script/proof-with-io.json \
    --vm_program wordle/program/elf/riscv32im-succinct-zkvm-elf

CONTRACT_ADDRESS=0x04C89607413713Ec9775E14b954286519d836FEf
RPC_URL=http://localhost:8545
ALIGNED_VERIFICATION_DATA=./aligned_verification_data/1a7d17a0_0.json

verify_proof:
	@python3 verify.py \
		--aligned-verification-data $(ALIGNED_VERIFICATION_DATA) \
		--rpc-url $(RPC_URL) \
		--contract-address $(CONTRACT_ADDRESS)
