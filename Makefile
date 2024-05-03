.PHONY: help tests

OS := $(shell uname -s)

CONFIG_FILE?=config-files/config.yaml

ifeq ($(OS),Linux)
	JQ_INSTALL_CMD = sudo apt-get install jq
	YQ_INSTALL_CMD = sudo apt-get install yq
endif

ifeq ($(OS),Darwin)
	JQ_INSTALL_CMD = brew install jq
	YQ_INSTALL_CMD = brew install yq
endif

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install-foundry:
	curl -L https://foundry.paradigm.xyz | bash

install-eigenlayer-cli:
	@go install github.com/Layr-Labs/eigenlayer-cli/cmd/eigenlayer@latest

install-jq:
	$(JQ_INSTALL_CMD)

install-yq:
	$(YQ_INSTALL_CMD)

deps: ## Install deps
	git submodule update --init --recursive
	go install github.com/maoueh/zap-pretty@latest
	go install github.com/ethereum/go-ethereum/cmd/abigen@latest
	make install-foundry
	make install-eigenlayer-cli
	make install-jq
	make install-yq

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
	go run operator/cmd/main.go --config $(CONFIG_FILE) \
	2>&1 | zap-pretty

bindings:
	cd contracts && ./generate-go-bindings.sh

test:
	go test ./...

integration-test:
	./tests/integration_test.sh

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

	@go run operator/scripts/deposit_into_strategy/main.go \
		--config $(CONFIG_FILE) \
		--strategy-address $(STRATEGY_ADDRESS) \
		--amount 1000

operator-deposit-into-strategy:
	@echo "Depositing into strategy"
	@go run operator/scripts/deposit_into_strategy/main.go \
		--config $(CONFIG_FILE) \
		--amount 1000

operator-register-with-aligned-layer:
	@echo "Registering operator with AlignedLayer"
	@go run operator/scripts/register_with_aligned_layer/main.go \
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
		2>&1 | zap-pretty

send-plonk_bn254-proof: ## Send a PLONK BN254 proof using the task sender
	@echo "Sending PLONK BN254 proof..."
	@go run task_sender/cmd/main.go send-task \
		--proving-system plonk_bn254 \
		--proof task_sender/test_examples/bn254/plonk.proof \
		--public-input task_sender/test_examples/bn254/plonk_pub_input.pub \
		--verification-key task_sender/test_examples/bn254/plonk.vk \
		--config config-files/config.yaml \
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
		2>&1 | zap-pretty

__DEPLOYMENT__:
deploy-aligned-contracts: ## Deploy Aligned Contracts
	@echo "Deploying Aligned Contracts..."
	@. contracts/scripts/.env && . contracts/scripts/deploy_aligned_contracts.sh

build-aligned-contracts:
	@cd contracts/src/core && forge build
