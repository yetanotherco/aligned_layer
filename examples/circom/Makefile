SHELL := /bin/bash

generate_circom_groth16_bn256_setup: ## Run the circom_groth16_bn256_script setup
	@echo "Running circom_groth16_bn256 script setup..."
	@cd circuits && ./generate_setup.sh
	$(MAKE) get_vk_commitment

deploy_contract:
	@. ./contracts/.env && . ./contracts/deploy.sh

deploy_contract_devnet:
	@. ./contracts/.env.devnet && . ./contracts/deploy.sh

get_vk_commitment: 
	cargo run --release --bin get_vk_commitment

run:
	cargo run --release --bin main

run_devnet:
	cargo run --release --bin main -- .env.devnet

