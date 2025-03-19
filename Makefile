.PHONY: help tests

SHELL := /bin/bash
OS := $(shell uname -s)

CONFIG_FILE?=config-files/config.yaml
export OPERATOR_ADDRESS ?= $(shell yq -r '.operator.address' $(CONFIG_FILE))
AGG_CONFIG_FILE?=config-files/config-aggregator.yaml

OPERATOR_VERSION=v0.14.0
EIGEN_SDK_GO_VERSION_TESTNET=v0.2.0-beta.1
EIGEN_SDK_GO_VERSION_MAINNET=v0.1.13

ifeq ($(OS),Linux)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_linux
endif

ifeq ($(OS),Darwin)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_macos
endif

ifeq ($(OS),Linux)
	export LD_LIBRARY_PATH+=$(CURDIR)/operator/risc_zero_old/lib:$(CURDIR)/operator/risc_zero/lib
	OPERATOR_FFIS=$(CURDIR)/operator/risc_zero_old/lib:$(CURDIR)/operator/risc_zero/lib
endif

ifeq ($(OS),Linux)
	BUILD_OPERATOR = $(MAKE) build_operator_linux 
endif

ifeq ($(OS),Darwin)
	BUILD_OPERATOR = $(MAKE) build_operator_macos
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
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

submodules:
	git submodule update --init --recursive
	@echo "Updated submodules"

deps: submodules go_deps build_all_ffi ## Install deps

go_deps:
	@echo "Installing Go dependencies..."
	go install github.com/maoueh/zap-pretty@v0.3.0
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
	
anvil_upgrade_initialize_disable_verifiers:
	@echo "Initializing disabled verifiers..."
	. contracts/scripts/anvil/upgrade_disabled_verifiers_in_service_manager.sh

lint_contracts:
	@cd contracts && npm run lint:sol

anvil_start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json

anvil_start_with_block_time:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7

anvil_start_with_block_time_with_more_prefunded_accounts:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7 -a 2000

_AGGREGATOR_:

build_aggregator:
	$(GET_SDK_VERSION)
	@echo "Building aggregator"
	@go build -o ./build/aligned-aggregator ./aggregator/cmd/main.go

aggregator_start:
	$(GET_SDK_VERSION)
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(AGG_CONFIG_FILE) \
	2>&1 | zap-pretty

aggregator_send_dummy_responses:
	@echo "Sending dummy responses to Aggregator..."
	@cd aggregator && go run dummy/submit_task_responses.go

test_go_retries:
	@cd core/ && \
	go test -v -timeout 15m

__OPERATOR__:

operator_start:
	$(GET_SDK_VERSION)
	@echo "Starting Operator..."
	go run operator/cmd/main.go start --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

operator_set_eigen_sdk_go_version_testnet:
	@echo "Setting Eigen SDK version to: $(EIGEN_SDK_GO_VERSION_TESTNET)"
	go get github.com/Layr-Labs/eigensdk-go@$(EIGEN_SDK_GO_VERSION_TESTNET)

operator_set_eigen_sdk_go_version_devnet: operator_set_eigen_sdk_go_version_mainnet

operator_set_eigen_sdk_go_version_mainnet:
	@echo "Setting Eigen SDK version to: $(EIGEN_SDK_GO_VERSION_MAINNET)"
	go get github.com/Layr-Labs/eigensdk-go@$(EIGEN_SDK_GO_VERSION_MAINNET)

operator_set_eigen_sdk_go_version_error:
	@echo "Error setting Eigen SDK version, missing ENVIRONMENT. Possible values for ENVIRONMENT=<devnet|testnet|mainnet>"
	exit 1

operator_full_registration: operator_get_eth operator_register_with_eigen_layer operator_mint_mock_tokens operator_deposit_into_mock_strategy operator_whitelist_devnet operator_register_with_aligned_layer

operator_register_and_start: operator_full_registration operator_start

build_operator: deps
	$(GET_SDK_VERSION)
	$(BUILD_OPERATOR)

build_operator_macos:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

