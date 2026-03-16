-include .env
export

.PHONY: help deploy-base-sepolia deploy-base-mainnet verify bridge-l1-to-base-sepolia bridge-l1-to-base-mainnet withdraw-base-to-l1-sepolia withdraw-base-to-l1-mainnet prove-withdrawal-sepolia prove-withdrawal-mainnet finalize-withdrawal-sepolia finalize-withdrawal-mainnet

help: ## Show help
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# OptimismMintableERC20Factory
# Mainnet uses Base-specific factory to avoid address conflicts with Optimism.
# Source: https://docs.base.org/chain/base-contracts
FACTORY_SEPOLIA=0x4200000000000000000000000000000000000012
FACTORY_MAINNET=0xF10122D428B4bc8A9d050D06a2037259b4c4B83B

# --- Deployment ---

deploy-base-sepolia: ## Deploy ALIGN on BaseSepolia
	@L2_TOKEN=$$(cast send $(FACTORY_SEPOLIA) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_SEPOLIA) "Aligned Token" "ALIGN" \
		--private-key $(DEPLOYER_PRIVATE_KEY) \
		--rpc-url $(BASE_SEPOLIA_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "L2 Token deployed at: $$L2_TOKEN"

deploy-base-mainnet: ## Deploy ALIGN on BaseMainnet
	@L2_TOKEN=$$(cast send $(FACTORY_MAINNET) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_MAINNET) "Aligned Token" "ALIGN" \
		--interactive \
		--rpc-url $(BASE_MAINNET_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "L2 Token deployed at: $$L2_TOKEN"

# --- Verification ---

verify: ## Verify L2 token (requires L2_TOKEN, RPC_URL)
	@cast call $(L2_TOKEN) "name()(string)" --rpc-url $(RPC_URL)
	@cast call $(L2_TOKEN) "symbol()(string)" --rpc-url $(RPC_URL)
	@cast call $(L2_TOKEN) "decimals()(uint8)" --rpc-url $(RPC_URL)
	@cast call $(L2_TOKEN) "REMOTE_TOKEN()(address)" --rpc-url $(RPC_URL)
	@cast call $(L2_TOKEN) "BRIDGE()(address)" --rpc-url $(RPC_URL)
	@cast call $(L2_TOKEN) "totalSupply()(uint256)" --rpc-url $(RPC_URL)

# --- Bridging L1 -> Base ---

bridge-l1-to-base-sepolia: ## Bridge ALIGN from Sepolia to BaseSepolia (requires AMOUNT)
	cast send $(L1_TOKEN_SEPOLIA) "approve(address,uint256)" $(L1_BRIDGE_SEPOLIA) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)
	cast send $(L1_BRIDGE_SEPOLIA) "depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_SEPOLIA) $(L2_TOKEN_SEPOLIA) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)

bridge-l1-to-base-mainnet: ## Bridge ALIGN from Ethereum to Base (requires AMOUNT)
	cast send $(L1_TOKEN_MAINNET) "approve(address,uint256)" $(L1_BRIDGE_MAINNET) $(AMOUNT) \
		--interactive --rpc-url $(L1_MAINNET_RPC_URL)
	cast send $(L1_BRIDGE_MAINNET) "depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_MAINNET) $(L2_TOKEN_MAINNET) $(AMOUNT) 200000 0x \
		--interactive --rpc-url $(L1_MAINNET_RPC_URL)

bridge-l1-to-base-sepolia-to: ## Bridge ALIGN from Sepolia to BaseSepolia to a different address (requires AMOUNT, TO)
	cast send $(L1_TOKEN_SEPOLIA) "approve(address,uint256)" $(L1_BRIDGE_SEPOLIA) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)
	cast send $(L1_BRIDGE_SEPOLIA) "depositERC20To(address,address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_SEPOLIA) $(L2_TOKEN_SEPOLIA) $(TO) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(L1_SEPOLIA_RPC_URL)

bridge-l1-to-base-mainnet-to: ## Bridge ALIGN from Ethereum to Base to a different address (requires AMOUNT, TO)
	cast send $(L1_TOKEN_MAINNET) "approve(address,uint256)" $(L1_BRIDGE_MAINNET) $(AMOUNT) \
		--interactive --rpc-url $(L1_MAINNET_RPC_URL)
	cast send $(L1_BRIDGE_MAINNET) "depositERC20To(address,address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_MAINNET) $(L2_TOKEN_MAINNET) $(TO) $(AMOUNT) 200000 0x \
		--interactive --rpc-url $(L1_MAINNET_RPC_URL)

# --- Bridging Base -> L1 (withdrawal) ---
# This initiates the withdrawal on L2. After this, you must:
#   1. Wait ~1 hour for the L2 output to be proposed
#   2. Prove the withdrawal on L1 (requires Optimism SDK or Base Bridge UI)
#   3. Wait 7 days (challenge period)
#   4. Finalize the withdrawal on L1
# L2StandardBridge predeploy: 0x4200000000000000000000000000000000000010

withdraw-base-to-l1-sepolia: ## Initiate ALIGN withdrawal from BaseSepolia to Sepolia (requires AMOUNT)
	cast send 0x4200000000000000000000000000000000000010 \
		"withdraw(address,uint256,uint32,bytes)" \
		$(L2_TOKEN_SEPOLIA) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(BASE_SEPOLIA_RPC_URL)

withdraw-base-to-l1-mainnet: ## Initiate ALIGN withdrawal from Base to Ethereum (requires AMOUNT)
	cast send 0x4200000000000000000000000000000000000010 \
		"withdraw(address,uint256,uint32,bytes)" \
		$(L2_TOKEN_MAINNET) $(AMOUNT) 200000 0x \
		--interactive --rpc-url $(BASE_MAINNET_RPC_URL)

withdraw-base-to-l1-sepolia-to: ## Initiate ALIGN withdrawal from BaseSepolia to a different address on Sepolia (requires AMOUNT, TO)
	cast send 0x4200000000000000000000000000000000000010 \
		"withdrawTo(address,address,uint256,uint32,bytes)" \
		$(L2_TOKEN_SEPOLIA) $(TO) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) --rpc-url $(BASE_SEPOLIA_RPC_URL)

withdraw-base-to-l1-mainnet-to: ## Initiate ALIGN withdrawal from Base to a different address on Ethereum (requires AMOUNT, TO)
	cast send 0x4200000000000000000000000000000000000010 \
		"withdrawTo(address,address,uint256,uint32,bytes)" \
		$(L2_TOKEN_MAINNET) $(TO) $(AMOUNT) 200000 0x \
		--interactive --rpc-url $(BASE_MAINNET_RPC_URL)

# --- Prove & Finalize (requires npm install) ---

prove-withdrawal-sepolia: ## Prove withdrawal on L1 Sepolia (requires TX_HASH)
	npx tsx scripts/withdraw.ts prove --tx-hash $(TX_HASH) --network sepolia

prove-withdrawal-mainnet: ## Prove withdrawal on L1 Mainnet (requires TX_HASH)
	npx tsx scripts/withdraw.ts prove --tx-hash $(TX_HASH) --network mainnet

finalize-withdrawal-sepolia: ## Finalize withdrawal on L1 Sepolia (requires TX_HASH)
	npx tsx scripts/withdraw.ts finalize --tx-hash $(TX_HASH) --network sepolia

finalize-withdrawal-mainnet: ## Finalize withdrawal on L1 Mainnet (requires TX_HASH)
	npx tsx scripts/withdraw.ts finalize --tx-hash $(TX_HASH) --network mainnet
