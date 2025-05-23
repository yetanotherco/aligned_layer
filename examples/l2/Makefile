.PHONY: 

SHELL := /bin/bash

gen_devnet_owner_wallet:
	rm -f ./contract-owner.keystore.json
	cast wallet import -k . contract-owner.keystore.json --private-key 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a --unsafe-password "<!w5Y%In94Vw"
	@echo "Funding on Aligned" 
	cd ../../batcher/aligned/ && \
	cargo run --release -- deposit-to-batcher \
	 --amount 0.01ether \
	 --private_key 0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a \
	 --network devnet \
	 --rpc_url http://localhost:8545

gen_env_l2_holesky:
	@cp .env.holesky .env

gen_env_l2_devnet:
	@cp .env.devnet .env

run_l2:
	@cd crates/l2 && cargo run --release --bin main

generate_program_id:
	@cd crates/l2 && cargo run --release --bin write_program_id
	@cat ./crates/l2/programs_ids.json

gen_env_contract_holesky:
	@cp ./contracts/.env.holesky ./contracts/.env

gen_env_contract_devnet:
	@cp ./contracts/.env.devnet ./contracts/.env

deploy_contract:
	@. ./contracts/.env && . ./contracts/scripts/deploy.sh
