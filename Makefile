.PHONY: help tests

OS := $(shell uname -s)

CONFIG_FILE?=config-files/config.yaml
AGG_CONFIG_FILE?=config-files/config-aggregator.yaml

OPERATOR_VERSION=v0.2.1

ifeq ($(OS),Linux)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_linux
endif

ifeq ($(OS),Darwin)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_macos
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

deps: submodules build_all_ffi ## Install deps

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

anvil_start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json

anvil_start_with_block_time:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 7

aggregator_start:
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(AGG_CONFIG_FILE) \
	2>&1 | zap-pretty

aggregator_send_dummy_responses:
	@echo "Sending dummy responses to Aggregator..."
	@cd aggregator && go run dummy/submit_task_responses.go

operator_start:
	@echo "Starting Operator..."
	go run operator/cmd/main.go start --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

operator_register_and_start: operator_full_registration operator_start

build_operator: deps
	@echo "Building Operator..."
	@go build -ldflags "-X main.Version=$(OPERATOR_VERSION)" -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Operator built into /operator/build/aligned-operator"

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
	. ./scripts/mint_mock_token.sh $(CONFIG_FILE) 1000

operator_whitelist_devnet:
	@echo "Whitelisting operator"
	$(eval OPERATOR_ADDRESS = $(shell yq -r '.operator.address' $(CONFIG_FILE)))
	@echo "Operator address: $(OPERATOR_ADDRESS)"
	RPC_URL="http://localhost:8545" PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" OUTPUT_PATH=./script/output/devnet/alignedlayer_deployment_output.json ./contracts/scripts/whitelist_operator.sh $(OPERATOR_ADDRESS)

operator_whitelist:
	@echo "Whitelisting operator $(OPERATOR_ADDRESS)"
	@. contracts/scripts/.env && . contracts/scripts/whitelist_operator.sh $(OPERATOR_ADDRESS)

operator_deposit_into_mock_strategy:
	@echo "Depositing into strategy"
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.addresses.strategies.MOCK' contracts/script/output/devnet/eigenlayer_deployment_output.json))

	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount 1000

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

operator_full_registration: operator_get_eth operator_register_with_eigen_layer operator_mint_mock_tokens operator_deposit_into_mock_strategy operator_whitelist_devnet operator_register_with_aligned_layer

operator_start_docker:
	@echo "Starting Operator..."
	@docker-compose -f operator/docker/compose.yaml up

__BATCHER__:

BURST_SIZE=5

user_fund_payment_service:
	@. ./scripts/user_fund_payment_service_devnet.sh

./batcher/aligned-batcher/.env:
	@echo "To start the Batcher ./batcher/aligned-batcher/.env needs to be manually set"; false;

batcher_start: ./batcher/aligned-batcher/.env user_fund_payment_service 
	@echo "Starting Batcher..."
	@cargo +nightly-2024-04-17 run --manifest-path ./batcher/aligned-batcher/Cargo.toml --release -- --config ./config-files/config-batcher.yaml --env-file ./batcher/aligned-batcher/.env

install_batcher:
	@cargo +nightly-2024-04-17 install --path batcher/aligned-batcher

install_aligned:
	@./batcher/aligned/install_aligned.sh

uninstall_aligned:
	@rm -rf ~/.aligned && echo "Aligned uninstalled"

install_aligned_compiling:
	@cargo +nightly-2024-04-17 install --path batcher/aligned

build_batcher_client:
	@cd batcher/aligned && cargo b --release

batcher/target/release/aligned:
	@cd batcher/aligned && cargo b --release

batcher_send_sp1_task:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci.elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_sp1_burst:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof ../../scripts/test_files/sp1/sp1_fibonacci.proof \
		--vm_program ../../scripts/test_files/sp1/sp1_fibonacci.elf \
		--repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

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
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_risc0_burst:
	@echo "Sending Risc0 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Risc0 \
		--proof ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof \
        --vm_program ../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin \
        --public_input ../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub \
        --repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_plonk_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_plonk_bn254_burst: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof ../../scripts/test_files/gnark_plonk_bn254_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bn254_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions 15

batcher_send_plonk_bls12_381_task: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_plonk_bls12_381_burst: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.proof \
		--public_input ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--vk ../../scripts/test_files/gnark_plonk_bls12_381_script/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions 15


batcher_send_groth16_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_groth16_burst: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.proof \
		--public_input ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.pub \
		--vk ../../scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_1_groth16.vk \
		--repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_infinite_groth16: batcher/target/release/aligned ## Send a different Groth16 BN254 proof using the client every 3 seconds
	@mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs
	@echo "Sending a different GROTH16 BN254 proof in a loop every n seconds..."
	@./batcher/aligned/send_infinite_tasks.sh 4

