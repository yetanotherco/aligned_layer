.PHONY: help tests

SHELL := /bin/bash
OS := $(shell uname -s)

NETWORK ?= devnet # devnet | holesky-stage | holesky
ifeq ($(NETWORK),holesky)
	RPC_URL ?= https://ethereum-holesky-rpc.publicnode.com
	BEACON_URL ?= https://eth-beacon-chain-holesky.drpc.org/rest/
else ifeq ($(NETWORK), holesky-stage)
	RPC_URL ?= https://ethereum-holesky-rpc.publicnode.com
	BEACON_URL ?= https://eth-beacon-chain-holesky.drpc.org/rest/
else
	RPC_URL ?= http://localhost:8545
	BEACON_URL ?= http://localhost:58801
endif

CONFIG_FILE?=config-files/config.yaml
export OPERATOR_ADDRESS ?= $(shell yq -r '.operator.address' $(CONFIG_FILE))
AGG_CONFIG_FILE?=config-files/config-aggregator.yaml

OPERATOR_VERSION=v0.17.0
EIGEN_SDK_GO_VERSION_DEVNET=v0.2.0-beta.1
EIGEN_SDK_GO_VERSION_TESTNET=v0.2.0-beta.1
EIGEN_SDK_GO_VERSION_MAINNET=v0.2.0-beta.1

ifeq ($(OS),Linux)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_linux
endif

ifeq ($(OS),Darwin)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_macos
endif

ifeq ($(OS),Linux)
	export LD_LIBRARY_PATH+=$(CURDIR)/operator/risc_zero/lib:$(CURDIR)/operator/sp1/lib
	OPERATOR_FFIS=$(CURDIR)/operator/risc_zero/lib:$(CURDIR)/operator/sp1/lib
endif

ifeq ($(OS),Linux)
	BUILD_OPERATOR = $(MAKE) operator_build_linux
endif

ifeq ($(OS),Darwin)
	BUILD_OPERATOR = $(MAKE) operator_build_macos
endif

ifeq ($(ENVIRONMENT), devnet)
	GET_SDK_VERSION = $(MAKE) operator_set_eigen_sdk_go_version_devnet
else ifeq ($(ENVIRONMENT), testnet)
	GET_SDK_VERSION = $(MAKE) operator_set_eigen_sdk_go_version_testnet
else ifeq ($(ENVIRONMENT), mainnet)
	GET_SDK_VERSION = $(MAKE) operator_set_eigen_sdk_go_version_mainnet
else
	GET_SDK_VERSION = $(MAKE) operator_set_eigen_sdk_go_version_error
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
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {if ($$1 ~ /^__/) printf "\033[33m%-50s\033[0m %s\n", $$1, $$2; else printf "\033[36m%-50s\033[0m %s\n", $$1, $$2}'

__DEPENDENCIES__: ## ____

submodules: ## Initialize and update git submodules
	git submodule update --init --recursive
	@echo "Updated submodules"

deps: submodules go_deps build_all_ffi ## Install deps

go_deps:
	@echo "Installing Go dependencies..."
	go install github.com/maoueh/zap-pretty@v0.3.0
	go install github.com/ethereum/go-ethereum/cmd/abigen@latest
	go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

foundry_install:
	curl -L https://foundry.paradigm.xyz | bash

eigenlayer_cli_install: ## Install Eigenlayer CLI v0.13.0
	curl -sSfL https://raw.githubusercontent.com/layr-labs/eigenlayer-cli/master/scripts/install.sh | sh -s -- v0.13.0

__UTILS__: ## ____

bindings: ## Generate Go bindings for contracts
	cd contracts && ./generate-go-bindings.sh

lint_contracts: ## Lint Solidity contracts
	@cd contracts && npm run lint:sol

build_aligned_contracts: ## Build AlignedLayer contracts
	@cd contracts/src/core && forge build --via-ir

show_aligned_error_codes: ## Show AlignedLayer error codes
	@echo "\nAlignedLayerServiceManager errors:"
	@cd contracts && forge inspect src/core/IAlignedLayerServiceManager.sol:IAlignedLayerServiceManager errors
	@echo "\nBatcherPaymentService errors:"
	@cd contracts && forge inspect src/core/BatcherPaymentService.sol:BatcherPaymentService errors

__CONTRACTS_DEPLOYMENT_ANVIL__: ## ____

anvil_deploy_all_contracts: anvil_deploy_eigen_contracts anvil_deploy_risc0_contracts anvil_deploy_sp1_contracts anvil_deploy_aligned_contracts

anvil_deploy_eigen_contracts: ## Deploy EigenLayer Contracts on ANVIL
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil_deploy_risc0_contracts: ## Deploy RISC0 Contracts used by Aggregation Mode on ANVIL
	@echo "Deploying RISC0 Contracts..."
	. contracts/scripts/anvil/deploy_risc0_contracts.sh

anvil_deploy_sp1_contracts: ## Deploy SP1 Contracts used by Aggregation Mode on ANVIL
	@echo "Deploying SP1 Contracts..."
	. contracts/scripts/anvil/deploy_sp1_contracts.sh

anvil_deploy_aligned_contracts: ## Deploy Aligned Contracts (Verification Layer and Aggregation Mode) on ANVIL
	@echo "Deploying Aligned Contracts..."
	. contracts/scripts/anvil/deploy_aligned_contracts.sh

anvil_upgrade_aligned_contracts: ## Upgrade Aligned Contracts (Verification Layer and Aggregation Mode) on ANVIL
	@echo "Upgrading Aligned Contracts..."
	. contracts/scripts/anvil/upgrade_aligned_contracts.sh

anvil_upgrade_batcher_payment_service: ## Upgrade BatcherPaymentService contract on ANVIL
	@echo "Upgrading BatcherPayments contract..."
	. contracts/scripts/anvil/upgrade_batcher_payment_service.sh

anvil_upgrade_registry_coordinator: ## Upgrade Registry Coordinator Contracts on ANVIL
	@echo "Upgrading Registry Coordinator Contracts..."
	. contracts/scripts/anvil/upgrade_registry_coordinator.sh

anvil_upgrade_bls_apk_registry: ## Upgrade Bls Apk Registry Contract on ANVIL
	@echo "Upgrading Bls Apk Registry Contract..."
	. contracts/scripts/anvil/upgrade_bls_apk_registry.sh

anvil_upgrade_stake_registry: ## Upgrade Stake Registry Contract on ANVIL
	@echo "Upgrading Stake Registry Contract..."
	. contracts/scripts/anvil/upgrade_stake_registry.sh

anvil_upgrade_index_registry: ## Upgrade Index Registry Contracts on ANVIL
	@echo "Upgrading Index Registry Contracts..."
	. contracts/scripts/anvil/upgrade_index_registry.sh

anvil_upgrade_add_aggregator:
	@echo "Adding Aggregator to Aligned Contracts..."
	. contracts/scripts/anvil/upgrade_add_aggregator_to_service_manager.sh

__CONTRACTS_MANAGEMENT__: ## ____

pause_all_aligned_service_manager: ## Pause all Aligned Service Manager contracts
	@echo "Pausing all contracts..."
	. contracts/scripts/pause_aligned_service_manager.sh all