build_operator_linux:
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION) -r $(OPERATOR_FFIS)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

update_operator:
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

bindings:
	cd contracts && ./generate-go-bindings.sh

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
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.addresses.strategies.MOCK' contracts/script/output/devnet/eigenlayer_deployment_output.json))
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

operator_deposit_and_register: operator_deposit_into_strategy operator_register_with_aligned_layer


# The verifier ID to enable or disable corresponds to the index of the verifier in the `ProvingSystemID` enum.
verifier_enable_devnet:
	@echo "Enabling verifier with id: $(VERIFIER_ID)"
	PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 RPC_URL=http://localhost:8545 OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/enable_verifier.sh $(VERIFIER_ID)

verifier_disable_devnet:
	@echo "Disabling verifier with id: $(VERIFIER_ID)"
	PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 RPC_URL=http://localhost:8545 OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/disable_verifier.sh $(VERIFIER_ID)

verifier_enable:
	@echo "Enabling verifier with ID: $(VERIFIER_ID)"
	@. contracts/scripts/.env && . contracts/scripts/enable_verifier.sh $(VERIFIER_ID)

verifier_disable:
	@echo "Disabling verifier with ID: $(VERIFIER_ID)"
	@. contracts/scripts/.env && . contracts/scripts/disable_verifier.sh $(VERIFIER_ID)

strategies_get_weight:
	@echo "Getting weight of strategy: $(STRATEGY_INDEX)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/get_strategy_weight.sh $(STRATEGY_INDEX)

strategies_update_weight:
	@echo "Updating strategy weights: "
	@echo "STRATEGY_INDICES: $(STRATEGY_INDICES)"
	@echo "NEW_MULTIPLIERS: $(NEW_MULTIPLIERS)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/update_strategy_weight.sh $(STRATEGY_INDICES) $(NEW_MULTIPLIERS)

strategies_remove:
	@echo "Removing strategies: $(INDICES_TO_REMOVE)"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/remove_strategy.sh $(INDICES_TO_REMOVE)

strategies_get_addresses:
	@echo "Getting strategy addresses"
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/get_restakeable_strategies.sh

__BATCHER__:

BURST_SIZE ?= 5

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

batcher_start_local_no_fund:
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
		--network $(NETWORK)

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
		--repetitions $(BURST_SIZE) \
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


__TASK_SENDER__:
BURST_TIME_SECS ?= 3

task_sender_generate_groth16_proofs:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- generate-proofs \
	--number-of-proofs $(NUMBER_OF_PROOFS) --proof-type groth16 \
	--dir-to-save-proofs $(CURDIR)/scripts/test_files/task_sender/proofs

# ===== DEVNET =====
task_sender_fund_wallets_devnet:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- generate-and-fund-wallets \
	--eth-rpc-url http://localhost:8545 \
	--network devnet \
	--amount-to-deposit 1 \
	--amount-to-deposit-to-aligned 0.9999 \
	--private-keys-filepath $(CURDIR)/batcher/aligned-task-sender/wallets/devnet

task_sender_send_infinite_proofs_devnet:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- send-infinite-proofs \
	--burst-size $(BURST_SIZE) --burst-time-secs $(BURST_TIME_SECS) \
	--eth-rpc-url http://localhost:8545 \
	--network devnet \
	--proofs-dirpath $(CURDIR)/scripts/test_files/task_sender/proofs \
	--private-keys-filepath $(CURDIR)/batcher/aligned-task-sender/wallets/devnet

task_sender_test_connections_devnet:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- test-connections \
	--num-senders $(NUM_SENDERS) \
	--network devnet

# ===== HOLESKY-STAGE =====
task_sender_generate_and_fund_wallets_holesky_stage:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- generate-and-fund-wallets \
	--eth-rpc-url https://ethereum-holesky-rpc.publicnode.com \
	--network holesky-stage \
	--funding-wallet-private-key $(FUNDING_WALLET_PRIVATE_KEY) \
	--number-wallets $(NUM_WALLETS) \
	--amount-to-deposit $(AMOUNT_TO_DEPOSIT) \
	--amount-to-deposit-to-aligned $(AMOUNT_TO_DEPOSIT_TO_ALIGNED) \
	--private-keys-filepath $(CURDIR)/batcher/aligned-task-sender/wallets/holesky-stage

