.PHONY: help tests

FORGE_INSTALLED := $(shell forge --version dot 2> /dev/null)

help:
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

deps: ## Install deps
	git submodule update --init --recursive
	go install github.com/maoueh/zap-pretty@latest
ifndef FORGE_INSTALLED
	curl -L https://foundry.paradigm.xyz | bash
	foundryup
endif


anvil-deploy-eigen-contracts:
	@echo "Deploying Eigen Contracts..."
	. contracts/scripts/anvil/deploy_eigen_contracts.sh

anvil-start:
	@echo "Starting Anvil..."
	anvil --load-state contracts/scripts/anvil/state/eigenlayer-deployed-anvil-state.json 

