.PHONY: help tests

SHELL := /bin/bash
OS := $(shell uname -s)

CONFIG_FILE?=config-files/config.yaml
AGG_CONFIG_FILE?=config-files/config-aggregator.yaml

OPERATOR_VERSION=v0.9.2

ifeq ($(OS),Linux)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_linux
endif

ifeq ($(OS),Darwin)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_macos
endif

ifeq ($(OS),Linux)
	export LD_LIBRARY_PATH += $(CURDIR)/operator/risc_zero/lib
endif

ifeq ($(OS),Linux)
	BUILD_OPERATOR = $(MAKE) build_operator_linux 
endif

ifeq ($(OS),Darwin)
	BUILD_OPERATOR = $(MAKE) build_operator_macos
endif


FFI_FOR_RELEASE ?= true

ifeq ($(FFI_FOR_RELEASE),true)
	RELEASE_FLAG=--release
	TARGET_REL_PATH=release
else
	RELEASE_FLAG=
	TARGET_REL_PATH=debug
endif

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

submodules:
	git submodule update --init --recursive
	@echo "Updated submodules"

deps: submodules go_deps build_all_ffi ## Install deps

go_deps:
	@echo "Installing Go dependencies..."
	go install github.com/maoueh/zap-pretty@latest
	go install github.com/ethereum/go-ethereum/cmd/abigen@latest
	go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

install_foundry:
	curl -L https://foundry.paradigm.xyz | bash

anvil_deploy_eigen_contracts:
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil_deploy_aligned_contracts:
	@echo "Deploying Aligned Contracts..."
	. contracts/scripts/anvil/deploy_aligned_contracts.sh

anvil_upgrade_aligned_contracts:
	@echo "Upgrading Aligned Contracts..."
	. contracts/scripts/anvil/upgrade_aligned_contracts.sh

anvil_upgrade_batcher_payment_service:
	@echo "Upgrading BatcherPayments contract..."
	. contracts/scripts/anvil/upgrade_batcher_payment_service.sh

anvil_upgrade_registry_coordinator:
	@echo "Upgrading Registry Coordinator Contracts..."
	. contracts/scripts/anvil/upgrade_registry_coordinator.sh

anvil_upgrade_bls_apk_registry:
	@echo "Upgrading Bls Apk Registry Contract..."
	. contracts/scripts/anvil/upgrade_bls_apk_registry.sh

anvil_upgrade_stake_registry:
	@echo "Upgrading Stake Registry Contract..."
	. contracts/scripts/anvil/upgrade_stake_registry.sh

anvil_upgrade_index_registry:
	@echo "Upgrading Index Registry Contracts..."
	. contracts/scripts/anvil/upgrade_index_registry.sh

anvil_upgrade_add_aggregator:
	@echo "Adding Aggregator to Aligned Contracts..."
	. contracts/scripts/anvil/upgrade_add_aggregator_to_service_manager.sh

pause_all_aligned_service_manager:
	@echo "Pausing all contracts..."
	. contracts/scripts/pause_aligned_service_manager.sh all

unpause_all_aligned_service_manager:
	@echo "Pausing all contracts..."
	. contracts/scripts/unpause_aligned_service_manager.sh all

get_paused_state_aligned_service_manager:
	@echo "Getting paused state of Aligned Service Manager contract..."
	. contracts/scripts/get_paused_state_aligned_service_manager.sh

pause_batcher_payment_service:
	@echo "Pausing BatcherPayments contract..."
	. contracts/scripts/pause_batcher_payment_service.sh

unpause_batcher_payment_service:
	@echo "Unpausing BatcherPayments contract..."
	. contracts/scripts/unpause_batcher_payment_service.sh

get_paused_state_batcher_payments_service:
	@echo "Getting paused state of Batcher Payments Service contract..."
	. contracts/scripts/get_paused_state_batcher_payments_service.sh

lint_contracts:
	@cd contracts && npm run lint:sol

anvil_start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json

anvil_start_with_block_time:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7

_AGGREGATOR_:

aggregator_start:
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(AGG_CONFIG_FILE) \
	2>&1 | zap-pretty