task_sender_send_infinite_proofs_holesky_stage:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- send-infinite-proofs \
	--burst-size $(BURST_SIZE) --burst-time-secs $(BURST_TIME_SECS) \
	--eth-rpc-url https://ethereum-holesky-rpc.publicnode.com \
	--network holesky-stage \
	--proofs-dirpath $(CURDIR)/scripts/test_files/task_sender/proofs \
	--private-keys-filepath $(CURDIR)/batcher/aligned-task-sender/wallets/holesky-stage

task_sender_test_connections_holesky_stage:
	@cd batcher/aligned-task-sender && \
	cargo run --release -- test-connections \
	--num-senders $(NUM_SENDERS) \
	--network holesky-stage

__UTILS__:
aligned_get_user_balance_devnet:
	@cd batcher/aligned/ && cargo run --release -- get-user-balance \
		--user_addr $(USER_ADDR) \
		--network devnet

aligned_get_user_balance_holesky:
	@cd batcher/aligned/ && cargo run --release -- get-user-balance \
		--rpc_url https://ethereum-holesky-rpc.publicnode.com \
		--network holesky \
		--user_addr $(USER_ADDR)


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
# Prometheus and Grafana
metrics_remove_containers:
	@docker stop prometheus grafana
	@docker rm prometheus grafana
metrics_clean_db: metrics_remove_containers
	@docker volume rm aligned_layer_grafana_data aligned_layer_prometheus_data

run_metrics: ## Run metrics using metrics-docker-compose.yaml
	@echo "Running metrics..."
	@docker compose -f metrics-docker-compose.yaml up

__STORAGE__:
run_storage: ## Run storage using storage-docker-compose.yaml
	@echo "Running storage..."
	@docker compose -f storage-docker-compose.yaml up

__DEPLOYMENT__: ## ____
deploy_aligned_contracts: ## Deploy Aligned Contracts. Parameters: NETWORK=<mainnet|holesky|sepolia>
	@echo "Deploying Aligned Contracts on $(NETWORK) network..."
	@. contracts/scripts/.env.$(NETWORK) && . contracts/scripts/deploy_aligned_contracts.sh

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

# @cp -r scripts/test_files/sp1/fibonacci_proof_generator/script/sp1_fibonacci.elf scripts/test_files/sp1/
generate_sp1_fibonacci_proof:
	@cd scripts/test_files/sp1/fibonacci_proof_generator/script && RUST_LOG=info cargo run --release
	@mv scripts/test_files/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf scripts/test_files/sp1/sp1_fibonacci.elf
	@mv scripts/test_files/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof scripts/test_files/sp1/
	@echo "Fibonacci proof and ELF generated in scripts/test_files/sp1 folder"

generate_risc_zero_empty_journal_proof:
	@cd scripts/test_files/risc_zero/no_public_inputs && RUST_LOG=info cargo run --release
	@echo "Fibonacci proof and ELF with empty journal generated in scripts/test_files/risc_zero/no_public_inputs folder"

build_sp1_macos_old:
	@cd operator/sp1_old/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1_old/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_old_ffi.dylib operator/sp1_old/lib/libsp1_verifier_old_ffi.dylib

build_sp1_linux_old:
	@cd operator/sp1_old/lib && cargo build $(RELEASE_FLAG)
	@cp operator/sp1_old/lib/target/$(TARGET_REL_PATH)/libsp1_verifier_old_ffi.so operator/sp1_old/lib/libsp1_verifier_old_ffi.so

test_sp1_rust_ffi_old:
	@echo "Testing SP1 Rust FFI source code..."
	@cd operator/sp1_old/lib && RUST_MIN_STACK=83886080 cargo t --release

test_sp1_go_bindings_macos_old: build_sp1_macos_old
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1_old/... -v

