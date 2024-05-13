.PHONY: help tests

CONFIG_FILE?=config-files/config.yaml
DA_SOLUTION=calldata

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

deps: ## Install deps
	git submodule update --init --recursive
	go install github.com/maoueh/zap-pretty@latest
	go install github.com/ethereum/go-ethereum/cmd/abigen@latest

install-foundry:
	curl -L https://foundry.paradigm.xyz | bash

install-eigenlayer-cli:
	@go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

anvil-deploy-eigen-contracts:
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil-deploy-mock-strategy:
	@echo "Deploying Mock Strategy..."
	. contracts/scripts/anvil/deploy_mock_strategy.sh

anvil-deploy-aligned-contracts:
	@echo "Deploying Aligned Contracts..."
	. contracts/scripts/anvil/deploy_aligned_contracts.sh

anvil-start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json 

# TODO: Allow enviroment variables / different configuration files
aggregator-start:
	@echo "Starting Aggregator..."
	@go run aggregator/cmd/main.go --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

aggregator-send-dummy-responses:
	@echo "Sending dummy responses to Aggregator..."
	@cd aggregator && go run dummy/submit_task_responses.go

operator-start:
	@echo "Starting Operator..."
	go run operator/cmd/main.go start --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

bindings:
	cd contracts && ./generate-go-bindings.sh

test:
	go test ./...


get-delegation-manager-address:
	@sed -n 's/.*"delegationManager": "\([^"]*\)".*/\1/p' contracts/script/output/devnet/eigenlayer_deployment_output.json

operator-generate-keys:
	@echo "Generating BLS keys"
	eigenlayer operator keys create --key-type bls --insecure operator
	@echo "Generating ECDSA keys"
	eigenlayer operator keys create --key-type ecdsa --insecure operator

operator-generate-config:
	@echo "Generating operator config"
	eigenlayer operator config create

operator-get-eth:
	@echo "Sending funds to operator address on devnet"
	@. ./scripts/fund_operator_devnet.sh

operator-register-with-eigen-layer:
	@echo "Registering operator with EigenLayer"
	@echo "" | eigenlayer operator register $(CONFIG_FILE)

operator-mint-mock-tokens:
	@echo "Minting tokens"
	. ./scripts/mint_mock_token.sh $(CONFIG_FILE) 1000

operator-deposit-into-mock-strategy:
	@echo "Depositing into strategy"
	$(eval STRATEGY_ADDRESS = $(shell jq -r '.erc20MockStrategy' contracts/script/output/devnet/strategy_deployment_output.json))

	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount 1000

operator-deposit-into-strategy:
	@echo "Depositing into strategy"
	@go run operator/cmd/main.go deposit-into-strategy \
		--config $(CONFIG_FILE) \
		--amount 1000

operator-register-with-aligned-layer:
	@echo "Registering operator with AlignedLayer"
	@go run operator/cmd/main.go register \
		--config $(CONFIG_FILE)

operator-deposit-and-register: operator-deposit-into-strategy operator-register-with-aligned-layer

operator-full-registration: operator-get-eth operator-register-with-eigen-layer operator-mint-mock-tokens operator-deposit-into-mock-strategy operator-register-with-aligned-layer

__TASK_SENDERS__:
 # TODO add a default proving system

send-plonk_bls12_381-proof: ## Send a PLONK BLS12_381 proof using the task sender
	@echo "Sending PLONK BLS12_381 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system plonk_bls12_381 \
		--proof task_sender/test_examples/bls12_381/plonk.proof \
		--public-input task_sender/test_examples/bls12_381/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/bls12_381/plonk.vk \
		--config config-files/config.yaml \
		--quorum-threshold 98 \
		--da $(DA_SOLUTION) \
		--batch-size 10 \
		2>&1 | zap-pretty

send-plonk_bls12_381-proof-loop: ## Send a PLONK BLS12_381 proof using the task sender every 10 seconds
	@echo "Sending PLONK BLS12_381 proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system plonk_bls12_381 \
		--proof task_sender/test_examples/bls12_381/plonk.proof \
		--public-input task_sender/test_examples/bls12_381/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/bls12_381/plonk.vk \
		--config config-files/config.yaml \
		--interval 10 \
		--da $(DA_SOLUTION) \
		--batch-size 10 \
		2>&1 | zap-pretty

send-plonk_bn254-proof: ## Send a PLONK BN254 proof using the task sender
	@echo "Sending PLONK BN254 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system plonk_bn254 \
		--proof task_sender/test_examples/bn254/plonk.proof \
		--public-input task_sender/test_examples/bn254/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/bn254/plonk.vk \
		--config config-files/config.yaml \
		--da $(DA_SOLUTION) \
		--batch-size 1 \
		2>&1 | zap-pretty

send-plonk_bn254-proof-loop: ## Send a PLONK BN254 proof using the task sender every 10 seconds
	@echo "Sending PLONK BN254 proof in a loop every 10 seconds..."
	@go run task_sender/cmd/main.go loop-tasks \
		--proving-system plonk_bn254 \
		--proof task_sender/test_examples/bn254/plonk.proof \
		--public-input task_sender/test_examples/bn254/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/bn254/plonk.vk \
		--config config-files/config.yaml \
		--interval 10 \
		--da $(DA_SOLUTION) \
		--batch-size 10 \
		2>&1 | zap-pretty

send-sp1-proof:
	@go run task_sender/cmd/main.go send-task \
    		--proving-system sp1 \
    		--proof task_sender/test_examples/sp1/sp1_fibonacci.proof \
    		--public-input task_sender/test_examples/sp1/elf/riscv32im-succinct-zkvm-elf \
    		--config config-files/config.yaml \
    		--da $(DA_SOLUTION) \
    		2>&1 | zap-pretty

__DEPLOYMENT__:
deploy-aligned-contracts: ## Deploy Aligned Contracts
	@echo "Deploying Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_aligned_contracts.sh

build-aligned-contracts:
	@cd contracts/src/core && forge build

__BUILD__:
build-binaries:
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
build-sp1-macos:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.dylib operator/sp1/lib/libsp1_verifier.dylib
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.a operator/sp1/lib/libsp1_verifier.a

build-sp1-linux:
	@cd operator/sp1/lib && cargo build --release
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.so operator/sp1/lib/libsp1_verifier.so
	@cp operator/sp1/lib/target/release/libsp1_verifier_ffi.a operator/sp1/lib/libsp1_verifier.a

test-sp1-rust-ffi:
	@echo "Testing SP1 Rust FFI source code..."
	@cd operator/sp1/lib && RUST_MIN_STACK=83886080 cargo t --release

test-sp1-go-bindings-macos: build-sp1-macos
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

test-sp1-go-bindings-linux: build-sp1-linux
	@echo "Testing SP1 Go bindings..."
	go test ./operator/sp1/... -v

# @cp -r task_sender/test_examples/sp1/fibonacci_proof_generator/script/elf task_sender/test_examples/sp1/
generate-sp1-fibonacci-proof:
	@cd task_sender/test_examples/sp1/fibonacci_proof_generator/script && RUST_LOG=info cargo run --release
	@mv task_sender/test_examples/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf task_sender/test_examples/sp1/elf
	@mv task_sender/test_examples/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof task_sender/test_examples/sp1/
	@echo "Fibonacci proof and ELF generated in task_sender/test_examples/sp1 folder"


