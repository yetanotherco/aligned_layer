.PHONY: help tests

CONFIG_FILE?=config-files/config.yaml

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

deps: ## Install deps
	git submodule update --init --recursive
	go install github.com/maoueh/zap-pretty@latest
	go install github.com/ethereum/go-ethereum/cmd/abigen@latest

install_foundry:
	curl -L https://foundry.paradigm.xyz | bash

install_eigenlayer_cli:
	@go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

anvil_deploy_eigen_contracts:
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil_deploy_mock_strategy:
	@echo "Deploying Mock Strategy..."
	. contracts/scripts/anvil/deploy_mock_strategy.sh

anvil_deploy_aligned_contracts:
	@echo "Deploying Aligned Contracts..."
	. contracts/scripts/anvil/deploy_aligned_contracts.sh

anvil_start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json

anvil_start_with_block_time:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json --block-time 2

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

operator_deposit_into_mock_strategy:
	@echo "Depositing into strategy"
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.erc20MockStrategy' contracts/script/output/devnet/strategy_deployment_output.json))

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

operator_full_registration: operator_get_eth operator_register_with_eigen_layer operator_mint_mock_tokens operator_deposit_into_mock_strategy operator_register_with_aligned_layer

__BATCHER__:

BURST_SIZE=5

./batcher/.env:
	@echo "To start the Batcher ./batcher/.env needs to be manually"; false;

batcher_start: ./batcher/.env
	@echo "Starting Batcher..."
	@cargo +nightly-2024-04-17 run --manifest-path ./batcher/Cargo.toml --release -- --config ./config-files/config.yaml --env-file ./batcher/.env


build_batcher_client:
	@cd batcher/client && cargo b --release

batcher/client/target/release/batcher-client:
	@cd batcher/client && cargo b --release

batcher_send_sp1_task:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/client/ && cargo run --release -- \
	@cd batcher/client/ && cargo run --release -- \
		--proving_system SP1 \
		--proof test_files/sp1/sp1_fibonacci.proof \
		--vm_program test_files/sp1/sp1_fibonacci-elf

batcher_send_sp1_burst_5:
	@echo "Sending SP1 fibonacci task to Batcher..."
	@cd batcher/client/ && cargo run --release -- \
		--proving_system SP1 \
		--proof test_files/sp1/sp1_fibonacci.proof \
		--vm_program test_files/sp1/sp1_fibonacci-elf \
		--repetitions 5

batcher_send_plonk_bls12_381_task: batcher/client/target/release/batcher-client
	@echo "Sending Groth16 BLS12-381 1!=0 task to Batcher..."
	@cd batcher/client/ && cargo run --release -- \
		--proving_system GnarkPlonkBls12_381 \
		--proof test_files/plonk_bls12_381/plonk.proof \
		--public_input test_files/plonk_bls12_381/plonk_pub_input.pub \
		--vk test_files/plonk_bls12_381/plonk.vk \


batcher_send_groth16_bn254_task: batcher/client/target/release/batcher-client
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/client/ && cargo run --release --  \
		--proving_system Groth16Bn254 \
		--proof test_files/groth16_bn254/ineq_1_groth16.proof \
		--public_input test_files/groth16_bn254/ineq_1_groth16.pub \
		--vk test_files/groth16_bn254/ineq_1_groth16.vk

batcher_send_plonk_bn254_task: batcher/client/target/release/batcher-client
	@echo "Sending PlonkBn254 1!=0 task to Batcher..."
	@cd batcher/client/ && cargo run --release --  \
		--proving_system GnarkPlonkBn254 \
		--proof test_files/plonk_bn254/ineq_1_plonk.proof \
		--public_input test_files/plonk_bn254/ineq_1_plonk.pub \
		--vk test_files/plonk_bn254/ineq_1_plonk.vk \

batcher_send_groth16_burst_5: batcher/client/target/release/batcher-client
	@echo "Sending Groth16Bn254 1!=0 task to Batcher..."
	@cd batcher/client/ && cargo run --release -- \
		--proving_system Groth16Bn254 \
		--proof test_files/groth16/ineq_1_groth16.proof \
		--public_input test_files/groth16/ineq_1_groth16.pub \
		--vk test_files/groth16/ineq_1_groth16.vk \
		--repetitions 5

batcher_send_infinite_groth16_task: ./batcher/client/target/release/batcher-client ## Send a different Groth16 BN254 proof using the task sender every 3 seconds
	@mkdir -p task_sender/test_examples/gnark_groth16_bn254_infinite_script/infinite_proofs
	@echo "Sending a different GROTH16 BN254 proof in a loop every n seconds..."
	@./batcher/client/send_infinite_tasks.sh 4 Groth16


batcher_send_infinite_plonk_bn254_tasks: ./batcher/client/target/release/batcher-client ## Send a different PLONK BN254 proof using the task sender every 3 seconds
	@mkdir -p task_sender/test_examples/gnark_plonk_bn254_infinite_script/infinite_proofs	
	@echo "Sending a different PLONK BN254 proof in a loop every n seconds..."
	@./batcher/client/send_infinite_tasks.sh 4 PlonkBn254

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

generate_plonk_bn254_ineq_proof: ## Run the gnark_plonk_bn254_script
	@echo "Running gnark_plonk_bn254_ineq script..."
	@go run task_sender/test_examples/gnark_plonk_bn254_infinite_script/cmd/main.go 1

send_infinite_plonk_bn254_proof: ## Send a different Groth16 BN254 proof using the task sender every 3 seconds
	@echo "Sending a different PLONK BN254 proof in a loop every 3 seconds..."
	@go run task_sender/cmd/main.go infinite-tasks \
		--proving-system plonk_bn254 \
		--config config-files/config.yaml \
		--interval 3 \
		2>&1 | zap-pretty

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
	@go run task_sender/test_examples/gnark_groth16_bn254_infinite_script/cmd/main.go 1

send_sp1_proof:
	@go run task_sender/cmd/main.go send-task \
    		--proving-system sp1 \
    		--proof task_sender/test_examples/sp1/sp1_fibonacci.proof \
    		--public-input task_sender/test_examples/sp1/elf/riscv32im-succinct-zkvm-elf \
    		--config config-files/config.yaml \
    		2>&1 | zap-pretty

__METRICS__:
run_metrics: ## Run metrics using metrics-docker-compose.yaml
	@echo "Running metrics..."
	@docker-compose -f metrics-docker-compose.yaml up

__DEPLOYMENT__:
deploy_aligned_contracts: ## Deploy Aligned Contracts
	@echo "Deploying Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_aligned_contracts.sh

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

run_local:
	./scripts/run_local.sh

__SP1_FFI__: ##
build_sp1_macos:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.dylib operator/sp1/lib/libsp1_verifier.dylib
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.a operator/sp1/lib/libsp1_verifier.a

build_sp1_linux:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.so operator/sp1/lib/libsp1_verifier.so
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.a operator/sp1/lib/libsp1_verifier.a

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