aggregator_send_dummy_responses:
	@echo "Sending dummy responses to Aggregator..."
	@cd aggregator && go run dummy/submit_task_responses.go


__OPERATOR__:

operator_start:
	@echo "Starting Operator..."
	go run operator/cmd/main.go start --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

operator_full_registration: operator_get_eth operator_register_with_eigen_layer operator_mint_mock_tokens operator_deposit_into_mock_strategy operator_whitelist_devnet operator_register_with_aligned_layer

operator_register_and_start: operator_full_registration operator_start

build_operator: deps
	$(BUILD_OPERATOR)

build_operator_macos:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

build_operator_linux:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION) -r $(LD_LIBRARY_PATH)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

update_operator:
	@echo "Updating Operator..."
	@./scripts/fetch_latest_release.sh
	@make build_operator
	@./operator/build/aligned-operator --version

operator_valid_marshall_fuzz_macos:
	@cd operator/pkg && go test -fuzz=FuzzValidMarshall -ldflags=-extldflags=-Wl,-ld_classic

operator_valid_marshall_fuzz_linux:
	@cd operator/pkg && \
	go test -fuzz=FuzzValidMarshall

operator_marshall_unmarshall_fuzz_macos:
	@cd operator/pkg && go test -fuzz=FuzzMarshalUnmarshal -ldflags=-extldflags=-Wl,-ld_classic

operator_marshall_unmarshall_fuzz_linux:
	@cd operator/pkg && \
	go test -fuzz=FuzzMarshalUnmarshal

bindings:
	cd contracts && ./generate-go-bindings.sh

test:
	go test ./...


get_delegation_manager_address:
	@sed -n 's/.*"delegationManager": "\([^"]*\)".*/\1/p' contracts/script/output/devnet/eigenlayer_deployment_output.json

operator_generate_keys:
	@echo "Generating BLS keys"
	eigenlayer operator keys create --key-type bls --insecure operator
	@echo "Generating ECDSA keys"
	eigenlayer operator keys create --key-type ecdsa --insecure operator

operator_generate_config:
	@echo "Generating operator config"
	eigenlayer operator config create

operator_get_eth:
	@echo "Sending funds to operator address on devnet"
	@. ./scripts/fund_operator_devnet.sh

operator_register_with_eigen_layer:
	@echo "Registering operator with EigenLayer"
	@echo "" | eigenlayer operator register $(CONFIG_FILE)

operator_mint_mock_tokens:
	@echo "Minting tokens"
	. ./scripts/mint_mock_token.sh $(CONFIG_FILE) 100000000000000000

operator_whitelist_devnet:
	@echo "Whitelisting operator"
	$(eval OPERATOR_ADDRESS = $(shell yq -r '.operator.address' $(CONFIG_FILE)))
	@echo "Operator address: $(OPERATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/whitelist_operator.sh $(OPERATOR_ADDRESS)

operator_remove_devnet:
	@echo "Removing operator"
	$(eval OPERATOR_ADDRESS = $(shell yq -r '.operator.address' $(CONFIG_FILE)))
	@echo "Operator address: $(OPERATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/remove_operator.sh $(OPERATOR_ADDRESS)

operator_whitelist:
	@echo "Whitelisting operator $(OPERATOR_ADDRESS)"
	@. contracts/scripts/.env && . contracts/scripts/whitelist_operator.sh $(OPERATOR_ADDRESS)

operator_deposit_into_mock_strategy:
	@echo "Depositing into mock strategy"
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.addresses.strategies.MOCK' contracts/script/output/devnet/eigenlayer_deployment_output.json))
	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount 100000000000000000

operator_deposit_into_strategy:
	@echo "Depositing into strategy"
	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--amount 1000

operator_register_with_aligned_layer:
	@echo "Registering operator with AlignedLayer"
	@go run operator/cmd/main.go register \
		--config $(CONFIG_FILE)

operator_deposit_and_register: operator_deposit_into_strategy operator_register_with_aligned_layer


__BATCHER__:

BURST_SIZE=5

user_fund_payment_service:
	@. ./scripts/user_fund_payment_service_devnet.sh

