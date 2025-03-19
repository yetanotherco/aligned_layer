.PHONY: help deploy-aligned-token-implementation deploy-aligned-token-proxy deploy-claimable-airdrop-implementation deploy-claimable-airdrop-proxy upgrade-aligned-token-implementation aligned-token-proxy-deploy-data aligned-token-init-data aligned-token-upgrade-data aligned-token-create2 aligned-token-proxy-create2


help: ## 📚 Show help for each of the Makefile recipes
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# Calldata

calldata-update-merkle-root: ## 💾 Calldata for the method `updateMerkleRoot` to use in a transaction
	cast calldata "updateMerkleRoot(bytes32)" $(MERKLE_ROOT)

calldata-update-limit-timestamp: ## 💾 Calldata for the method `extendClaimPeriod` to use in a transaction
	cast calldata "extendClaimPeriod(uint256)" $(LIMIT_TIMESTAMP)

calldata-approve-spending: ## 💾 Calldata for the method `approve` to use in a transaction
	cast calldata "approve(address,uint256)" $(CLAIM_PROXY_ADDRESS) 2600000000000000000000000000

calldata-unpause: ## 💾 Calldata for the method `unpause` to use in a transaction
	cast calldata "unpause()"

calldata-pause: ## 💾 Calldata for the method `pause` to use in a transaction
	cast calldata "pause()"

# Deployments

RPC_URL?=http://localhost:8545
DEPLOYER_PRIVATE_KEY?=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
CONFIG?=example
deploy-token: ## 🚀 Deploy the token contract
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" \
	$(CONFIG) \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast \
	--verbosity 3

deploy-token-testnet: ## 🚀 Deploy the token contract
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" sepolia \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast \
	--verbosity 3 \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-token-prod: ## 🚀 Deploy the token contract
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" \
	$(PROD_CONFIG) \
	--keystore $(KEYSTORE_PATH) \
	--rpc-url $(PROD_RPC_URL) \
	--broadcast \
	--verbosity 3

deploy-claimable-local: ## 🚀 Deploy the airdrop contract in localnet
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" example \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url http://localhost:8545 \
	--broadcast

deploy-claimable-testnet: ## 🚀 Deploy the airdrop contract in Sepolia
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" sepolia \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-claimable-mainnet: ## 🚀 Deploy the airdrop contract in Mainnet
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" mainnet \
	--keystore $(KEYSTORE_PATH) \
	--rpc-url https://eth.llamarpc.com \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)


update_token_proxy:
	@NEW_TOKEN_PROXY=$$(jq -r '.tokenProxy' "script-out/deployed_token_addresses.json") && \
	jq --arg new_proxy "$$NEW_TOKEN_PROXY" '.tokenProxy = $$new_proxy' $(CONFIG) > $(CONFIG).tmp \
	&& mv $(CONFIG).tmp $(CONFIG)

upgrade-token: ## 🚀 Upgrade the token contract
	cd script && \
	forge script UpgradeToken.s.sol \
	--sig "run(string)" \
	$(CONFIG) \
	--private-key $(PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast \
	--verbosity 3

# Miscellaneous

DISTRIBUTOR_PRIVATE_KEY?=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
OWNER_PRIVATE_KEY=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
AIRDROP=0xBC9129Dc0487fc2E169941C75aABC539f208fb01
TOKEN=0x2E983A1Ba5e8b38AAAeC4B440B9dDcFBf72E15d1

approve-claimable: ## 🚀 Approve the ClaimableAirdrop contract to spend the token
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(DISTRIBUTOR_PRIVATE_KEY) \
		$(TOKEN) \
		'approve(address,uint256)' $(AIRDROP) 2600000000000000000000000000

claimable-update-root: ## 🚀 Update the merkle root of the ClaimableAirdrop contract
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(OWNER_PRIVATE_KEY) \
		$(AIRDROP) \
		'updateMerkleRoot(bytes32)' $(MERKLE_ROOT)

claimable-get-root: ## 🚀 Get the merkle root of the ClaimableAirdrop contract
	cast call \
		--rpc-url $(RPC_URL) \
		$(AIRDROP) \
		"claimMerkleRoot()(bytes32)"

claimable-update-timestamp: ## 🚀 Update the limit timestamp of the ClaimableAirdrop contract
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(OWNER_PRIVATE_KEY) \
		$(AIRDROP) \
		'extendClaimPeriod(uint256)' $(TIMESTAMP)

claimable-get-timestamp: ## 🚀 Get the limit timestamp of the ClaimableAirdrop contract
	cast call \
		--rpc-url $(RPC_URL) \
		$(AIRDROP) \
		"limitTimestampToClaim()(uint256)"

claimable-pause:
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(OWNER_PRIVATE_KEY) \
		$(AIRDROP) "pause()"

claimable-unpause:
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(OWNER_PRIVATE_KEY) \
		$(AIRDROP) "unpause()"

# Upgrades

upgrade-aligned-token-implementation: ## 🚀 Upgrade the AlignedToken implementation contract
	cd script/aligned_token && \
	forge script UpgradeAlignedTokenImplementation.s.sol \
	--sig "function run(address,address,uint256,address,address,address,address,uint256)" \
	$(PROXY) $(IMPLEMENTATION) $(VERSION) $(OWNER) $(BENEFICIARY1) $(BENEFICIARY2) $(BENEFICIARY3) $(MINT)\
	--rpc-url $(RPC_URL) \
	--private-key $(PRIVATE_KEY) \
	--broadcast

# Test targets

test-token:
	cast call $(ADDRESS) "name()(string)" --rpc-url $(RPC_URL)
	cast call $(ADDRESS) "symbol()(string)" --rpc-url $(RPC_URL)
	cast call $(ADDRESS) "totalSupply()(uint256)" --rpc-url $(RPC_URL)

# The following target needs the proof API running on localhost:4000
AMOUNT_TO_CLAIM=$(shell curl -S -H "Content-Type: application/json" http://localhost:4000/api/proof/\$(CLAIMER) | jq -r .amount)
MERKLE_PROOF_TO_CLAIM=$(shell curl -S -H "Content-Type: application/json" http://localhost:4000/api/proof/\$(CLAIMER) | jq .proof | tr -d '"\n ')
test-claim:
	cast send $(AIRDROP) --private-key $(CLAIMER_PRIVATE_KEY) "claim(uint256,bytes32[])" $(AMOUNT_TO_CLAIM) "$(MERKLE_PROOF_TO_CLAIM)" --rpc-url $(RPC_URL)

test-claimed:
	cast call $(AIRDROP) "hasClaimed(address)(bool)" $(CLAIMER) --rpc-url $(RPC_URL)
	cast balance --erc20 $(TOKEN) $(CLAIMER) --rpc-url $(RPC_URL)

OWNER_PRIVATE_KEY?=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

test-airdrop:
	cast call $(AIRDROP) "paused()(bool)" --rpc-url $(RPC_URL)
	cast call $(AIRDROP) "owner()(address)" --rpc-url $(RPC_URL)

# Requires MERKLE_ROOT, TIMESTAMP
deploy-example: deploy-token deploy-claimable-local claimable-update-root claimable-update-timestamp approve-claimable claimable-unpause
