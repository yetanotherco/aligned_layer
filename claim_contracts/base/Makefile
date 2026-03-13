-include .env

.PHONY: help deploy-base-sepolia deploy-base-mainnet verify-base-sepolia verify-base-mainnet bridge-l1-to-base-sepolia bridge-l1-to-base-mainnet

help: ## Show help for each of the Makefile recipes
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# OptimismMintableERC20Factory predeploy address (same on all OP Stack chains)
FACTORY=0x4200000000000000000000000000000000000012
TOKEN_NAME="Aligned Token"
TOKEN_SYMBOL="ALIGN"

# Default values
DEPLOYER_PRIVATE_KEY?=
BASE_SEPOLIA_RPC_URL?=https://sepolia.base.org
BASE_MAINNET_RPC_URL?=https://mainnet.base.org

# L1 token addresses (set these or pass via env/CLI)
L1_TOKEN_SEPOLIA?=0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A
L1_TOKEN_MAINNET?=0x0000000000000000000000000000000000000000

# L2 token addresses
L2_TOKEN_SEPOLIA?=0x4AAcFbc2C31598a560b285dB20966E00B73F9F81
L2_TOKEN_MAINNET?=0x0000000000000000000000000000000000000000

# L1 Standard Bridge addresses
L1_BRIDGE_SEPOLIA=0xfd0Bf71F60660E2f608ed56e1659C450eB113120
L1_BRIDGE_MAINNET=0x3154Cf16ccdb4C6d922629664174b904d80F2C35

# L1 RPC URLs
L1_SEPOLIA_RPC_URL?=https://ethereum-sepolia-rpc.publicnode.com
L1_MAINNET_RPC_URL?=https://ethereum-rpc.publicnode.com

# Deployments
# Following: https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token#create-an-l2-erc-20-token
# The deployed L2 token address is extracted from the OptimismMintableERC20Created event logs.

