.PHONY: 

SHELL := /bin/bash

gen_env_l2_holesky:
	cp .env.holesky .env

run_l2:
	@cd crates/l2 && cargo run --release --bin main

generate_program_id:
	@cd crates/l2 && cargo run --release --bin write_program_id
	@cat ./crates/l2/programs_ids.json

gen_env_contract_holesky:
	cp ./contracts/.env.holesky ./contracts/.env

deploy_contract:
	@. ./contracts/.env && . ./contracts/scripts/deploy.sh
