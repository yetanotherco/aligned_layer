.PHONY: 

SHELL := /bin/bash

run:
	@cd crates/l2 && cargo run --release --bin main

print_program_id:
	@cd crates/l2 && cargo run --release --bin write_program_id
	@cat ./crates/l2/programs_ids.json


deploy_contract_holesky:
	@. ./contracts/.env.holesky && . ./contracts/scripts/deploy.sh
