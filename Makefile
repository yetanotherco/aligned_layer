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

aggregator-start:
	@echo "Starting Aggregator..."
	cd aggregator && go run cmd/aggregator.go

aggregator-test:
	@echo "Testing Aggregator..."
	cd aggregator && go test ./...

aggregator-send-dummy-responses:
	@echo "Sending dummy responses to Aggregator..."
	cd aggregator && go run dummy/submit_task_responses.go
