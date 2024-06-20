.PHONY: help tests

OS := $(shell uname -s)

CONFIG_FILE?=config-files/config.yaml

ifeq ($(OS),Linux)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_linux
endif

ifeq ($(OS),Darwin)
	BUILD_ALL_FFI = $(MAKE) build_all_ffi_macos
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
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 3

# TODO: Allow enviroment variables / different configuration files
aggregator_start:
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(CONFIG_FILE) \
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
	@go build -o ./operator/build/aligned-operator ./operator/cmd/main.go
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

./batcher/aligned-batcher/.env:
	@echo "To start the Batcher ./batcher/aligned-batcher/.env needs to be manually set"; false;

batcher_start: ./batcher/aligned-batcher/.env
	@echo "Starting Batcher..."
	@cargo +nightly-2024-04-17 run --manifest-path ./batcher/aligned-batcher/Cargo.toml --release -- --config ./config-files/config.yaml --env-file ./batcher/aligned-batcher/.env

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
		--proof test_files/sp1/sp1_fibonacci.proof \
		--vm_program test_files/sp1/sp1_fibonacci-elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_sp1_burst:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system SP1 \
		--proof test_files/sp1/sp1_fibonacci.proof \
		--vm_program test_files/sp1/sp1_fibonacci-elf \
		--repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_jolt_task:
	@echo "Sending Jolt fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- \
		--proving_system Jolt \
		--proof test_files/jolt/fibonacci-guest.proof \
		--vm_program test_files/jolt/fibonacci-guest.elf \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_jolt_burst:
	@echo "Sending Jolt fibonacci task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- \
		--proving_system Jolt \
		--proof test_files/jolt/fibonacci-guest.proof \
		--vm_program test_files/jolt/fibonacci-guest.elf \
		--repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657


batcher_send_infinite_sp1:
	@echo "Sending infinite SP1 fibonacci task to Batcher..."
	@./batcher/aligned/send_infinite_sp1_tasks/send_infinite_sp1_tasks.sh

batcher_send_plonk_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof test_files/plonk_bn254/plonk.proof \
		--public_input test_files/plonk_bn254/plonk_pub_input.pub \
		--vk test_files/plonk_bn254/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_plonk_bn254_burst: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBn254 \
		--proof test_files/plonk_bn254/plonk.proof \
		--public_input test_files/plonk_bn254/plonk_pub_input.pub \
		--vk test_files/plonk_bn254/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions 15

batcher_send_plonk_bls12_381_task: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof test_files/plonk_bls12_381/plonk.proof \
		--public_input test_files/plonk_bls12_381/plonk_pub_input.pub \
		--vk test_files/plonk_bls12_381/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_plonk_bls12_381_burst: batcher/target/release/aligned
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system GnarkPlonkBls12_381 \
		--proof test_files/plonk_bls12_381/plonk.proof \
		--public_input test_files/plonk_bls12_381/plonk_pub_input.pub \
		--vk test_files/plonk_bls12_381/plonk.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657 \
		--repetitions 15


batcher_send_groth16_bn254_task: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof test_files/groth16/ineq_1_groth16.proof \
		--public_input test_files/groth16/ineq_1_groth16.pub \
		--vk test_files/groth16/ineq_1_groth16.vk \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_groth16_burst: batcher/target/release/aligned
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Groth16Bn254 \
		--proof test_files/groth16/ineq_1_groth16.proof \
		--public_input test_files/groth16/ineq_1_groth16.pub \
		--vk test_files/groth16/ineq_1_groth16.vk \
		--repetitions 15 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_infinite_groth16: batcher/target/release/aligned ## Send a different Groth16 BN254 proof using the task sender every 3 seconds
	@mkdir -p task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs
	@echo "Sending a different GROTH16 BN254 proof in a loop every n seconds..."
	@./batcher/aligned/send_infinite_tasks.sh 4

batcher_send_burst_groth16: batcher/target/release/aligned
	@echo "Sending a burst of tasks to Batcher..."
	@mkdir -p task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs
	@./batcher/aligned/send_burst_tasks.sh $(BURST_SIZE) $(START_COUNTER)