deploy-base-sepolia: ## Deploy ALIGN token on BaseSepolia via OptimismMintableERC20Factory
	@echo "Deploying ALIGN on BaseSepolia..."
	@echo "L1 Remote Token: $(L1_TOKEN_SEPOLIA)"
	@echo "Factory: $(FACTORY)"
	@L2_TOKEN=$$(cast send $(FACTORY) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_SEPOLIA) $(TOKEN_NAME) $(TOKEN_SYMBOL) \
		--private-key $(DEPLOYER_PRIVATE_KEY) \
		--rpc-url $(BASE_SEPOLIA_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "---------------------------------------------" && \
	echo "ALIGN L2 Token deployed at: $$L2_TOKEN" && \
	echo "---------------------------------------------"

deploy-base-mainnet: ## Deploy ALIGN token on BaseMainnet via OptimismMintableERC20Factory
	@echo "Deploying ALIGN on BaseMainnet..."
	@echo "L1 Remote Token: $(L1_TOKEN_MAINNET)"
	@echo "Factory: $(FACTORY)"
	@L2_TOKEN=$$(cast send $(FACTORY) \
		"createOptimismMintableERC20(address,string,string)" \
		$(L1_TOKEN_MAINNET) $(TOKEN_NAME) $(TOKEN_SYMBOL) \
		--private-key $(DEPLOYER_PRIVATE_KEY) \
		--rpc-url $(BASE_MAINNET_RPC_URL) \
		--json | jq -r '.logs[0].topics[2]' | cast parse-bytes32-address) && \
	echo "---------------------------------------------" && \
	echo "ALIGN L2 Token deployed at: $$L2_TOKEN" && \
	echo "---------------------------------------------"

# Verification

verify-base-sepolia: ## Verify the deployed L2 token on BaseSepolia (requires L2_TOKEN)
	@echo "Verifying L2 token deployment on BaseSepolia..."
	@echo "--- Token metadata ---"
	cast call $(L2_TOKEN) "name()(string)" --rpc-url $(BASE_SEPOLIA_RPC_URL)
	cast call $(L2_TOKEN) "symbol()(string)" --rpc-url $(BASE_SEPOLIA_RPC_URL)
	cast call $(L2_TOKEN) "decimals()(uint8)" --rpc-url $(BASE_SEPOLIA_RPC_URL)
	@echo "--- Bridge configuration ---"
	cast call $(L2_TOKEN) "REMOTE_TOKEN()(address)" --rpc-url $(BASE_SEPOLIA_RPC_URL)
	cast call $(L2_TOKEN) "BRIDGE()(address)" --rpc-url $(BASE_SEPOLIA_RPC_URL)
	@echo "--- Total supply (should be 0 before any bridging) ---"
	cast call $(L2_TOKEN) "totalSupply()(uint256)" --rpc-url $(BASE_SEPOLIA_RPC_URL)

verify-base-mainnet: ## Verify the deployed L2 token on BaseMainnet (requires L2_TOKEN)
	@echo "Verifying L2 token deployment on BaseMainnet..."
	@echo "--- Token metadata ---"
	cast call $(L2_TOKEN) "name()(string)" --rpc-url $(BASE_MAINNET_RPC_URL)
	cast call $(L2_TOKEN) "symbol()(string)" --rpc-url $(BASE_MAINNET_RPC_URL)
	cast call $(L2_TOKEN) "decimals()(uint8)" --rpc-url $(BASE_MAINNET_RPC_URL)
	@echo "--- Bridge configuration ---"
	cast call $(L2_TOKEN) "REMOTE_TOKEN()(address)" --rpc-url $(BASE_MAINNET_RPC_URL)
	cast call $(L2_TOKEN) "BRIDGE()(address)" --rpc-url $(BASE_MAINNET_RPC_URL)
	@echo "--- Total supply (should be 0 before any bridging) ---"
	cast call $(L2_TOKEN) "totalSupply()(uint256)" --rpc-url $(BASE_MAINNET_RPC_URL)

# Bridging L1 -> L2
# Step 1: Approve the L1StandardBridge to spend ALIGN tokens
# Step 2: Deposit tokens via the bridge (tokens appear on Base after ~20 min)
# Requires: AMOUNT (in wei), USER_PRIVATE_KEY
AMOUNT?=1000000000000000000 # Default to 1 ALIGN (18 decimals)
USER_PRIVATE_KEY?=

bridge-l1-to-base-sepolia: ## Bridge ALIGN tokens from Sepolia L1 to BaseSepolia (requires AMOUNT)
	@echo "Step 1: Approving L1StandardBridge to spend $(AMOUNT) ALIGN on Sepolia..."
	cast send $(L1_TOKEN_SEPOLIA) \
		"approve(address,uint256)" \
		$(L1_BRIDGE_SEPOLIA) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) \
		--rpc-url $(L1_SEPOLIA_RPC_URL)
	@echo "Step 2: Depositing $(AMOUNT) ALIGN to BaseSepolia..."
	cast send $(L1_BRIDGE_SEPOLIA) \
		"depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_SEPOLIA) $(L2_TOKEN_SEPOLIA) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) \
		--rpc-url $(L1_SEPOLIA_RPC_URL)
	@echo "---------------------------------------------"
	@echo "Bridge initiated. Tokens will appear on BaseSepolia in ~20 minutes."
	@echo "---------------------------------------------"

bridge-l1-to-base-mainnet: ## Bridge ALIGN tokens from Ethereum L1 to Base (requires AMOUNT)
	@echo "Step 1: Approving L1StandardBridge to spend $(AMOUNT) ALIGN on Mainnet..."
	cast send $(L1_TOKEN_MAINNET) \
		"approve(address,uint256)" \
		$(L1_BRIDGE_MAINNET) $(AMOUNT) \
		--private-key $(USER_PRIVATE_KEY) \
		--rpc-url $(L1_MAINNET_RPC_URL)
	@echo "Step 2: Depositing $(AMOUNT) ALIGN to Base..."
	cast send $(L1_BRIDGE_MAINNET) \
		"depositERC20(address,address,uint256,uint32,bytes)" \
		$(L1_TOKEN_MAINNET) $(L2_TOKEN_MAINNET) $(AMOUNT) 200000 0x \
		--private-key $(USER_PRIVATE_KEY) \
		--rpc-url $(L1_MAINNET_RPC_URL)
	@echo "---------------------------------------------"
	@echo "Bridge initiated. Tokens will appear on Base in ~20 minutes."
	@echo "---------------------------------------------"