./batcher/aligned-batcher/.env:
	@echo "To start the Batcher ./batcher/aligned-batcher/.env needs to be manually set"; false;

batcher_start: ./batcher/aligned-batcher/.env user_fund_payment_service
	@echo "Starting Batcher..."
	@cargo run --manifest-path ./batcher/aligned-batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./batcher/aligned-batcher/.env

batcher_start_local: user_fund_payment_service
	@echo "Starting Batcher..."
	@$(MAKE) run_storage &
	@cargo run --manifest-path ./batcher/aligned-batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./batcher/aligned-batcher/.env.dev

install_batcher:
	@cargo install --path batcher/aligned-batcher

install_aligned:
	@./batcher/aligned/install_aligned.sh

uninstall_aligned:
	@rm -rf ~/.aligned && echo "Aligned uninstalled"

install_aligned_compiling:
	@cargo install --path batcher/aligned

build_batcher_client:
	@cd batcher/aligned && cargo b --release

batcher/target/release/aligned:
	@cd batcher/aligned && cargo b --release


RPC_URL=http://localhost:8545
NETWORK=devnet # devnet | holesky-stage | holesky

batcher_send_sp1_task:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci.elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_sp1_burst:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci.elf \
		--repetitions $(BURST_SIZE) \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_infinite_sp1:
	@echo "Sending infinite SP1 fibonacci task to Batcher..."
	@./batcher/aligned/send_infinite_sp1_tasks/send_infinite_sp1_tasks.sh

batcher_send_risc0_task:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
        --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
        --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_risc0_task_no_pub_input:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/no_public_inputs/risc_zero_no_pub_input.proof \
        --vm_program ../../scripts/test_files/risc_zero/no_public_inputs/no_pub_input_id.bin \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--payment_service_addr $(BATCHER_PAYMENTS_CONTRACT_ADDRESS)

batcher_send_risc0_burst:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
        --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
        --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
        --repetitions $(BURST_SIZE) \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_plonk_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_plonk_bn254_burst: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--repetitions 4 \
		--network $(NETWORK)

batcher_send_plonk_bls12_381_task: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_plonk_bls12_381_burst: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions 15 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_groth16_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_infinite_groth16: batcher/target/release/aligned ## Send a different Groth16 BN254 proof using the client every 3 seconds
	@mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs
	@echo "Sending a different GROTH16 BN254 proof in a loop every n seconds..."
	@./batcher/aligned/send_infinite_tasks.sh 4

batcher_send_burst_groth16: batcher/target/release/aligned
	@echo "Sending a burst of tasks to Batcher..."
	@mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs
	@./batcher/aligned/send_burst_tasks.sh $(BURST_SIZE) $(START_COUNTER)

__GENERATE_PROOFS__:
 # TODO add a default proving system

generate_plonk_bls12_381_proof: ## Run the gnark_plonk_bls12_381_script
	@echo "Running gnark_plonk_bls12_381 script..."
	@go run scripts/test_files/gnark_plonk_bls12_381_script/main.go

generate_plonk_bn254_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_plonk_bn254 script..."
	@go run scripts/test_files/gnark_plonk_bn254_script/main.go

generate_groth16_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_groth_bn254 script..."
	@go run scripts/test_files/gnark_groth16_bn254_script/main.go

generate_groth16_ineq_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_groth_bn254_ineq script..."
	@go run scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go 1

__METRICS__:
# Prometheus and graphana
run_metrics: ## Run metrics using metrics-docker-compose.yaml
	@echo "Running metrics..."
	@docker compose -f metrics-docker-compose.yaml up

__STORAGE__:
run_storage: ## Run storage using storage-docker-compose.yaml
	@echo "Running storage..."
	@docker compose -f storage-docker-compose.yaml up

__DEPLOYMENT__:
deploy_aligned_contracts: ## Deploy Aligned Contracts
	@echo "Deploying Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_aligned_contracts.sh

deploy_pauser_registry: ## Deploy Pauser Registry
	@echo "Deploying Pauser Registry..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_pauser_registry.sh