batcher_send_halo2_ipa_task: batcher/target/release/aligned
	@echo "Sending Halo2 IPA 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2IPA \
		--proof test_files/halo2_ipa/proof.bin \
		--public_input test_files/halo2_ipa/pub_input.bin \
		--vk test_files/halo2_ipa/params.bin \

batcher_send_halo2_ipa_task_burst_5: batcher/target/release/aligned
	@echo "Sending Halo2 IPA 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2IPA \
		--proof test_files/halo2_ipa/proof.bin \
		--public_input test_files/halo2_ipa/pub_input.bin \
		--vk test_files/halo2_ipa/params.bin \
		--repetitions 5

batcher_send_halo2_kzg_task: batcher/target/release/aligned
	@echo "Sending Halo2 KZG 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2KZG \
		--proof test_files/halo2_kzg/proof.bin \
		--public_input test_files/halo2_kzg/pub_input.bin \
		--vk test_files/halo2_kzg/params.bin \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

batcher_send_halo2_kzg_task_burst_5: batcher/target/release/aligned
	@echo "Sending Halo2 KZG 1!=0 task to Batcher..."
	@cd batcher/aligned/ && cargo run --release -- submit \
		--proving_system Halo2KZG \
		--proof test_files/halo2_kzg/proof.bin \
		--public_input test_files/halo2_kzg/pub_input.bin \
		--vk test_files/halo2_kzg/params.bin \
		--repetitions 5 \
		--proof_generator_addr 0x66f9664f97F2b50F62D13eA064982f936dE76657

__TASK_SENDERS__:
 # TODO add a default proving system

send_plonk_bls12_381_proof: ## Send a PLONK BLS12_381 proof using the task sender
	@echo "Sending PLONK BLS12_381 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system plonk_bls12_381 \
		--proof task_sender/test_examples/gnark_plonk_bls12_381_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_plonk_bls12_381_script/plonk.vk \
		--config config-files/config.yaml \
		--quorum-threshold 98 \
		2>&1 | zap-pretty

send_plonk_bls12_381_proof_loop: ## Send a PLONK BLS12_381 proof using the task sender every 10 seconds
	@echo "Sending PLONK BLS12_381 proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system plonk_bls12_381 \
		--proof task_sender/test_examples/gnark_plonk_bls12_381_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_plonk_bls12_381_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_plonk_bls12_381_script/plonk.vk \
		--config config-files/config.yaml \
		--interval 10 \
		2>&1 | zap-pretty

generate_plonk_bls12_381_proof: ## Run the gnark_plonk_bls12_381_script
	@echo "Running gnark_plonk_bls12_381 script..."
	@go run task_sender/test_examples/gnark_plonk_bls12_381_script/main.go


send_plonk_bn254_proof: ## Send a PLONK BN254 proof using the task sender
	@echo "Sending PLONK BN254 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system plonk_bn254 \
		--proof task_sender/test_examples/gnark_plonk_bn254_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_plonk_bn254_script/plonk.vk \
		--config config-files/config.yaml \
		2>&1 | zap-pretty

send_plonk_bn254_proof_loop: ## Send a PLONK BN254 proof using the task sender every 10 seconds
	@echo "Sending PLONK BN254 proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system plonk_bn254 \
		--proof task_sender/test_examples/gnark_plonk_bn254_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_plonk_bn254_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_plonk_bn254_script/plonk.vk \
		--config config-files/config.yaml \
		--interval 10 \
		2>&1 | zap-pretty

generate_plonk_bn254_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_plonk_bn254 script..."
	@go run task_sender/test_examples/gnark_plonk_bn254_script/main.go

send_groth16_bn254_proof: ## Send a Groth16 BN254 proof using the task sender
	@echo "Sending GROTH16 BN254 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system groth16_bn254 \
		--proof task_sender/test_examples/gnark_groth16_bn254_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_groth16_bn254_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_groth16_bn254_script/plonk.vk \
		--config config-files/config.yaml \
		--quorum-threshold 98 \
		2>&1 | zap-pretty

