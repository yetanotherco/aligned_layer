deps: venv

venv: venv/touchfile

venv/touchfile: requirements.txt
	@python3 -m venv venv
	@. venv/bin/activate && pip install -r requirements.txt
	@touch venv/touchfile

deploy_verifier:
	@./contracts/deploy.sh

answer_quiz:
	@cd quiz/script && cargo run -r

ADDRESS=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

submit_proof:
	@aligned submit \
    --proving_system SP1 \
    --proof quiz/script/proof-with-io.json \
    --vm_program quiz/program/elf/riscv32im-succinct-zkvm-elf \
    --proof_generator_addr $(ADDRESS) \
    --conn wss://batcher.alignedlayer.com

CONTRACT_ADDRESS=0x01F7556EA65A5B4142C0a0e36a45C65Ca9F3A4FF
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
VERIFICATION_DATA=./aligned_verification_data/0a1fab5df88a71e48633cbdeedc8d1a234b790d15a8a2fd04cd6a03c1e05b5ef_212.json
PRIVATE_KEY=ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

encode_verification_data: venv
	@. venv/bin/activate; python3 encode_verification_data.py --aligned-verification-data $(VERIFICATION_DATA)

verify_and_get_reward: venv
	@cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(PRIVATE_KEY) \
		$(CONTRACT_ADDRESS) \
		$(shell source venv/bin/activate && python3 encode_verification_data.py --aligned-verification-data $(VERIFICATION_DATA))