unpause_all_aligned_service_manager: ## Unpause all Aligned Service Manager contracts
	@echo "Pausing all contracts..."
	. contracts/scripts/unpause_aligned_service_manager.sh all

get_paused_state_aligned_service_manager: ## Get paused state of Aligned Service Manager contracts
	@echo "Getting paused state of Aligned Service Manager contract..."
	. contracts/scripts/get_paused_state_aligned_service_manager.sh

pause_batcher_payment_service: ## Pause BatcherPaymentService contract
	@echo "Pausing BatcherPayments contract..."
	. contracts/scripts/pause_batcher_payment_service.sh

unpause_batcher_payment_service: ## Unpause BatcherPaymentService contract
	@echo "Unpausing BatcherPayments contract..."
	. contracts/scripts/unpause_batcher_payment_service.sh

get_paused_state_batcher_payments_service: ## Get paused state of BatcherPaymentService contract
	@echo "Getting paused state of Batcher Payments Service contract..."
	. contracts/scripts/get_paused_state_batcher_payments_service.sh
	
anvil_upgrade_initialize_disable_verifiers:
	@echo "Initializing disabled verifiers..."
	. contracts/scripts/anvil/upgrade_disabled_verifiers_in_service_manager.sh

# The verifier ID to enable or disable corresponds to the index of the verifier in the `ProvingSystemID` enum.
verifier_enable_devnet: ## Enable a verifier on devnet
	@echo "Enabling verifier with id: $(VERIFIER_ID)"
	PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 RPC_URL=http://localhost:8545 OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/enable_verifier.sh $(VERIFIER_ID)

verifier_disable_devnet: ## Disable a verifier on devnet
	@echo "Disabling verifier with id: $(VERIFIER_ID)"
	PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 RPC_URL=http://localhost:8545 OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/disable_verifier.sh $(VERIFIER_ID)

verifier_enable: ## Enable a verifier
	@echo "Enabling verifier with ID: $(VERIFIER_ID)"
	@. contracts/scripts/.env && . contracts/scripts/enable_verifier.sh $(VERIFIER_ID)

verifier_disable: ## Disable a verifier
	@echo "Disabling verifier with ID: $(VERIFIER_ID)"
	@. contracts/scripts/.env && . contracts/scripts/disable_verifier.sh $(VERIFIER_ID)

strategies_get_weight: ## Get the weight of a strategy
	@echo "Getting weight of strategy: $(STRATEGY_INDEX)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/get_strategy_weight.sh $(STRATEGY_INDEX)

strategies_update_weight: ## TODO
	@echo "Updating strategy weights: "
	@echo "STRATEGY_INDICES: $(STRATEGY_INDICES)"
	@echo "NEW_MULTIPLIERS: $(NEW_MULTIPLIERS)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/update_strategy_weight.sh $(STRATEGY_INDICES) $(NEW_MULTIPLIERS)

strategies_remove: ## TODO
	@echo "Removing strategies: $(INDICES_TO_REMOVE)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/remove_strategy.sh $(INDICES_TO_REMOVE)

strategies_get_addresses: ## TODO
	@echo "Getting strategy addresses"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/get_restakeable_strategies.sh

__ANVIL__: ## ____

anvil_start: ## Start Anvil with pre-deployed state
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7

anvil_start_with_more_prefunded_accounts:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7 -a 2000

__AGGREGATION_MODE__: ## ____

is_aggregator_set:
	@if [ -z "$(AGGREGATOR)" ]; then \
		echo "Error: AGGREGATOR is not set. Please provide arg AGGREGATOR='sp1' or 'risc0'."; \
		exit 1; \
	fi

reset_last_aggregated_block:
	@echo "Resetting last aggregated block..."
	@echo '{"last_aggregated_block":0}' > config-files/proof-aggregator.last_aggregated_block.json