upgrade_aligned_contracts: ## Upgrade Aligned Contracts
	@echo "Upgrading Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_aligned_contracts.sh

upgrade_pauser_aligned_contracts: ## Upgrade Aligned Contracts with Pauser initialization
	@echo "Upgrading Aligned Contracts with Pauser initialization..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_add_pausable_to_service_manager.sh

upgrade_registry_coordinator: ## Upgrade Registry Coordinator
	@echo "Upgrading Registry Coordinator..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_registry_coordinator.sh

upgrade_bls_apk_registry: ## Upgrade Registry Coordinator
	@echo "Upgrading BLS Apk Registry Coordinator..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_bls_apk_registry.sh

upgrade_index_registry: ## Upgrade Registry Coordinator
	@echo "Upgrading Index Registry..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_index_registry.sh

upgrade_stake_registry: ## Upgrade Stake Registry
	@echo "Upgrading Stake Registry..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_stake_registry.sh

upgrade_add_aggregator: ## Add Aggregator to Aligned Contracts
	@echo "Adding Aggregator to Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_add_aggregator_to_service_manager.sh

deploy_verify_batch_inclusion_caller:
	@echo "Deploying VerifyBatchInclusionCaller contract..."
	@. examples/verify/.env && . examples/verify/scripts/deploy_verify_batch_inclusion_caller.sh

deploy_batcher_payment_service:
	@echo "Deploying BatcherPayments contract..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_batcher_payment_service.sh

upgrade_batcher_payment_service:
	@echo "Upgrading BatcherPayments contract..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_batcher_payment_service.sh

build_aligned_contracts:
	@cd contracts/src/core && forge build

show_aligned_error_codes:
	@echo "\nAlignedLayerServiceManager errors:"
	@cd contracts && forge inspect src/core/IAlignedLayerServiceManager.sol:IAlignedLayerServiceManager errors
	@echo "\nBatcherPaymentService errors:"
	@cd contracts && forge inspect src/core/BatcherPaymentService.sol:BatcherPaymentService errors

__BUILD__:
build_binaries:
	@echo "Building aggregator..."
	@go build -o ./aggregator/build/aligned-aggregator ./aggregator/cmd/main.go
	@echo "Aggregator built into /aggregator/build/aligned-aggregator"
	@echo "Building aligned layer operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Aligned layer operator built into /operator/build/aligned-operator"

__SP1_FFI__: ##
build_sp1_macos:
	@cd operator/sp1/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_ffi.dylib operator/sp1/lib/libsp1_verifier.dylib

build_sp1_linux:
	@cd operator/sp1/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_ffi.so operator/sp1/lib/libsp1_verifier.so

test_sp1_rust_ffi:
	@echo "Testing SP1 Rust FFI source code..."
	@cd operator/sp1/lib && RUST_MIN_STACK=83886080 cargo t --release

test_sp1_go_bindings_macos: build_sp1_macos
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

test_sp1_go_bindings_linux: build_sp1_linux
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

# @cp -r scripts/test_files/sp1/fibonacci_proof_generator/script/sp1_fibonacci.elf scripts/test_files/sp1/
generate_sp1_fibonacci_proof:
	@cd scripts/test_files/sp1/fibonacci_proof_generator/script && RUST_LOG=info cargo run --release
	@mv scripts/test_files/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf scripts/test_files/sp1/sp1_fibonacci.elf
	@mv scripts/test_files/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof scripts/test_files/sp1/
	@echo "Fibonacci proof and ELF generated in scripts/test_files/sp1 folder"

generate_risc_zero_empty_journal_proof:
	@cd scripts/test_files/risc_zero/no_public_inputs && RUST_LOG=info cargo run --release
	@echo "Fibonacci proof and ELF with empty journal generated in scripts/test_files/risc_zero/no_public_inputs folder"


__RISC_ZERO_FFI__: ##
build_risc_zero_macos:
	@cd operator/risc_zero/lib && cargo build $(RELEASE_FLAG)
	@cp operator/risc_zero/lib/target/$(TARGET_REL_PATH)/librisc_zero_verifier_ffi.dylib operator/risc_zero/lib/librisc_zero_verifier_ffi.dylib

