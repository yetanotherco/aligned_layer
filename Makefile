.PHONY: help tests


help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

deps: ## Install deps
	git submodule update --init --recursive
	go install github.com/maoueh/zap-pretty@latest

install-foundry:
	curl -L https://foundry.paradigm.xyz | bash

anvil-deploy-eigen-contracts:
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil-deploy-aligned-contracts:
	@echo "Deploying Aligned Contracts..."
	. contracts/scripts/anvil/deploy_aligned_contracts.sh

anvil-deploy-all: anvil-deploy-eigen-contracts anvil-deploy-aligned-contracts

anvil-start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json 

# TODO: Allow enviroment variables / different configuration files
aggregator-start:
	@echo "Starting Aggregator..."
	go run aggregator/cmd/main.go --config aggregator/config/config.yaml \
		--aligned-layer-deployment contracts/script/output/devnet/alignedlayer_deployment_output.json \
		--ecdsa-private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

aggregator-send-dummy-responses:
	@echo "Sending dummy responses to Aggregator..."
	cd aggregator && go run dummy/submit_task_responses.go

bindings:
	cd contracts && ./generate-go-bindings.sh

test:
	go test ./...

__TASK_SENDERS__:
send-plonk-proof: ## Send a PLONK proof using the task sender
	go run task_sender/cmd/main.go \
		--system plonk \
		--proof task_sender/test_examples/proof.base64 \
		--public-input task_sender/test_examples/public_inputs.base64
