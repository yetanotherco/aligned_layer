.PHONY: 

SHELL := /bin/bash

gen_env:
	cp .env.example .env

run_l2:
	@cd crates/l2 && cargo run --release --bin main

generate_program_id:
	@cd crates/l2 && cargo run --release --bin write_program_id
	@cat ./crates/l2/programs_ids.json

deploy_contract_holesky:
	@. ./contracts/.env.holesky && . ./contracts/scripts/deploy.sh