test_sp1_go_bindings_linux_old: build_sp1_linux_old
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1_old/... -v

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

build_risc_zero_macos_old:
	@cd operator/risc_zero_old/lib && cargo build $(RELEASE_FLAG)
	@cp operator/risc_zero_old/lib/target/$(TARGET_REL_PATH)/librisc_zero_verifier_old_ffi.dylib operator/risc_zero_old/lib/librisc_zero_verifier_old_ffi.dylib

build_risc_zero_linux_old:
	@cd operator/risc_zero_old/lib && cargo build $(RELEASE_FLAG)
	@cp operator/risc_zero_old/lib/target/$(TARGET_REL_PATH)/librisc_zero_verifier_old_ffi.so operator/risc_zero_old/lib/librisc_zero_verifier_old_ffi.so

test_risc_zero_rust_ffi_old:
	@echo "Testing RISC Zero Rust FFI source code..."
	@cd operator/risc_zero_old/lib && cargo test --release

test_risc_zero_go_bindings_macos_old: build_risc_zero_macos_old
	@echo "Testing RISC Zero Go bindings..."
	go test ./operator/risc_zero_old/... -v

test_risc_zero_go_bindings_linux_old: build_risc_zero_linux_old
	@echo "Testing RISC Zero Go bindings..."
	go test ./operator/risc_zero_old/... -v


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
	@$(MAKE) build_sp1_macos_old
	@$(MAKE) build_risc_zero_macos_old
	@$(MAKE) build_merkle_tree_macos
	@echo "All macOS FFIs built successfully."

build_all_ffi_linux: ## Build all FFIs for Linux
	@echo "Building all FFIs for Linux..."
	@$(MAKE) build_sp1_linux
	@$(MAKE) build_risc_zero_linux
	@$(MAKE) build_sp1_linux_old
	@$(MAKE) build_risc_zero_linux_old
	@$(MAKE) build_merkle_tree_linux
	@echo "All Linux FFIs built successfully."

__EXPLORER__:

run_explorer_without_docker_db: explorer_ecto_setup_db
	@cd explorer/ && \
		pnpm install --prefix assets && \
		mix setup && \
		./start.sh

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
              --proof ./scripts/test_files/sp1/sp1_fibonacci.proof \
              --vm_program ./scripts/test_files/sp1/sp1_fibonacci.elf \
              --repetitions $(DOCKER_BURST_SIZE) \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_risc0_burst:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system Risc0 \
              --proof ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
              --vm_program ./scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
              --public_input ./scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
              --repetitions $(DOCKER_BURST_SIZE) \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_plonk_bn254_burst:
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system GnarkPlonkBn254 \
              --proof ./scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
              --public_input ./scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
              --vk ./scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --rpc_url $(DOCKER_RPC_URL) \
              --repetitions $(DOCKER_BURST_SIZE) \
			  --max_fee 0.1ether

docker_batcher_send_plonk_bls12_381_burst:
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
              --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
              --proving_system GnarkPlonkBls12_381 \
              --proof ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
              --public_input ./scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
              --vk ./scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
              --proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
              --repetitions $(DOCKER_BURST_SIZE) \
              --rpc_url $(DOCKER_RPC_URL) \
			  --max_fee 0.1ether

docker_batcher_send_groth16_burst:
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	docker exec $(shell docker ps | grep batcher | awk '{print $$1}') aligned submit \
            --private_key $(DOCKER_PROOFS_PRIVATE_KEY) \
			--proving_system Groth16Bn254 \
			--proof ./scripts/test_files/gnark_groth16_bn254_script/groth16.proof \
			--public_input ./scripts/test_files/gnark_groth16_bn254_script/plonk_pub_input.pub \
			--vk ./scripts/test_files/gnark_groth16_bn254_script/groth16.vk \
			--proof_generator_addr $(PROOF_GENERATOR_ADDRESS) \
			--repetitions $(DOCKER_BURST_SIZE) \
			--rpc_url $(DOCKER_RPC_URL) \
			--max_fee 0.1ether