send_groth16_bn254_proof_loop: ## Send a Groth16 BN254 proof using the task sender every 10 seconds
	@echo "Sending GROTH16 BN254 proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system groth16_bn254 \
		--proof task_sender/test_examples/gnark_groth16_bn254_script/plonk.proof \
		--public-input task_sender/test_examples/gnark_groth16_bn254_script/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/gnark_groth16_bn254_script/plonk.vk \
		--config config-files/config.yaml \
		--interval 10 \
		2>&1 | zap-pretty

send_infinite_groth16_bn254_proof: ## Send a different Groth16 BN254 proof using the task sender every 3 seconds
	@echo "Sending a different GROTH16 BN254 proof in a loop every 3 seconds..."
	@go run task_sender/cmd/main.go infinite-tasks \
		--proving-system groth16_bn254 \
		--config config-files/config.yaml \
		--interval 3 \
		2>&1 | zap-pretty


generate_groth16_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_groth_bn254 script..."
	@go run task_sender/test_examples/gnark_groth16_bn254_script/main.go

generate_groth16_ineq_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_groth_bn254_ineq script..."
	@go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/main.go 1

send_sp1_proof:
	@go run task_sender/cmd/main.go send-task \
    		--proving-system sp1 \
    		--proof task_sender/test_examples/sp1/sp1_fibonacci.proof \
    		--public-input task_sender/test_examples/sp1/elf/riscv32im-succinct-zkvm-elf \
    		--config config-files/config.yaml \
    		2>&1 | zap-pretty

send_jolt_proof:
	@go run task_sender/cmd/main.go send-task \
    		--proving-system jolt \
    		--proof task_sender/test_examples/jolt/fibonacci/fibonacci-guest.proof \
    		--public-input task_sender/test_examples/jolt/fibonacci/elf/fibonacci-guest.elf \
    		--config config-files/config.yaml \
    		2>&1 | zap-pretty

send_halo2_ipa_proof: ## Send a Halo2 IPA proof using the task sender
	@echo "Sending Halo2 IPA proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system halo2_ipa \
		--proof task_sender/test_examples/halo2_ipa/proof.bin \
		--public-input task_sender/test_examples/halo2_ipa/pub_input.bin \
		--verification-key task_sender/test_examples/halo2_ipa/params.bin \
		--config config-files/config.yaml \
		2>&1 | zap-pretty

send_halo2_ipa_proof_loop: ## Send a Halo2 IPA proof using the task sender every 10 seconds
	@echo "Sending Halo2 IPA proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system halo2_ipa \
		--proof task_sender/test_examples/halo2_ipa/proof.bin \
		--public-input task_sender/test_examples/halo2_ipa/pub_input.bin \
		--verification-key task_sender/test_examples/halo2_ipa/params.bin \
		--config config-files/config.yaml \
		--interval 10 \
		2>&1 | zap-pretty

send_halo2_kzg_proof: ## Send a Halo2 KZG proof using the task sender
	@echo "Sending Halo2 KZG proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system halo2_kzg \
		--proof task_sender/test_examples/halo2_kzg/proof.bin \
		--public-input task_sender/test_examples/halo2_kzg/pub_input.bin \
		--verification-key task_sender/test_examples/halo2_kzg/params.bin \
		--config config-files/config.yaml \
		2>&1 | zap-pretty

send_halo2_kzg_proof_loop: ## Send a Halo2 KZG proof using the task sender every 10 seconds
	@echo "Sending Halo2 KZG proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system halo2_kzg \
		--proof task_sender/test_examples/halo2_kzg/proof.bin \
		--public-input task_sender/test_examples/halo2_kzg/pub_input.bin \
		--verification-key task_sender/test_examples/halo2_kzg/params.bin \
		--config config-files/config.yaml \
		--interval 10 \
		2>&1 | zap-pretty

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
	@. contracts/scripts/.env && . ./contracts/scripts/deploy_verify_batch_inclusion_caller.sh
	
