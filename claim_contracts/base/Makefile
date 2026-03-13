-include .env

.PHONY: help deploy-base-sepolia deploy-base-mainnet verify bridge-l1-to-base-sepolia bridge-l1-to-base-mainnet

help: ## Show help
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# Factory predeploy (same on all OP Stack chains)
FACTORY=0x4200000000000000000000000000000000000012

# --- Deployment ---

deploy-base-sepolia: ## Deploy ALIGN on BaseSepolia
	@L2_TOKEN=$$(cast send $(FACTORY) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_SEPOLIA) "Aligned Token" "ALIGN" \
		--private-key $(DEPLOYER_PRIVATE_KEY) \
		--rpc-url $(BASE_SEPOLIA_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "L2 Token deployed at: $$L2_TOKEN"

deploy-base-mainnet: ## Deploy ALIGN on BaseMainnet
	@L2_TOKEN=$$(cast send $(FACTORY) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_MAINNET) "Aligned Token" "ALIGN" \
		--private-key $(DEPLOYER_PRIVATE_KEY) \
		--rpc-url $(BASE_MAINNET_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "L2 Token deployed at: $$L2_TOKEN"

# --- Verification ---

verify: ## Verify L2 token (requires L2_TOKEN, RPC_URL)
	cast call $(L2_TOKEN) "name()(string)" --rpc-url $(RPC_URL)
	cast call $(L2_TOKEN) "symbol()(string)" --rpc-url $(RPC_URL)
	cast call $(L2_TOKEN) "decimals()(uint8)" --rpc-url $(RPC_URL)
	cast call $(L2_TOKEN) "REMOTE_TOKEN()(address)" --rpc-url $(RPC_URL)
	cast call $(L2_TOKEN) "BRIDGE()(address)" --rpc-url $(RPC_URL)
	cast call $(L2_TOKEN) "totalSupply()(uint256)" --rpc-url $(RPC_URL)

# --- Bridging L1 -> Base ---

bridge-l1-to-base-sepolia: ## Bridge ALIGN from Sepolia to BaseSepolia (requires AMOUNT)
	cast send $(L1_TOKEN_SEPOLIA) "approve(address,uint256)" $(L1_BRIDGE_SEPOLIA) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)
	cast send $(L1_BRIDGE_SEPOLIA) "depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_SEPOLIA) $(L2_TOKEN_SEPOLIA) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)

bridge-l1-to-base-mainnet: ## Bridge ALIGN from Ethereum to Base (requires AMOUNT)
	cast send $(L1_TOKEN_MAINNET) "approve(address,uint256)" $(L1_BRIDGE_MAINNET) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_MAINNET_RPC_URL)
	cast send $(L1_BRIDGE_MAINNET) "depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_MAINNET) $(L2_TOKEN_MAINNET) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_MAINNET_RPC_URL)