batcher_send_burst_groth16: batcher/target/release/aligned
	@echo "Sending a burst of tasks to Batcher..."
	@mkdir -p scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs
	@./batcher/aligned/send_burst_tasks.sh $(BURST_SIZE) $(START_COUNTER)

batcher_send_halo2_ipa_task: batcher/target/release/aligned
	@echo "Sending Halo2 IPA 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2IPA \
		--proof ../../scripts/test_files/halo2_ipa/proof.bin \
		--public_input ../../scripts/test_files/halo2_ipa/pub_input.bin \
		--vk ../../scripts/test_files/halo2_ipa/params.bin \

batcher_send_halo2_ipa_task_burst_5: batcher/target/release/aligned
	@echo "Sending Halo2 IPA 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2IPA \
		--proof ../../scripts/test_files/halo2_ipa/proof.bin \
		--public_input ../../scripts/test_files/halo2_ipa/pub_input.bin \
		--vk ../../scripts/test_files/halo2_ipa/params.bin \
		--repetitions 5

batcher_send_halo2_kzg_task: batcher/target/release/aligned
	@echo "Sending Halo2 KZG 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2KZG \
		--proof ../../scripts/test_files/halo2_kzg/proof.bin \
		--public_input ../../scripts/test_files/halo2_kzg/pub_input.bin \
		--vk ../../scripts/test_files/halo2_kzg/params.bin \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_halo2_kzg_task_burst_5: batcher/target/release/aligned
	@echo "Sending Halo2 KZG 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2KZG \
		--proof ../../scripts/test_files/halo2_kzg/proof.bin \
		--public_input ../../scripts/test_files/halo2_kzg/pub_input.bin \
		--vk ../../scripts/test_files/halo2_kzg/params.bin \
		--repetitions 5 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

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
run_metrics: ## Run metrics using metrics-docker-compose.yaml
	@echo "Running metrics..."
	@docker-compose -f metrics-docker-compose.yaml up

__DEPLOYMENT__:
deploy_aligned_contracts: ## Deploy Aligned Contracts
	@echo "Deploying Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_aligned_contracts.sh

upgrade_aligned_contracts: ## Upgrade Aligned Contracts
	@echo "Upgrading Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/upgrade_aligned_contracts.sh

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

__HALO2_KZG_FFI__: ##
build_halo2_kzg_macos:
	@cd operator/halo2kzg/lib && cargo build $(RELEASE_FLAG)
	@cp operator/halo2kzg/lib/target/$(TARGET_REL_PATH)/libhalo2kzg_verifier_ffi.dylib operator/halo2kzg/lib/libhalo2kzg_verifier.dylib
	@cp operator/halo2kzg/lib/target/$(TARGET_REL_PATH)/libhalo2kzg_verifier_ffi.a operator/halo2kzg/lib/libhalo2kzg_verifier.a

build_halo2_kzg_linux:
	@cd operator/halo2kzg/lib && cargo build $(RELEASE_FLAG)
	@cp operator/halo2kzg/lib/target/$(TARGET_REL_PATH)/libhalo2kzg_verifier_ffi.so operator/halo2kzg/lib/libhalo2kzg_verifier.so
	@cp operator/halo2kzg/lib/target/$(TARGET_REL_PATH)/libhalo2kzg_verifier_ffi.a operator/halo2kzg/lib/libhalo2kzg_verifier.a

test_halo2_kzg_rust_ffi:
	@echo "Testing Halo2-KZG Rust FFI source code..."
	@cd operator/halo2kzg/lib && cargo t --release

test_halo2_kzg_go_bindings_macos: build_halo2_kzg_macos
	@echo "Testing Halo2-KZG Go bindings..."
	go test ./operator/halo2kzg/... -v

test_halo2_kzg_go_bindings_linux: build_halo2_kzg_linux
	@echo "Testing Halo2-KZG Go bindings..."
	go test ./operator/halo2kzg/... -v

generate_halo2_kzg_proof:
	@cd scripts/test_files/halo2_kzg && \
	cargo clean && \
	rm params.bin proof.bin pub_input.bin && \
	RUST_LOG=info cargo run --release && \
	echo "Generating halo2 plonk proof..." && \
	echo "Generated halo2 plonk proof!"