build_risc_zero_linux:
	@cd operator/risc_zero/lib && cargo build $(RELEASE_FLAG)
	@cp operator/risc_zero/lib/target/$(TARGET_REL_PATH)/librisc_zero_verifier_ffi.so operator/risc_zero/lib/librisc_zero_verifier_ffi.so

test_risc_zero_rust_ffi:
	@echo "Testing RISC Zero Rust FFI source code..."
	@cd operator/risc_zero/lib && cargo test --release

test_risc_zero_go_bindings_macos: build_risc_zero_macos
	@echo "Testing RISC Zero Go bindings..."
	go test ./operator/risc_zero/... -v

test_risc_zero_go_bindings_linux: build_risc_zero_linux
	@echo "Testing RISC Zero Go bindings..."
	go test ./operator/risc_zero/... -v

generate_risc_zero_fibonacci_proof:
	@cd scripts/test_files/risc_zero/fibonacci_proof_generator && \
		RUST_LOG=info cargo run --release && \
		echo "Fibonacci proof, pub input and image ID generated in scripts/test_files/risc_zero folder"

__MERKLE_TREE_FFI__: ##
build_merkle_tree_macos:
	@cd operator/merkle_tree/lib && cargo build $(RELEASE_FLAG)
	@cp operator/merkle_tree/lib/target/$(TARGET_REL_PATH)/libmerkle_tree.dylib operator/merkle_tree/lib/libmerkle_tree.dylib
	@cp operator/merkle_tree/lib/target/$(TARGET_REL_PATH)/libmerkle_tree.a operator/merkle_tree/lib/libmerkle_tree.a

build_merkle_tree_linux:
	@cd operator/merkle_tree/lib && cargo build $(RELEASE_FLAG)
	@cp operator/merkle_tree/lib/target/$(TARGET_REL_PATH)/libmerkle_tree.so operator/merkle_tree/lib/libmerkle_tree.so
	@cp operator/merkle_tree/lib/target/$(TARGET_REL_PATH)/libmerkle_tree.a operator/merkle_tree/lib/libmerkle_tree.a

test_merkle_tree_rust_ffi:
	@echo "Testing Merkle Tree Rust FFI source code..."
	@cd operator/merkle_tree/lib && RUST_MIN_STACK=83886080 cargo t --release

test_merkle_tree_go_bindings_macos: build_merkle_tree_macos
	@echo "Testing Merkle Tree Go bindings..."
	go test ./operator/merkle_tree/... -v

test_merkle_tree_go_bindings_linux: build_merkle_tree_linux
	@echo "Testing Merkle Tree Go bindings..."
	go test ./operator/merkle_tree/... -v

test_merkle_tree_old_go_bindings_macos: build_merkle_tree_macos_old
	@echo "Testing Old Merkle Tree Go bindings..."
	go test ./operator/merkle_tree_old/... -v


__BUILD_ALL_FFI__:

build_all_ffi: ## Build all FFIs
	$(BUILD_ALL_FFI)
	@echo "Created FFIs"

build_all_ffi_macos: ## Build all FFIs for macOS
	@echo "Building all FFIs for macOS..."
	@$(MAKE) build_sp1_macos
	@$(MAKE) build_risc_zero_macos
	@$(MAKE) build_merkle_tree_macos
	@echo "All macOS FFIs built successfully."

build_all_ffi_linux: ## Build all FFIs for Linux
	@echo "Building all FFIs for Linux..."
	@$(MAKE) build_sp1_linux
	@$(MAKE) build_risc_zero_linux
	@$(MAKE) build_merkle_tree_linux
	@echo "All Linux FFIs built successfully."

__EXPLORER__:
run_explorer: explorer_run_db explorer_ecto_setup_db
	@cd explorer/ && \
		pnpm install --prefix assets && \
		mix setup && \
		./start.sh

explorer_build_db:
	@cd explorer && \
		docker build -t explorer-postgres-image .

explorer_run_db: explorer_remove_db_container
	@cd explorer && \
		docker run -d --name explorer-postgres-container -p 5432:5432 -v explorer-postgres-data:/var/lib/postgresql/data explorer-postgres-image