# Update target as new proofs are supported.
docker_batcher_send_all_proofs_burst:
	@$(MAKE) docker_batcher_send_sp1_burst
	@$(MAKE) docker_batcher_send_risc0_burst
	@$(MAKE) docker_batcher_send_plonk_bn254_burst
	@$(MAKE) docker_batcher_send_plonk_bls12_381_burst
	@$(MAKE) docker_batcher_send_groth16_burst

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
	              --proving_system Groth16Bn254 \
	              --proof scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_groth16.proof \
	              --public_input scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_groth16.pub \
	              --vk scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_$${counter}_groth16.vk \
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
			if [ $$(ls -1 ./aligned_verification_data/*.cbor | wc -l) -ne 5 ]; then \
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

__TELEMETRY__:
# Collector, Jaeger and Elixir API
telemetry_full_start: telemetry_compile_bls_verifier open_telemetry_start telemetry_start

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

telemetry_compile_bls_verifier:
	@cd telemetry_api/priv && \
	go build ../bls_verifier/bls_verify.go

setup_local_aligned_all:
	tmux kill-session -t aligned_layer || true
	tmux new-session -d -s aligned_layer

	tmux new-window -t aligned_layer -n anvil
	tmux send-keys -t aligned_layer 'make anvil_start_with_block_time' C-m

	tmux new-window -t aligned_layer -n aggregator
	tmux send-keys -t aligned_layer:aggregator 'make aggregator_start' C-m

	tmux new-window -t aligned_layer -n operator
	tmux send-keys -t aligned_layer:operator 'sleep 5 && make operator_register_and_start' C-m

	tmux new-window -t aligned_layer -n batcher
	tmux send-keys -t aligned_layer:batcher 'sleep 60 && make batcher_start_local' C-m

	tmux new-window -t aligned_layer -n explorer
	tmux send-keys -t aligned_layer:explorer 'make explorer_create_env && make explorer_build_db && make run_explorer' C-m

	tmux new-window -t aligned_layer -n telemetry
	tmux send-keys -t aligned_layer:telemetry 'docker compose -f telemetry-docker-compose.yaml down && make telemetry_create_env && make telemetry_run_db && make open_telemetry_start && make telemetry_start' C-m

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

__ETHEREUM_PACKAGE__:  ## ____

ethereum_package_start: ## Starts the ethereum_package environment
	kurtosis run --enclave aligned github.com/ethpandaops/ethereum-package --args-file network_params.yaml

ethereum_package_inspect: ## Prints detailed information about the net
	kurtosis enclave inspect aligned

ethereum_package_rm: ## Stops and removes the ethereum_package environment and used resources
	kurtosis enclave rm aligned -f

batcher_start_ethereum_package: user_fund_payment_service
	@echo "Starting Batcher..."
	@$(MAKE) run_storage &
	@cargo run --manifest-path ./batcher/aligned-batcher/Cargo.toml --release -- --config ./config-files/config-batcher-ethereum-package.yaml --env-file ./batcher/aligned-batcher/.env.dev

aggregator_start_ethereum_package:
	$(MAKE) aggregator_start AGG_CONFIG_FILE=config-files/config-aggregator-ethereum-package.yaml

operator_start_ethereum_package:
	$(MAKE) operator_start OPERATOR_ADDRESS=0x70997970C51812dc3A010C7d01b50e0d17dc79C8 CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml

operator_register_start_ethereum_package:
	$(MAKE) operator_full_registration OPERATOR_ADDRESS=0x70997970C51812dc3A010C7d01b50e0d17dc79C8 CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml \
	$(MAKE) operator_start OPERATOR_ADDRESS=0x70997970C51812dc3A010C7d01b50e0d17dc79C8 CONFIG_FILE=config-files/config-operator-1-ethereum-package.yaml


install_spamoor: ## Instal spamoor to spam transactions
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

__NODE_EXPORTER_: ##__

install_node_exporter:
	@./scripts/install_node_exporter.sh