build_aligned_contracts:
	@cd contracts/src/core && forge build

__BUILD__:
build_binaries:
	@echo "Building aggregator..."
	@go build -o ./aggregator/build/aligned-aggregator ./aggregator/cmd/main.go
	@echo "Aggregator built into /aggregator/build/aligned-aggregator"
	@echo "Building aligned layer operator..."
	@go build -o ./operator/build/aligned-operator ./operator/cmd/main.go
	@echo "Aligned layer operator built into /operator/build/aligned-operator"
	@echo "Building task sender.."
	@go build -o ./task_sender/build/aligned-task-sender ./task_sender/cmd/main.go
	@echo "Task sender built into /task_sender/build/aligned-task-sender"

__SP1_FFI__: ##
build_sp1_macos:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.dylib operator/sp1/lib/libsp1_verifier.dylib

build_sp1_linux:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.so operator/sp1/lib/libsp1_verifier.so

test_sp1_rust_ffi:
	@echo "Testing SP1 Rust FFI source code..."
	@cd operator/sp1/lib && RUST_MIN_STACK=83886080 cargo t --release

test_sp1_go_bindings_macos: build_sp1_macos
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

test_sp1_go_bindings_linux: build_sp1_linux
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

# @cp -r task_sender/test_examples/sp1/fibonacci_proof_generator/script/elf task_sender/test_examples/sp1/
generate_sp1_fibonacci_proof:
	@cd task_sender/test_examples/sp1/fibonacci_proof_generator/script && RUST_LOG=info cargo run --release
	@mv task_sender/test_examples/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf task_sender/test_examples/sp1/elf
	@mv task_sender/test_examples/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof task_sender/test_examples/sp1/
	@echo "Fibonacci proof and ELF generated in task_sender/test_examples/sp1 folder"

__JOLT_FFI__: ##
build_jolt_macos:
	@cd operator/jolt/lib && cargo build --release
	@cp operator/jolt/lib/target/release/libjolt_verifier_ffi.dylib operator/jolt/lib/libjolt_verifier.dylib
	@cp operator/jolt/lib/target/release/libjolt_verifier_ffi.a operator/jolt/lib/libjolt_verifier.a

build_jolt_linux:
	@cd operator/jolt/lib && cargo build --release
	@cp operator/jolt/lib/target/release/libjolt_verifier_ffi.so operator/jolt/lib/libjolt_verifier.so
	@cp operator/jolt/lib/target/release/libjolt_verifier_ffi.a operator/jolt/lib/libjolt_verifier.a

test_jolt_rust_ffi:
	@echo "Testing Jolt Rust FFI source code..."
	@cd operator/jolt/lib && RUST_MIN_STACK=93886080 cargo test --release

test_jolt_go_bindings_macos: build_jolt_macos
	@echo "Testing JOLT Go bindings..."
	go test ./operator/jolt/... -v

test_jolt_go_bindings_linux: build_jolt_linux
	@echo "Testing Jolt Go bindings..."
	go test ./operator/jolt/... -v

generate_jolt_fibonacci_proof:
	@cd task_sender/test_examples/jolt/fibonacci && JOLT_SAVE=true cargo run --release

generate_jolt_sha3_proof:
	@cd task_sender/test_examples/jolt/sha3-ex && JOLT_SAVE=true cargo run --release

__RISC_ZERO_FFI__: ##
build_risc_zero_macos:
	@cd operator/risc_zero/lib && cargo build --release
	@cp operator/risc_zero/lib/target/release/librisc_zero_verifier_ffi.dylib operator/risc_zero/lib/librisc_zero_verifier_ffi.dylib
	@cp operator/risc_zero/lib/target/release/librisc_zero_verifier_ffi.a operator/risc_zero/lib/librisc_zero_verifier_ffi.a