explorer_ecto_setup_db:
		@cd explorer/ && \
		./ecto_setup_db.sh

explorer_remove_db_container:
	@cd explorer && \
		docker stop explorer-postgres-container || true  && \
		docker rm explorer-postgres-container || true

explorer_clean_db: explorer_remove_db_container
	@cd explorer && \
		docker volume rm explorer-postgres-data || true

explorer_dump_db:
	@cd explorer && \
		docker exec -t explorer-postgres-container pg_dumpall -c -U explorer_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /explorer"

explorer_recover_db: explorer_run_db
	@read -p $$'\e[32mEnter the dump file to recover (e.g., dump.20230607_123456.sql): \e[0m' DUMP_FILE && \
	cd explorer && \
	docker cp $$DUMP_FILE explorer-postgres-container:/dump.sql && \
	docker exec -t explorer-postgres-container psql -U explorer_user -d explorer_db -f /dump.sql && \
	echo "Recovered database successfully from $$DUMP_FILE"

explorer_fetch_old_batches:
	@cd explorer && \
	./scripts/fetch_old_batches.sh 1728056 1729806

explorer_fetch_old_operators_strategies_restakes:
	@cd explorer && \
	./scripts/fetch_old_operators_strategies_restakes.sh 0

explorer_create_env:
	@cd explorer && \
	cp .env.dev .env

__TRACKER__:

tracker_devnet_start: tracker_run_db
	@cd operator_tracker/ && \
		cargo run -r -- --env-file .env.dev

tracker_install: tracker_build_db
	cargo install --path ./operator_tracker

tracker_build_db:
	@cd operator_tracker && \
		docker build -t tracker-postgres-image .

tracker_run_db: tracker_build_db tracker_remove_db_container
	@cd operator_tracker && \
		docker run -d --name tracker-postgres-container -p 5433:5432 -v tracker-postgres-data:/var/lib/postgresql/data tracker-postgres-image

tracker_remove_db_container:
	docker stop tracker-postgres-container || true  && \
	    docker rm tracker-postgres-container || true

tracker_clean_db: tracker_remove_db_container
	docker volume rm tracker-postgres-data || true

tracker_dump_db:
	@cd operator_tracker && \
		docker exec -t tracker-postgres-container pg_dumpall -c -U tracker_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /operator_tracker"

__TELEMETRY__:
# Collector, Jaeger and Elixir API
telemetry_full_start: open_telemetry_start telemetry_start

# Collector and Jaeger
open_telemetry_start: ## Run open telemetry services using telemetry-docker-compose.yaml
	@echo "Running telemetry..."
	@docker compose -f telemetry-docker-compose.yaml up -d

open_telemetry_prod_start: ## Run open telemetry services with Cassandra using telemetry-prod-docker-compose.yaml
	@echo "Running telemetry for Prod..."
	@docker compose -f telemetry-prod-docker-compose.yaml up -d

# Elixir API
telemetry_start: telemetry_run_db telemetry_ecto_migrate ## Run Telemetry API
	@cd telemetry_api && \
	 	./start.sh

telemetry_ecto_migrate: ##
		@cd telemetry_api && \
			./ecto_setup_db.sh

telemetry_build_db:
	@cd telemetry_api && \
		docker build -t telemetry-postgres-image .

telemetry_run_db: telemetry_build_db telemetry_remove_db_container
	@cd telemetry_api && \
		docker run -d --name telemetry-postgres-container -p 5434:5432 -v telemetry-postgres-data:/var/lib/postgresql/data telemetry-postgres-image

telemetry_remove_db_container:
	@docker stop telemetry-postgres-container || true  && \
	    docker rm telemetry-postgres-container || true

telemetry_clean_db: telemetry_remove_db_container
	@docker volume rm telemetry-postgres-data || true

telemetry_dump_db:
	@cd telemetry_api && \
		docker exec -t telemetry-postgres-container pg_dumpall -c -U telemetry_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /telemetry_api"

telemetry_create_env:
	@cd telemetry_api && \
		cp .env.dev .env