__HALO2_IPA_FFI__: ##
build_halo2_ipa_macos:
	@cd operator/halo2ipa/lib && cargo build $(RELEASE_FLAG)
	@cp operator/halo2ipa/lib/target/$(TARGET_REL_PATH)/libhalo2ipa_verifier_ffi.dylib operator/halo2ipa/lib/libhalo2ipa_verifier.dylib
	@cp operator/halo2ipa/lib/target/$(TARGET_REL_PATH)/libhalo2ipa_verifier_ffi.a operator/halo2ipa/lib/libhalo2ipa_verifier.a

build_halo2_ipa_linux:
	@cd operator/halo2ipa/lib && cargo build $(RELEASE_FLAG)
	@cp operator/halo2ipa/lib/target/$(TARGET_REL_PATH)/libhalo2ipa_verifier_ffi.so operator/halo2ipa/lib/libhalo2ipa_verifier.so
	@cp operator/halo2ipa/lib/target/$(TARGET_REL_PATH)/libhalo2ipa_verifier_ffi.a operator/halo2ipa/lib/libhalo2ipa_verifier.a

test_halo2_ipa_rust_ffi:
	@echo "Testing Halo2-KZG Rust FFI source code..."
	@cd operator/halo2ipa/lib && cargo t --release

test_halo2_ipa_go_bindings_macos: build_halo2_ipa_macos
	@echo "Testing Halo2-KZG Go bindings..."
	go test ./operator/halo2ipa/... -v

test_halo2_ipa_go_bindings_linux: build_halo2_ipa_linux
	@echo "Testing Halo2-KZG Go bindings..."
	go test ./operator/halo2ipa/... -v

generate_halo2_ipa_proof:
	@cd scripts/test_files/halo2_ipa && \
	cargo clean && \
	rm params.bin proof.bin pub_input.bin && \
	RUST_LOG=info cargo run --release && \
	echo "Generating halo2 plonk proof..." && \
	echo "Generated halo2 plonk proof!"


__BUILD_ALL_FFI__:

build_all_ffi: ## Build all FFIs
	$(BUILD_ALL_FFI)
	@echo "Created FFIs"

build_all_ffi_macos: ## Build all FFIs for macOS
	@echo "Building all FFIs for macOS..."
	@$(MAKE) build_sp1_macos
	@$(MAKE) build_risc_zero_macos
#	@$(MAKE) build_merkle_tree_macos
	@$(MAKE) build_halo2_ipa_macos
	@$(MAKE) build_halo2_kzg_macos
	@echo "All macOS FFIs built successfully."

build_all_ffi_linux: ## Build all FFIs for Linux
	@echo "Building all FFIs for Linux..."
	@$(MAKE) build_sp1_linux
	@$(MAKE) build_risc_zero_linux
#	@$(MAKE) build_merkle_tree_linux
	@$(MAKE) build_halo2_ipa_linux
	@$(MAKE) build_halo2_kzg_linux
	@echo "All Linux FFIs built successfully."


__EXPLORER__:
run_devnet_explorer: run_db ecto_setup_db
	@cd explorer/ && \
		mix setup && \
		cp .env.dev .env && \
		./start.sh

run_explorer: run_db ecto_setup_db
	@cd explorer/ && \
		mix setup && \
		./start.sh

build_db:
	@cd explorer && \
		docker build -t explorer-postgres-image .

run_db: remove_db_container
	@cd explorer && \
		docker run -d --name explorer-postgres-container -p 5432:5432 -v explorer-postgres-data:/var/lib/postgresql/data explorer-postgres-image

ecto_setup_db:
		@cd explorer/ && \
		./ecto_setup_db.sh

remove_db_container:
	@cd explorer && \
		docker stop explorer-postgres-container || true  && \
		docker rm explorer-postgres-container || true

clean_db: remove_db_container
	@cd explorer && \
		docker volume rm explorer-postgres-data || true

dump_db:
	@cd explorer && \
		docker exec -t explorer-postgres-container pg_dumpall -c -U explorer_user > dump.$$(date +\%Y\%m\%d_\%H\%M\%S).sql
	@echo "Dumped database successfully to /explorer"

recover_db: run_db
	@read -p $$'\e[32mEnter the dump file to recover (e.g., dump.20230607_123456.sql): \e[0m' DUMP_FILE && \
	cd explorer && \
	docker cp $$DUMP_FILE explorer-postgres-container:/dump.sql && \
	docker exec -t explorer-postgres-container psql -U explorer_user -d explorer_db -f /dump.sql && \
	echo "Recovered database successfully from $$DUMP_FILE"

explorer_fetch_old_batches:
	@cd explorer && \
	./scripts/fetch_old_batches.sh 1728056 1729806