build_risc_zero_linux:
	@cd operator/risc_zero/lib && cargo build --release
	@cp operator/risc_zero/lib/target/release/librisc_zero_verifier_ffi.so operator/risc_zero/lib/librisc_zero_verifier_ffi.so
	@cp operator/risc_zero/lib/target/release/librisc_zero_verifier_ffi.a operator/risc_zero/lib/librisc_zero_verifier_ffi.a

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
	@cd task_sender/test_examples/risc_zero/fibonacci_proof_generator && \
		cargo clean && \
		rm -f risc_zero_fibonacci.proof && \
		RUST_LOG=info cargo run --release && \
		echo "Fibonacci proof generated in task_sender/test_examples/risc_zero folder" && \
		echo "Fibonacci proof image ID generated in task_sender/test_examples/risc_zero folder"

__MERKLE_TREE_FFI__: ##
build_merkle_tree_macos:
	@cd operator/merkle_tree/lib && cargo build --release
	@cp operator/merkle_tree/lib/target/release/libmerkle_tree.dylib operator/merkle_tree/lib/libmerkle_tree.dylib
	@cp operator/merkle_tree/lib/target/release/libmerkle_tree.a operator/merkle_tree/lib/libmerkle_tree.a

build_merkle_tree_linux:
	@cd operator/merkle_tree/lib && cargo build --release
	@cp operator/merkle_tree/lib/target/release/libmerkle_tree.so operator/merkle_tree/lib/libmerkle_tree.so
	@cp operator/merkle_tree/lib/target/release/libmerkle_tree.a operator/merkle_tree/lib/libmerkle_tree.a

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
	@cd operator/halo2kzg/lib && cargo build --release
	@cp operator/halo2kzg/lib/target/release/libhalo2kzg_verifier_ffi.dylib operator/halo2kzg/lib/libhalo2kzg_verifier.dylib
	@cp operator/halo2kzg/lib/target/release/libhalo2kzg_verifier_ffi.a operator/halo2kzg/lib/libhalo2kzg_verifier.a

build_halo2_kzg_linux:
	@cd operator/halo2kzg/lib && cargo build --release
	@cp operator/halo2kzg/lib/target/release/libhalo2kzg_verifier_ffi.so operator/halo2kzg/lib/libhalo2kzg_verifier.so
	@cp operator/halo2kzg/lib/target/release/libhalo2kzg_verifier_ffi.a operator/halo2kzg/lib/libhalo2kzg_verifier.a

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
	@cd task_sender/test_examples/halo2_kzg && \
	cargo clean && \
	rm params.bin proof.bin pub_input.bin && \
	RUST_LOG=info cargo run --release && \
	echo "Generating halo2 plonk proof..." && \
	echo "Generated halo2 plonk proof!"

__HALO2_IPA_FFI__: ##
build_halo2_ipa_macos:
	@cd operator/halo2ipa/lib && cargo build --release
	@cp operator/halo2ipa/lib/target/release/libhalo2ipa_verifier_ffi.dylib operator/halo2ipa/lib/libhalo2ipa_verifier.dylib
	@cp operator/halo2ipa/lib/target/release/libhalo2ipa_verifier_ffi.a operator/halo2ipa/lib/libhalo2ipa_verifier.a

build_halo2_ipa_linux:
	@cd operator/halo2ipa/lib && cargo build --release
	@cp operator/halo2ipa/lib/target/release/libhalo2ipa_verifier_ffi.so operator/halo2ipa/lib/libhalo2ipa_verifier.so
	@cp operator/halo2ipa/lib/target/release/libhalo2ipa_verifier_ffi.a operator/halo2ipa/lib/libhalo2ipa_verifier.a

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
	@cd task_sender/test_examples/halo2_ipa && \
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
	@$(MAKE) build_jolt_macos
	@$(MAKE) build_risc_zero_macos
#	@$(MAKE) build_merkle_tree_macos
	@$(MAKE) build_halo2_ipa_macos
	@$(MAKE) build_halo2_kzg_macos
	@echo "All macOS FFIs built successfully."

build_all_ffi_linux: ## Build all FFIs for Linux
	@echo "Building all FFIs for Linux..."
	@$(MAKE) build_sp1_linux
	@$(MAKE) build_jolt_linux
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
		./scripts/fetch_old_batches.sh 1600000 1716277 