AGGREGATION_MODE_SOURCES = $(wildcard ./aggregation_mode/Cargo.toml) $(wildcard ./aggregation_mode/src/**) $(wildcard ./aggregation_mode/aggregation_programs/risc0/Cargo.toml) $(wildcard ./aggregation_mode/aggregation_programs/risc0/src/**) $(wildcard ./aggregation_mode/aggregation_programs/sp1/Cargo.toml) $(wildcard ./aggregation_mode/aggregation_programs/sp1/src/**)

### All Dev proof aggregator receipts with no real proving
./aggregation_mode/target/release/proof_aggregator_dev: $(AGGREGATION_MODE_SOURCES)
		AGGREGATOR=$(AGGREGATOR) cargo build --manifest-path ./aggregation_mode/Cargo.toml --release --bin proof_aggregator_dev

proof_aggregator_start_dev: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_dev ## Starts proof aggregator with mock proofs (DEV mode). Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) RISC0_DEV_MODE=1 ./aggregation_mode/target/release/proof_aggregator_dev config-files/config-proof-aggregator-mock.yaml

proof_aggregator_start_dev_ethereum_package: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_dev ## Starts proof aggregator with mock proofs (DEV mode) in ethereum package. Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) RISC0_DEV_MODE=1 ./aggregation_mode/target/release/proof_aggregator_dev config-files/config-proof-aggregator-mock-ethereum-package.yaml

### All CPU proof aggregator receipts
./aggregation_mode/target/release/proof_aggregator_cpu: $(AGGREGATION_MODE_SOURCES)
	AGGREGATOR=$(AGGREGATOR) cargo build --features prove --manifest-path ./aggregation_mode/Cargo.toml --release --bin proof_aggregator_cpu

proof_aggregator_start: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_cpu ## Starts proof aggregator with proving activated. Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) ./aggregation_mode/target/release/proof_aggregator_cpu config-files/config-proof-aggregator.yaml

proof_aggregator_start_ethereum_package: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_cpu ## Starts proof aggregator with proving activated in ethereum package. Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) ./aggregation_mode/target/release/proof_aggregator_cpu config-files/config-proof-aggregator-ethereum-package.yaml

### All GPU proof aggregator receipts
./aggregation_mode/target/release/proof_aggregator_gpu: $(AGGREGATION_MODE_SOURCES)
	AGGREGATOR=$(AGGREGATOR) cargo build --features "prove,gpu" --manifest-path ./aggregation_mode/Cargo.toml --release --bin proof_aggregator_gpu

proof_aggregator_start_gpu: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_gpu ## Starts proof aggregator with proving + GPU acceleration (CUDA). Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) SP1_PROVER=cuda ./aggregation_mode/target/release/proof_aggregator_gpu config-files/config-proof-aggregator.yaml

proof_aggregator_start_gpu_ethereum_package: is_aggregator_set reset_last_aggregated_block ./aggregation_mode/target/release/proof_aggregator_gpu ## Starts proof aggregator with proving activated in ethereum package. Parameters: AGGREGATOR=<sp1|risc0>
	AGGREGATOR=$(AGGREGATOR) SP1_PROVER=cuda ./aggregation_mode/target/release/proof_aggregator_gpu config-files/config-proof-aggregator-ethereum-package.yaml

verify_aggregated_proof_sp1: 
	@echo "Verifying SP1 in aggregated proofs on $(NETWORK)..."
	@cd crates/cli/ && \
	cargo run verify-agg-proof \
		--network $(NETWORK) \
		--from-block $(FROM_BLOCK) \
		--proving_system SP1 \
		--public_input ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.pub \
		--program-id-file ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.vk \
		--beacon_url $(BEACON_URL) \
		--rpc_url $(RPC_URL)

verify_aggregated_proof_risc0: 
	@echo "Verifying RISC0 in aggregated proofs on $(NETWORK)..."
	@cd crates/cli/ && \
	cargo run verify-agg-proof \
		--network $(NETWORK) \
		--from-block $(FROM_BLOCK) \
		--proving_system Risc0 \
		--program-id-file ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_1_0.bin \
		--public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.pub \
		--beacon_url $(BEACON_URL) \
		--rpc_url $(RPC_URL)

proof_aggregator_install: ## Install the aggregation mode with proving enabled
	cargo install --path aggregation_mode --features prove,gpu --bin proof_aggregator --locked

proof_aggregator_write_program_ids: ## Write proof aggregator zkvm programs ids
	@cd aggregation_mode && ./scripts/build_programs.sh

__AGGREGATOR__: ## ____

aggregator_start: ## Start the Aggregator. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>, AGG_CONFIG_FILE
	$(GET_SDK_VERSION)
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(AGG_CONFIG_FILE) \
	2>&1 | zap-pretty

aggregator_start_ethereum_package: ## Start the Aggregator with Ethereum package config. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>, AGG_CONFIG_FILE
	$(MAKE) aggregator_start AGG_CONFIG_FILE=config-files/config-aggregator-ethereum-package.yaml

aggregator_build: ## Build the Aggregator. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>
	$(GET_SDK_VERSION)
	@echo "Building aggregator"
	@go build -o ./build/aligned-aggregator ./aggregator/cmd/main.go

aggregator_send_dummy_responses:
	@echo "Sending dummy responses to Aggregator..."
	@cd aggregator && go run dummy/submit_task_responses.go

test_go_retries:
	@cd core/ && \
	go test -v -timeout 15m

__OPERATOR__: ## ____

operator_start: ## Start the Operator. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>, CONFIG_FILE
	$(GET_SDK_VERSION)
	@echo "Starting Operator..."
	go run operator/cmd/main.go start --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

operator_start_ethereum_package: ## Start the Operator with Ethereum package config
	$(MAKE) operator_start ENVIRONMENT=devnet CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml

operator_set_eigen_sdk_go_version_testnet:
	@echo "Setting Eigen SDK version to: $(EIGEN_SDK_GO_VERSION_TESTNET)"
	go get github.com/Layr-Labs/eigensdk-go@$(EIGEN_SDK_GO_VERSION_TESTNET)

operator_set_eigen_sdk_go_version_devnet:
	@echo "Setting Eigen SDK version to: $(EIGEN_SDK_GO_VERSION_DEVNET)"
	go get github.com/Layr-Labs/eigensdk-go@$(EIGEN_SDK_GO_VERSION_DEVNET)

operator_set_eigen_sdk_go_version_mainnet:
	@echo "Setting Eigen SDK version to: $(EIGEN_SDK_GO_VERSION_MAINNET)"
	go get github.com/Layr-Labs/eigensdk-go@$(EIGEN_SDK_GO_VERSION_MAINNET)

operator_set_eigen_sdk_go_version_error:
	@echo "Error setting Eigen SDK version, missing ENVIRONMENT. Possible values for ENVIRONMENT=<devnet|testnet|mainnet>"
	exit 1

operator_full_registration: operator_get_eth operator_register_with_eigen_layer operator_mint_mock_tokens operator_deposit_into_mock_strategy operator_whitelist_devnet operator_register_with_aligned_layer ## Register the operator in EigenLayer and AlignedLayer. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>, CONFIG_FILE

operator_full_registration_and_start: $(GET_SDK_VERSION) operator_full_registration operator_start ## Register the operator in EigenLayer and AlignedLayer, then start the Operator. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>, CONFIG_FILE

operator_full_registration_and_start_ethereum_package: ## Register the operator in EigenLayer and AlignedLayer, then start the Operator with Ethereum package config
	$(MAKE) operator_full_registration CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml
	$(MAKE) operator_start ENVIRONMENT=devnet CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml


operator_build: deps ## Build the Operator. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>
	$(GET_SDK_VERSION)
	$(BUILD_OPERATOR)

operator_build_macos:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

operator_build_linux:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION) -r $(OPERATOR_FFIS)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

operator_update: ## Update the Operator to the latest version and build it. Parameters: ENVIRONMENT=<devnet|testnet|mainnet>
	$(GET_SDK_VERSION)
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

test:
	go test ./... -timeout 15m

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
	@echo "Operator address: $(OPERATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/operator_whitelist.sh $(OPERATOR_ADDRESS)

operator_remove_from_whitelist_devnet:
	@echo "Removing operator"
	@echo "Operator address: $(OPERATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/operator_remove_from_whitelist.sh $(OPERATOR_ADDRESS)

operator_whitelist:
	@echo "Whitelisting operator $(OPERATOR_ADDRESS)"
	@. contracts/scripts/.env && . contracts/scripts/operator_whitelist.sh $(OPERATOR_ADDRESS)

operator_remove_from_whitelist:
	@echo "Removing operator $(OPERATOR_ADDRESS)"
	@. contracts/scripts/.env && . contracts/scripts/operator_remove_from_whitelist.sh $(OPERATOR_ADDRESS)

operator_deposit_into_mock_strategy:
	@echo "Depositing into mock strategy"
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.addresses.strategies.WETH' contracts/script/output/devnet/eigenlayer_deployment_output.json))
	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount 100000000000000000

AMOUNT ?= 1000

operator_deposit_into_strategy:
	@echo "Depositing into strategy"
	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount $(AMOUNT)

operator_register_with_aligned_layer:
	@echo "Registering operator with AlignedLayer"
	@go run operator/cmd/main.go register \
		--config $(CONFIG_FILE)

__BATCHER__: ## ____

BURST_SIZE ?= 5

user_fund_payment_service:
	@. ./scripts/user_fund_payment_service_devnet.sh

./crates/batcher/.env:
	@echo "To start the Batcher ./crates/batcher/.env needs to be manually set"; false;

batcher_start: ./crates/batcher/.env user_fund_payment_service
	@echo "Starting Batcher..."
	@cargo run --manifest-path ./crates/batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./crates/batcher/.env

batcher_start_local: user_fund_payment_service ## Start the Batcher locally. It runs LocalStack as S3 service.
	@echo "Starting Batcher..."
	@$(MAKE) storage_start &
	@cargo run --manifest-path ./crates/batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./crates/batcher/.env.dev

batcher_start_local_no_fund:
	@echo "Starting Batcher..."
	@$(MAKE) storage_start &
	@cargo run --manifest-path ./crates/batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./crates/batcher/.env.dev

batcher_start_ethereum_package: user_fund_payment_service ## Start the Batcher with Ethereum package config. It runs LocalStack as S3 service.
	@echo "Starting Batcher..."
	@$(MAKE) storage_start &
	@cargo run --manifest-path ./crates/batcher/Cargo.toml --release -- --config ./config-files/config-batcher-ethereum-package.yaml --env-file ./crates/batcher/.env.dev


batcher_install: ## Install latest version of Batcher
	@cargo install --path crates/batcher

__STORAGE__: ## ____
storage_start: ## Run S3-storage using storage-docker-compose.yaml
	@echo "Running storage..."
	@docker compose -f storage-docker-compose.yaml up

__ALIGNED_CLI__: ## ____

aligned_install: ## Install latest version of Aligned CLI
	@./crates/cli/install_aligned.sh

aligned_uninstall: ## Uninstall Aligned CLI
	@rm -rf ~/.aligned && echo "Aligned uninstalled"

aligned_install_compiling: ## Install Aligned CLI by compiling from source
	@cargo install --path crates/cli

__SEND_PROOFS__: ## ____

crates/target/release/aligned:
	@cd crates/cli && cargo b --release

batcher_send_sp1_task: ## Send a SP1 fibonacci proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending SP1 fibonacci proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_sp1_burst: ## Send a burst of SP1 fibonacci proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending SP1 fibonacci proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_sp1_infinite: ## Send burst of SP1 fibonacci proofs to Batcher every certain time
	@echo "Sending infinite SP1 fibonacci proofs to Batcher..."
	@./crates/cli/send_infinite_sp1_tasks/send_infinite_sp1_tasks.sh

batcher_send_risc0_task: ## Send a Risc0 fibonacci proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Risc0 fibonacci proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.proof \
        --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_1_0.bin \
        --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.pub \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_risc0_task_no_pub_input: ## Send a Risc0 proof without public input to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Risc0 no pub input proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/no_public_inputs/risc_zero_no_pub_input_2_1_0.proof \
        --vm_program ../../scripts/test_files/risc_zero/no_public_inputs/risc_zero_no_pub_input_id_2_1_0.bin \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_risc0_burst: ## Send a burst of Risc0 fibonacci proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending Risc0 fibonacci proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.proof \
        --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_1_0.bin \
        --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.pub \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
        --repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_plonk_bn254_task: crates/target/release/aligned ## Send a Gnark Plonk Bn254 1!=0 proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Gnark Plonk Bn254 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_pub_input_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_plonk_bn254_burst: crates/target/release/aligned ## Send a burst of Gnark Plonk Bn254 1!=0 proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending Gnark Plonk Bn254 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_pub_input_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_plonk_bls12_381_task: crates/target/release/aligned ## Send a Gnark Plonk BLS12-381 1!=0 proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Gnark Plonk BLS12-381 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_pub_input_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_plonk_bls12_381_burst: crates/target/release/aligned ## Send a burst of Gnark Plonk BLS12-381 1!=0 proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending Gnark Plonk BLS12-381 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_pub_input_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_groth16_bn254_task: crates/target/release/aligned ## Send a Gnark Groth16 Bn254 1!=0 proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Gnark Groth 16Bn254 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkGroth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_gnark_groth16_bn254_burst: crates/target/release/aligned ## Send a burst of Gnark Groth16 Bn254 1!=0 proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending Gnark Groth16 Bn254 1!=0 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system GnarkGroth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

## TODO: send_burst_tasks.sh and send_infinite_tasks.sh does a similar thing. We could delete one
batcher_send_gnark_groth16_bn254_infinite: crates/target/release/aligned ## Send a different Gnark Groth16 BN254 proof using the client every 3 seconds. Parameters: BURST_SIZE, START_COUNTER
	@echo "Sending a burst of proofs to Batcher..."
	@mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs
	@./crates/cli/send_burst_tasks.sh $(BURST_SIZE) $(START_COUNTER)

batcher_send_circom_groth16_bn256_task: crates/target/release/aligned ## Send a Circom Groth16 BN256 proof to Batcher. Parameters: RPC_URL, NETWORK
	@echo "Sending Circom Groth16 BN256 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system CircomGroth16Bn256 \
		--proof ../../scripts/test_files/circom_groth16_bn256_script/proof.json \
		--public_input ../../scripts/test_files/circom_groth16_bn256_script/public.json \
		--vk ../../scripts/test_files/circom_groth16_bn256_script/verification_key.json \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_circom_groth16_bn256_burst: crates/target/release/aligned ## Send a burst of Circom Groth16 BN256 proofs to Batcher. Parameters: RPC_URL, NETWORK, BURST_SIZE
	@echo "Sending Circom Groth16 BN256 proof to Batcher..."
	@cd crates/cli/ && cargo run --release -- submit \
		--proving_system CircomGroth16Bn256 \
		--proof ../../scripts/test_files/circom_groth16_bn256_script/proof.json \
		--public_input ../../scripts/test_files/circom_groth16_bn256_script/public.json \
		--vk ../../scripts/test_files/circom_groth16_bn256_script/verification_key.json \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions $(BURST_SIZE) \
		--rpc_url $(RPC_URL) \
		--network $(NETWORK)

batcher_send_proof_with_random_address: ## Send a proof with a random address to Batcher. Parameters: RPC_URL, NETWORK, PROOF_TYPE, REPETITIONS
	@cd crates/cli/ && ./send_proof_with_random_address.sh

batcher_send_burst_with_random_address: ## Send a burst of proofs with random addresses to Batcher. Parameters: RPC_URL, NETWORK, PROOF_TYPE, REPETITIONS
	@cd crates/cli/ && ./send_burst_with_random_address.sh

__TASK_SENDER__:
BURST_TIME_SECS ?= 3

task_sender_generate_gnark_groth16_proofs:
	@cd crates/task-sender && \
	cargo run --release -- generate-proofs \
	--number-of-proofs $(NUMBER_OF_PROOFS) --proof-type gnark_groth16 \
	--dir-to-save-proofs $(CURDIR)/scripts/test_files/task_sender/proofs

# ===== DEVNET =====
task_sender_fund_wallets_devnet:
	@cd crates/task-sender && \
	cargo run --release -- generate-and-fund-wallets \
	--eth-rpc-url http://localhost:8545 \
	--network devnet \
	--amount-to-deposit 1 \
	--amount-to-deposit-to-aligned 0.9999 \
	--private-keys-filepath $(CURDIR)/crates/task-sender/wallets/devnet

task_sender_send_infinite_proofs_devnet:
	@cd crates/task-sender && \
	cargo run --release -- send-infinite-proofs \
	--burst-size $(BURST_SIZE) --burst-time-secs $(BURST_TIME_SECS) \
	--eth-rpc-url http://localhost:8545 \
	--network devnet \
	--proofs-dirpath $(CURDIR)/scripts/test_files/task_sender/proofs \
	--private-keys-filepath $(CURDIR)/crates/task-sender/wallets/devnet

task_sender_test_connections_devnet:
	@cd crates/task-sender && \
	cargo run --release -- test-connections \
	--num-senders $(NUM_SENDERS) \
	--network devnet

# ===== HOLESKY-STAGE =====
task_sender_generate_and_fund_wallets_holesky_stage:
	@cd crates/task-sender && \
	cargo run --release -- generate-and-fund-wallets \
	--eth-rpc-url https://ethereum-holesky-rpc.publicnode.com \
	--network holesky-stage \
	--funding-wallet-private-key $(FUNDING_WALLET_PRIVATE_KEY) \
	--number-wallets $(NUM_WALLETS) \
	--amount-to-deposit $(AMOUNT_TO_DEPOSIT) \
	--amount-to-deposit-to-aligned $(AMOUNT_TO_DEPOSIT_TO_ALIGNED) \
	--private-keys-filepath $(CURDIR)/crates/task-sender/wallets/holesky-stage

task_sender_send_infinite_proofs_holesky_stage:
	@cd crates/task-sender && \
	cargo run --release -- send-infinite-proofs \
	--burst-size $(BURST_SIZE) --burst-time-secs $(BURST_TIME_SECS) \
	--eth-rpc-url https://ethereum-holesky-rpc.publicnode.com \
	--network holesky-stage \
	--proofs-dirpath $(CURDIR)/scripts/test_files/task_sender/proofs \
	--private-keys-filepath $(CURDIR)/crates/task-sender/wallets/holesky-stage

task_sender_test_connections_holesky_stage:
	@cd crates/task-sender && \
	cargo run --release -- test-connections \
	--num-senders $(NUM_SENDERS) \
	--network holesky-stage

__UTILS__:
aligned_get_user_balance_devnet:
	@cd crates/cli/ && cargo run --release -- get-user-balance \
		--user_addr $(USER_ADDR) \
		--network devnet

aligned_get_user_balance_holesky:
	@cd crates/cli/ && cargo run --release -- get-user-balance \
		--rpc_url https://ethereum-holesky-rpc.publicnode.com \
		--network holesky \
		--user_addr $(USER_ADDR)

__GENERATE_PROOFS__: ## ____
generate_sp1_fibonacci_proof: ## Run the SP1 Fibonacci proof generator script
	@cd scripts/test_files/sp1/fibonacci_proof_generator/script && RUST_LOG=info cargo run --release
	@echo "Fibonacci proof and ELF generated in scripts/test_files/sp1 folder"

generate_risc_zero_fibonacci_proof: ## Run the Risc0 Fibonacci proof generator script
	@cd scripts/test_files/risc_zero/fibonacci_proof_generator && \
	RUST_LOG=info cargo run --release && \
	echo "Fibonacci proof, pub input and image ID generated in scripts/test_files/risc_zero folder"

generate_risc_zero_empty_journal_proof: ## Run the Risc0 Fibonacci proof generator script with empty journal
	@cd scripts/test_files/risc_zero/no_public_inputs && RUST_LOG=info cargo run --release
	@echo "Fibonacci proof and ELF with empty journal generated in scripts/test_files/risc_zero/no_public_inputs folder"

generate_gnark_plonk_bls12_381_proof: ## Run the gnark_plonk_bls12_381_script
	@echo "Running gnark_plonk_bls12_381 script..."
	@go run scripts/test_files/gnark_plonk_bls12_381_script/main.go

generate_gnark_plonk_bn254_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_plonk_bn254 script..."
	@go run scripts/test_files/gnark_plonk_bn254_script/main.go

generate_gnark_groth16_bn254_proof: ## Run the gnark_groth16_bn254_script
	@echo "Running gnark_groth_bn254 script..."
	@go run scripts/test_files/gnark_groth16_bn254_script/main.go

generate_gnark_groth16_bn254_ineq_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_groth_bn254_ineq script..."
	@go run scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go 1

generate_circom_groth16_bn256_proof: ## Run the circom_groth16_bn256_script
	@echo "Running circom_groth16_bn256 script..."
	@cd scripts/test_files/circom_groth16_bn256_script && ./generate_proof.sh

generate_circom_groth16_bn256_setup: ## Run the circom_groth16_bn256_script setup
	@echo "Running circom_groth16_bn256 script setup..."
	@cd scripts/test_files/circom_groth16_bn256_script && ./generate_setup.sh

__CONTRACTS_DEPLOYMENT__: ## ____
deploy_aligned_contracts: ## Deploy Aligned Contracts. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Deploying Aligned Contracts on $(NETWORK) network..."
	@. co	ntracts/scripts/.env.$(NETWORK) && . contracts/scripts/deploy_aligned_contracts.sh

deploy_pauser_registry: ## Deploy Pauser Registry
	@echo "Deploying Pauser Registry..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_pauser_registry.sh

upgrade_aligned_contracts: ## Upgrade Aligned Contracts. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Upgrading Aligned Contracts on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/upgrade_aligned_contracts.sh

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

set_aggregator_address:
	@echo "Setting Aggregator Address in Aligned Service Manager Contract on $(NETWORK) network..."
	@echo "Aggregator address: $(AGGREGATOR_ADDRESS)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/set_aggregator_address.sh $(AGGREGATOR_ADDRESS)

set_aggregator_address_devnet:
	@echo "Setting Aggregator Address in Aligned Service Manager Contract..."
	@echo "Aggregator address: $(AGGREGATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/set_aggregator_address.sh $(AGGREGATOR_ADDRESS)

upgrade_initialize_disabled_verifiers:
	@echo "Adding disabled verifiers to Aligned Service Manager..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_disabled_verifiers_in_service_manager.sh

deploy_verify_batch_inclusion_caller:
	@echo "Deploying VerifyBatchInclusionCaller contract..."
	@. examples/verify/.env && . examples/verify/scripts/deploy_verify_batch_inclusion_caller.sh

deploy_batcher_payment_service: ## Deploy BatcherPayments contract. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Deploying BatcherPayments contract on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/deploy_batcher_payment_service.sh

upgrade_batcher_payment_service: ## Upgrade BatcherPayments contract. Parameters: NETWORK=<mainnet|holesky|sepolia
	@echo "Upgrading BatcherPayments Contract on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/upgrade_batcher_payment_service.sh

deploy_proof_aggregator: ## Deploy ProofAggregator contract. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Deploying ProofAggregator contract on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/deploy_proof_aggregator.sh

upgrade_proof_aggregator: ## Upgrade ProofAggregator contract. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Upgrading ProofAggregator Contract on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/upgrade_proof_aggregator.sh

__SP1_FFI__: ##
build_sp1_macos:
	@cd operator/sp1/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_ffi.dylib operator/sp1/lib/libsp1_verifier_ffi.dylib

build_sp1_linux:
	@cd operator/sp1/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_ffi.so operator/sp1/lib/libsp1_verifier_ffi.so

test_sp1_rust_ffi:
	@echo "Testing SP1 Rust FFI source code..."
	@cd operator/sp1/lib && RUST_MIN_STACK=83886080 cargo t --release

test_sp1_go_bindings_macos: build_sp1_macos
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

test_sp1_go_bindings_linux: build_sp1_linux
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

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

__FFI__: ## ____

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

__EXPLORER__: ## ____

explorer_start: explorer_start_db explorer_ecto_setup_db ## Start the Explorer with the database
	@cd explorer/ && \
		pnpm install --prefix assets && \
		mix setup && \
		./start.sh

explorer_build_db: ## Build the Explorer database image
	@cd explorer && \
		docker build -t explorer-postgres-image .

explorer_start_db: explorer_remove_db_container
	@cd explorer && \
		docker run -d --name explorer-postgres-container -p 5432:5432 -v explorer-postgres-data:/var/lib/postgresql/data explorer-postgres-image

explorer_ecto_setup_db:
		@cd explorer/ && \
		./ecto_setup_db.sh

explorer_remove_db_container:
	@cd explorer && \
		docker stop explorer-postgres-container || true  && \
		docker rm explorer-postgres-container || true

explorer_clean_db: explorer_remove_db_container ## Remove the Explorer database container and volume
	@cd explorer && \
		docker volume rm explorer-postgres-data || true

explorer_dump_db: ## Dump the Explorer database to a file
	@cd explorer && \
		docker exec -t explorer-postgres-container pg_dumpall -c -U explorer_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /explorer"

explorer_recover_db: explorer_start_db ## Recover the Explorer database from a dump file
	@read -p $$'\e[32mEnter the dump file to recover (e.g., dump.20230607_123456.sql): \e[0m' DUMP_FILE && \
	cd explorer && \
	docker cp $$DUMP_FILE explorer-postgres-container:/dump.sql && \
	docker exec -t explorer-postgres-container psql -U explorer_user -d explorer_db -f /dump.sql && \
	echo "Recovered database successfully from $$DUMP_FILE"

explorer_fetch_old_batches:
	@cd explorer && \
	./scripts/fetch_old_batches.sh $(FROM_BLOCK) $(TO_BLOCK)

explorer_fetch_old_operators_strategies_restakes: # recommended for prod: 19000000
	@cd explorer && \
	./scripts/fetch_old_operators_strategies_restakes.sh $(FROM_BLOCK)

explorer_create_env:
	@cd explorer && \
	cp .env.dev .env

DOCKER_RPC_URL=http://anvil:8545
PROOF_GENERATOR_ADDRESS=0x66f9664f97F2b50F62D13eA064982f936dE76657

docker_build_base_image:
	docker compose -f docker-compose.yaml --profile aligned_base build

docker_build_aggregator:
	docker compose -f docker-compose.yaml --profile aggregator build

docker_build_operator:
	docker compose -f docker-compose.yaml --profile operator build

docker_build_batcher:
	docker compose -f docker-compose.yaml --profile batcher build

docker_restart_aggregator:
	docker compose -f docker-compose.yaml --profile aggregator down
	docker compose -f docker-compose.yaml --profile aggregator up -d --remove-orphans --force-recreate

docker_restart_operator:
	docker compose -f docker-compose.yaml --profile operator down
	docker compose -f docker-compose.yaml --profile operator up -d --remove-orphans --force-recreate

docker_restart_batcher:
	docker compose -f docker-compose.yaml --profile batcher down
	docker compose -f docker-compose.yaml --profile batcher up -d --remove-orphans --force-recreate

docker_build:
	docker compose -f docker-compose.yaml --profile aligned_base build
	docker compose -f docker-compose.yaml --profile eigenlayer-cli build
	docker compose -f docker-compose.yaml --profile foundry build
	docker compose -f docker-compose.yaml --profile base build
	docker compose -f docker-compose.yaml --profile operator build
	docker compose -f docker-compose.yaml --profile batcher build
	docker compose -f docker-compose.yaml --profile aggregator build

docker_up:
	docker compose -f docker-compose.yaml --profile base up -d --remove-orphans --force-recreate
	@until [ "$$(docker inspect $$(docker ps | grep anvil | awk '{print $$1}') | jq -r '.[0].State.Health.Status')" = "healthy" ]; do sleep .5; done; sleep 2
	docker compose -f docker-compose.yaml --profile aggregator up -d --remove-orphans --force-recreate
	docker compose -f docker-compose.yaml run --rm fund-operator
	docker compose -f docker-compose.yaml run --rm register-operator-eigenlayer
	docker compose -f docker-compose.yaml run --rm mint-mock-tokens
	docker compose -f docker-compose.yaml run --rm operator-deposit-into-mock-strategy
	docker compose -f docker-compose.yaml run --rm operator-whitelist-devnet
	docker compose -f docker-compose.yaml run --rm operator-register-with-aligned-layer
	docker compose -f docker-compose.yaml --profile operator up -d --remove-orphans --force-recreate
	docker compose -f docker-compose.yaml run --rm user-fund-payment-service-devnet
	docker compose -f docker-compose.yaml --profile batcher up -d --remove-orphans --force-recreate
	@echo "Up and running"

docker_down:
	docker compose -f docker-compose.yaml --profile batcher down
	docker compose -f docker-compose.yaml --profile operator down
	docker compose -f docker-compose.yaml --profile base down
	@echo "Everything down"
	docker ps

DOCKER_BURST_SIZE=1
DOCKER_PROOFS_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

docker_batcher_send_sp1_burst:
	@echo "Sending SP1 fibonacci task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system SP1 \
              --proof ./scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
              --vm_program ./scripts/test_files/sp1/sp1_fibonacci_5_0_0.elf \
              --repetitions $(DOCKER_BURST_SIZE) \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_risc0_burst:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system Risc0 \
              --proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.proof \
              --vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_2_1_0.bin \
              --public_input ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_2_1_0.pub \
              --repetitions $(DOCKER_BURST_SIZE) \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_gnark_plonk_bn254_burst:
	@echo "Sending Gnark Plonk Bn254 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system GnarkPlonkBn254 \
              --proof ./scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.proof \
              --public_input ./scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_pub_input_0_12_0.pub \
              --vk ./scripts/test_files/gnark_plonk_bn254_script/gnark_plonk_0_12_0.vk \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
              --repetitions $(DOCKER_BURST_SIZE) \
			  --max_fee 0.1ether

docker_batcher_send_gnark_plonk_bls12_381_burst:
	@echo "Sending Gnark Plonk BLS12-381 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system GnarkPlonkBls12_381 \
              --proof ./scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.proof \
              --public_input ./scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_pub_input_0_12_0.pub \
              --vk ./scripts/test_files/gnark_plonk_bls12_381_script/gnark_plonk_0_12_0.vk \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --repetitions $(DOCKER_BURST_SIZE) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_gnark_groth16_burst:
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
            --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
			--proving_system GnarkGroth16Bn254 \
			--proof ./scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.proof \
			--public_input ./scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.pub \
			--vk ./scripts/test_files/gnark_groth16_bn254_script/gnark_groth16_0_12_0.vk \
			--proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
			--repetitions $(DOCKER_BURST_SIZE) \
			--rpc_url $(DOCKER_RPC_URL) \
			--max_fee 0.1ether

docker_batcher_send_circom_groth16_bn256_burst:
	@echo "Sending Circom Groth16 BN256 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
			  --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
			  --proving_system CircomGroth16Bn256 \
			  --proof ./scripts/test_files/circom_groth16_bn256_script/proof.json \
			  --public_input ./scripts/test_files/circom_groth16_bn256_script/public.json \
			  --vk ./scripts/test_files/circom_groth16_bn256_script/verification_key.json \
			  --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
			  --repetitions $(DOCKER_BURST_SIZE) \
			  --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

# Update target as new proofs are supported.
docker_batcher_send_all_proofs_burst:
	@$(MAKE) docker_batcher_send_sp1_burst
	@$(MAKE) docker_batcher_send_risc0_burst
	@$(MAKE) docker_batcher_send_gnark_plonk_bn254_burst
	@$(MAKE) docker_batcher_send_gnark_plonk_bls12_381_burst
	@$(MAKE) docker_batcher_send_gnark_groth16_burst
	@$(MAKE) docker_batcher_send_circom_groth16_bn256_burst

docker_batcher_send_infinite_groth16:
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') \
	sh -c ' \
		mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs; \
	  counter=1; \
	  timer=3; \
	  while true; do \
	    echo "Generating proof $${counter} != 0"; \
	    gnark_groth16_bn254_infinite_script $${counter}; \
	    aligned submit \
	              --rpc_url $(DOCKER_RPC_URL) \
	              --repetitions $(DOCKER_BURST_SIZE) \
	              --proving_system GnarkGroth16Bn254 \
	              --proof scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_gnark_groth16_0_12_0.proof \
	              --public_input scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_gnark_groth16_0_12_0.pub \
	              --vk scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_gnark_groth16_0_12_0.vk \
	              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS); \
				  --max_fee 0.1ether
	    sleep $${timer}; \
	    counter=$$((counter + 1)); \
	  done \
	'

docker_verify_proofs_onchain:
	@echo "Verifying proofs..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') \
	sh -c ' \
	    for proof in ./aligned_verification_data/*.cbor; do \
			  echo "Verifying $${proof}"; \
	      aligned verify-proof-onchain \
	                --aligned-verification-data $${proof} \
	                --rpc_url $(DOCKER_RPC_URL); \
	    done \
	  '

DOCKER_PROOFS_WAIT_TIME=60
DOCKER_SENT_PROOFS=6

docker_verify_proof_submission_success: 
	@echo "Verifying proofs were successfully submitted..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') \
	sh -c ' \
			if [ -z "$$(ls -A ./aligned_verification_data)" ]; then echo "ERROR: There are no proofs on aligned_verification_data/ directory" && exit 1; fi; \
			echo "Waiting $(DOCKER_PROOFS_WAIT_TIME) seconds before starting proof verification. \n"; \
			sleep $(DOCKER_PROOFS_WAIT_TIME); \
			for proof in ./aligned_verification_data/*.cbor; do \
				echo "Verifying proof $${proof} \n"; \
				verification=$$(aligned verify-proof-onchain \
									--aligned-verification-data $${proof} \
									--rpc_url $$(echo $(DOCKER_RPC_URL)) 2>&1); \
				cat $${proof%.cbor}.json; \
				echo "$$verification"; \
				if echo "$$verification" | grep -q not; then \
					echo "ERROR: Proof verification failed for $${proof}"; \
					exit 1; \
				elif echo "$$verification" | grep -q verified; then \
					echo "Proof verification succeeded for $${proof}"; \
				else \
					echo "WARNING: Unexpected verification result for $${proof}"; \
					echo "Output:"; \
					echo "$$verification"; \
					exit 1; \
				fi; \
				echo "---------------------------------------------------------------------------------------------------"; \
			done; \
			if [ $$(ls -1 ./aligned_verification_data/*.cbor | wc -l) -ne $(DOCKER_SENT_PROOFS) ]; then \
				echo "ERROR: Some proofs were verified successfully, but some proofs are missing in the aligned_verification_data/ directory"; \
				exit 1; \
			fi; \
			echo "All proofs verified successfully!"; \
		'

docker_attach_foundry:
	docker exec -ti $(shell docker ps | grep anvil | awk '{print $$1}') /bin/bash

docker_attach_anvil:
	docker exec -ti $(shell docker ps | grep anvil | awk '{print $$1}') /bin/bash

docker_attach_aggregator:
	docker exec -ti $(shell docker ps | grep aggregator | awk '{print $$1}') /bin/bash

docker_attach_operator:
	docker exec -ti $(shell docker ps | grep operator | awk '{print $$1}') /bin/bash

docker_attach_batcher:
	docker exec -ti $(shell docker ps | grep batcher | awk '{print $$1}') /bin/bash

docker_logs_anvil:
	docker compose -f docker-compose.yaml logs anvil -f

docker_logs_aggregator:
	docker compose -f docker-compose.yaml logs aggregator -f

docker_logs_operator:
	docker compose -f docker-compose.yaml logs operator -f

docker_logs_batcher:
	docker compose -f docker-compose.yaml logs batcher -f

__TELEMETRY__: ## ____
# TODO maybe add a target to run both metrics and telemetry

metrics_start: ## Run metrics (prometheus, grafana) using metrics-docker-compose.yaml
	@echo "Running metrics..."
	@docker compose -f metrics-docker-compose.yaml up

metrics_remove_containers: ## Remove Prometheus and Grafana containers
	@docker stop prometheus grafana
	@docker rm prometheus grafana

metrics_clean_db: metrics_remove_containers ## Remove Prometheus and Grafana volumes
	@docker volume rm aligned_layer_grafana_data aligned_layer_prometheus_data

telemetry_start_all: telemetry_compile_bls_verifier open_telemetry_start telemetry_start ## Run all telemetry services (open telemetry, telemetry API)

open_telemetry_start: ## Run open telemetry services (otel collector, jaeger, cassandra) using telemetry-docker-compose.yaml
	@echo "Running telemetry..."
	@docker compose -f telemetry-docker-compose.yaml up -d

open_telemetry_prod_start: # TODO check if we are using this target
	@echo "Running telemetry for Prod..."
	@docker compose -f telemetry-prod-docker-compose.yaml up -d

telemetry_start: telemetry_start_db telemetry_ecto_migrate ## Run Telemetry API
	@cd telemetry_api && \
	 	./start.sh

telemetry_ecto_migrate: ## Run Ecto migrations for Telemetry API
		@cd telemetry_api && \
			./ecto_setup_db.sh

telemetry_build_db: ## Build the Telemetry database image
	@cd telemetry_api && \
		docker build -t telemetry-postgres-image .

telemetry_start_db: telemetry_build_db telemetry_remove_db_container ## Run the Telemetry database container
	@cd telemetry_api && \
		docker run -d --name telemetry-postgres-container -p 5434:5432 -v telemetry-postgres-data:/var/lib/postgresql/data telemetry-postgres-image

telemetry_remove_db_container: ## Remove the Telemetry database container
	@docker stop telemetry-postgres-container || true  && \
	    docker rm telemetry-postgres-container || true

telemetry_clean_db: telemetry_remove_db_container ## Remove the Telemetry database container and volume
	@docker volume rm telemetry-postgres-data || true

telemetry_dump_db: ## Dump the Telemetry database to a file
	@cd telemetry_api && \
		docker exec -t telemetry-postgres-container pg_dumpall -c -U telemetry_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /telemetry_api"

telemetry_create_env:
	@cd telemetry_api && \
		cp .env.dev .env

telemetry_compile_bls_verifier: ## Compile the BLS verifier for Telemetry API
	@cd telemetry_api/priv && \
	go build ../bls_verifier/bls_verify.go

setup_local_aligned_all: # TODO check if we are using this target
	tmux kill-session -t aligned_layer || true
	tmux new-session -d -s aligned_layer

	tmux new-window -t aligned_layer -n anvil
	tmux send-keys -t aligned_layer 'make anvil_start' C-m

	tmux new-window -t aligned_layer -n aggregator
	tmux send-keys -t aligned_layer:aggregator 'make aggregator_start' C-m

	tmux new-window -t aligned_layer -n operator
	tmux send-keys -t aligned_layer:operator 'sleep 5 && make operator_full_registration_and_start' C-m

	tmux new-window -t aligned_layer -n batcher
	tmux send-keys -t aligned_layer:batcher 'sleep 60 && make batcher_start_local' C-m

	tmux new-window -t aligned_layer -n explorer
	tmux send-keys -t aligned_layer:explorer 'make explorer_create_env && make explorer_build_db && make explorer_start' C-m

	tmux new-window -t aligned_layer -n telemetry
	tmux send-keys -t aligned_layer:telemetry 'docker compose -f telemetry-docker-compose.yaml down && make telemetry_create_env && make telemetry_start_db && make open_telemetry_start && make telemetry_start' C-m

__ANSIBLE__: ## ____

ansible_batcher_create_env: ## Create empty variables files for the Batcher deploy
	@cp -n infra/ansible/playbooks/ini/caddy-batcher.ini.example infra/ansible/playbooks/ini/caddy-batcher.ini
	@cp -n infra/ansible/playbooks/ini/config-batcher.ini.example infra/ansible/playbooks/ini/config-batcher.ini
	@cp -n infra/ansible/playbooks/ini/env-batcher.ini.example infra/ansible/playbooks/ini/env-batcher.ini
	@echo "Config files for the Batcher created in infra/ansible/playbooks/ini"
	@echo "Please complete the values and run make ansible_batcher_deploy"

ansible_batcher_deploy: ## Deploy the Batcher. Parameters: INVENTORY, KEYSTORE
	@if [ -z "$(INVENTORY)" ] || [ -z "$(KEYSTORE)" ]; then \
		echo "Error: Both INVENTORY and KEYSTORE must be set."; \
		exit 1; \
	fi
	@ansible-playbook infra/ansible/playbooks/batcher.yaml \
		-i $(INVENTORY) \
		-e "keystore_path=$(KEYSTORE)"

ansible_aggregator_create_env: ## Create empty variables files for the Aggregator deploy
	@cp -n infra/ansible/playbooks/ini/config-aggregator.ini.example infra/ansible/playbooks/ini/config-aggregator.ini
	@echo "Config files for the Aggregator created in infra/ansible/playbooks/ini"
	@echo "Please complete the values and run make ansible_aggregator_deploy"

ansible_aggregator_deploy: ## Deploy the Operator. Parameters: INVENTORY
	@if [ -z "$(INVENTORY)" ] || [ -z "$(ECDSA_KEYSTORE)" ] || [ -z "$(BLS_KEYSTORE)" ]; then \
		echo "Error: INVENTORY, ECDSA_KEYSTORE, BLS_KEYSTORE must be set."; \
		exit 1; \
	fi
	@ansible-playbook infra/ansible/playbooks/aggregator.yaml \
		-i $(INVENTORY) \
		-e "ecdsa_keystore_path=$(ECDSA_KEYSTORE)" \
		-e "bls_keystore_path=$(BLS_KEYSTORE)"

ansible_operator_create_env: ## Create empty variables files for the Operator deploy
	@cp -n infra/ansible/playbooks/ini/config-operator.ini.example infra/ansible/playbooks/ini/config-operator.ini
	@cp -n infra/ansible/playbooks/ini/config-register-operator.ini.example infra/ansible/playbooks/ini/config-register-operator.ini
	@echo "Config files for the Operator created in infra/ansible/playbooks/ini"
	@echo "Please complete the values and run make ansible_operator_deploy"

ansible_operator_deploy: ## Deploy the Operator. Parameters: INVENTORY
	@if [ -z "$(INVENTORY)" ]  || [ -z "$(ECDSA_KEYSTORE)" ]  || [ -z "$(BLS_KEYSTORE)" ]; then \
		echo "Error: INVENTORY, ECDSA_KEYSTORE, BLS_KEYSTORE must be set."; \
		exit 1; \
	fi
	@ansible-playbook infra/ansible/playbooks/operator.yaml \
		-i $(INVENTORY) \
		-e "ecdsa_keystore_path=$(ECDSA_KEYSTORE)" \
		-e "bls_keystore_path=$(BLS_KEYSTORE)"

ansible_explorer_deploy: ## Deploy the Explorer. Parameters: INVENTORY
	@ansible-playbook infra/ansible/playbooks/explorer.yaml \
		-i $(INVENTORY)

ansible_telemetry_create_env: ## Create empty variables files for the Telemetry deploy
	@cp -n infra/ansible/playbooks/ini/config-telemetry.ini.example infra/ansible/playbooks/ini/config-telemetry.ini
	@echo "Config files for Telemetry created in infra/ansible/playbooks/ini"
	@echo "Please complete the values and run make ansible_telemetry_deploy"

ansible_telemetry_deploy: ## Deploy the Telemetry. Parameters: INVENTORY
	@ansible-playbook infra/ansible/playbooks/telemetry.yaml \
		-i $(INVENTORY)

__ETHEREUM_PACKAGE__:  ## ____

ethereum_package_start: ## Starts the ethereum_package environment
	kurtosis run --enclave aligned github.com/ethpandaops/ethereum-package --args-file network_params.yaml

ethereum_package_inspect: ## Prints detailed information about the net
	kurtosis enclave inspect aligned

ethereum_package_rm: ## Stops and removes the ethereum_package environment and used resources
	kurtosis enclave rm aligned -f
	kurtosis engine stop

spamoor_install: ## Instal spamoor to spam transactions
	@echo "Installing spamoor..."
	@git clone https://github.com/ethpandaops/spamoor.git
	@cd spamoor && make
	@mv spamoor/bin/spamoor $(HOME)/.local/bin
	@rm -rf spamoor
	@echo "======================================================================="
	@echo "Installation complete! Run 'spamoor --help' to verify the installation."
	@echo "If 'spamoor' is not recognized, make sure it's in your PATH by adding the following line to your shell configuration:"
	@echo "export PATH=\$$PATH:\$$HOME/.local/bin"
	@echo "======================================================================="

# Spamoor funding wallet
SPAMOOR_PRIVATE_KEY?=dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97
NUM_WALLETS?=1000
TX_PER_BLOCK?=250
# Similar to a swap
TX_CONSUMES_GAS?=150000

spamoor_send_transactions: ## Sends normal transactions and also replacement transactions
	spamoor gasburnertx -p $(SPAMOOR_PRIVATE_KEY) -c $(COUNT) \
		--gas-units-to-burn $(TX_CONSUMES_GAS) \
		--max-wallets $(NUM_WALLETS) --max-pending $(TX_PER_BLOCK) \
		-t $(TX_PER_BLOCK) -h http://127.0.0.1:8545/ -h http://127.0.0.1:8550/ -h http://127.0.0.1:8555/ -h http://127.0.0.1:8565/ \
		--refill-amount 5 --refill-balance 2 --tipfee $(TIP_FEE) --basefee 100  \
		2>&1 | grep -v 'checked child wallets (no funding needed)'

__NODE_EXPORTER_:

install_node_exporter:
	@./scripts/install_node_exporter.sh
