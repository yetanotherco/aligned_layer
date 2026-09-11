.PHONY: help launch-env \
	calldata-update-merkle-root calldata-update-limit-timestamp calldata-approve-spending calldata-unpause calldata-pause \
	deploy-token deploy-token-sepolia deploy-token-mainnet \
	deploy-claimable-local deploy-claimable-sepolia deploy-claimable-base-sepolia deploy-claimable-mainnet deploy-claimable-base-mainnet \
	update_token_proxy upgrade-token enable-claimability \
	approve-claimable claimable-update-root claimable-get-root claimable-update-timestamp claimable-get-timestamp claimable-pause claimable-unpause \
	upgrade-aligned-token-implementation \
	test-token test-claim test-claimed test-airdrop \
	deploy-example


# ============================================================================
# Configuration — override any of these on the command line, e.g.
#   make deploy-claimable-sepolia DEPLOYER_PRIVATE_KEY=0x... ETHERSCAN_API_KEY=...
# (ETHERSCAN_API_KEY, MERKLE_ROOT, TIMESTAMP, CLAIM_PROXY_ADDRESS,
#  LIMIT_TIMESTAMP, etc. have no default and are passed in per invocation.)
# The mainnet deploy targets sign with --interactive, so they prompt for the key instead of
# reading it from a variable or a keystore path.
# ============================================================================

# RPC endpoints (per network)
RPC_URL ?= http://localhost:8545
SEPOLIA_RPC_URL ?= https://ethereum-sepolia-rpc.publicnode.com
BASE_SEPOLIA_RPC_URL ?= https://sepolia.base.org
MAINNET_RPC_URL ?= https://ethereum-rpc.publicnode.com
BASE_MAINNET_RPC_URL ?= https://mainnet.base.org

# Signing keys (anvil defaults, for local use)
DEPLOYER_PRIVATE_KEY ?= 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
DISTRIBUTOR_PRIVATE_KEY ?= 0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
OWNER_PRIVATE_KEY ?= 0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a

# Deploy config + contract addresses (anvil defaults)
CONFIG ?= example
AIRDROP ?= 0xBC9129Dc0487fc2E169941C75aABC539f208fb01
TOKEN ?= 0x2E983A1Ba5e8b38AAAeC4B440B9dDcFBf72E15d1
APPROVE_AMOUNT ?= 2600000000000000000000000000

# Test inputs — require the proof API running on localhost:4000
AMOUNT_TO_CLAIM = $(shell curl -S -H "Content-Type: application/json" http://localhost:4000/api/proof/\$(CLAIMER) | jq -r .amount)
MERKLE_PROOF_TO_CLAIM = $(shell curl -S -H "Content-Type: application/json" http://localhost:4000/api/proof/\$(CLAIMER) | jq .proof | tr -d '"\n ')

# Mainnet launch environment — the file the launch runbook tells the operator to keep and source.
# The mainnet deploy targets depend on `launch-env`, so its values (and anything still MISSING) are
# printed right before the deploy prompts for the key, instead of relying on the operator having
# sourced the right file in the right shell.
LAUNCH_ENV ?= $(HOME)/align-launch.env
LAUNCH_ENV_VARS = ETH_RPC BASE_RPC ETH_TOKEN BASE_TOKEN CLAIM_ADMIN_SAFE ETH_DISTRIBUTOR_SAFE \
	BASE_DISTRIBUTOR_SAFE ETH_TREASURY_SAFE DEPLOYER START_TIMESTAMP ETH_DEADLINE \
	BASE_DEADLINE SMOKE_TEST_ADDRESS ETH_AMOUNT BASE_AMOUNT ETH_CLAIM BASE_CLAIM \
	ETH_ROOT BASE_ROOT
# Reported as set/MISSING only — never printed. The deploy targets read ETHERSCAN_API_KEY straight
# from the environment, so an unset one only surfaces as a forge argument error.
LAUNCH_ENV_SECRETS = ETHERSCAN_API_KEY

help: ## 📚 Show help for each of the Makefile recipes
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

launch-env: ## 🔎 Print the launch runbook variables from $(LAUNCH_ENV) (MISSING when unset)
	@if [ ! -r "$(LAUNCH_ENV)" ]; then \
		echo "launch env file not found: $(LAUNCH_ENV)"; \
		echo "Create it from the 'Environment' section of the launch runbook,"; \
		echo "or pass LAUNCH_ENV=/dev/null to run without it."; \
		exit 1; \
	fi; \
	echo "launch env: $(LAUNCH_ENV)"; \
	. "$(LAUNCH_ENV)"; \
	missing=0; \
	for v in $(LAUNCH_ENV_VARS); do \
		eval "val=\$${$$v:-}"; \
		if [ -n "$$val" ]; then \
			printf '  %-24s %s\n' "$$v" "$$val"; \
		else \
			printf '  %-24s MISSING\n' "$$v"; \
			missing=$$((missing + 1)); \
		fi; \
	done; \
	for v in $(LAUNCH_ENV_SECRETS); do \
		eval "val=\$${$$v:-}"; \
		if [ -n "$$val" ]; then \
			printf '  %-24s set\n' "$$v"; \
		else \
			printf '  %-24s MISSING\n' "$$v"; \
			missing=$$((missing + 1)); \
		fi; \
	done; \
	echo "($$missing missing — expected for values a later runbook step still has to produce)"

# Calldata

calldata-update-merkle-root: ## 💾 Calldata for the method `updateMerkleRoot` to use in a transaction
	cast calldata "updateMerkleRoot(bytes32)" $(MERKLE_ROOT)

calldata-update-limit-timestamp: ## 💾 Calldata for the method `extendClaimPeriod` to use in a transaction
	cast calldata "extendClaimPeriod(uint256)" $(LIMIT_TIMESTAMP)

calldata-approve-spending: ## 💾 Calldata for the method `approve` to use in a transaction (override APPROVE_AMOUNT)
	cast calldata "approve(address,uint256)" $(CLAIM_PROXY_ADDRESS) $(APPROVE_AMOUNT)

calldata-unpause: ## 💾 Calldata for the method `unpause` to use in a transaction
	cast calldata "unpause()"

calldata-pause: ## 💾 Calldata for the method `pause` to use in a transaction
	cast calldata "pause()"

# Deployments

deploy-token: ## 🚀 Deploy the token contract
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" \
	$(CONFIG) \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast

deploy-token-sepolia: ## 🚀 Deploy the token contract in Sepolia
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" sepolia \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(SEPOLIA_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-token-mainnet: ## 🚀 Deploy the token contract in Mainnet
	cd script && \
	forge script DeployAlignedToken.s.sol \
	--sig "run(string)" \
	mainnet \
	--interactive \
	--rpc-url $(MAINNET_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY) \
	--verbosity 3

deploy-claimable-local: ## 🚀 Deploy the airdrop contract in localnet
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" example \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url http://localhost:8545 \
	--broadcast

deploy-claimable-sepolia: ## 🚀 Deploy the airdrop contract in Sepolia
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" sepolia \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(SEPOLIA_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-claimable-base-sepolia: ## 🚀 Deploy the airdrop contract in Base Sepolia
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" base-sepolia \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(BASE_SEPOLIA_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-claimable-mainnet: launch-env ## 🚀 Deploy the airdrop contract in Mainnet
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" mainnet \
	--interactive \
	--rpc-url $(MAINNET_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)

deploy-claimable-base-mainnet: launch-env ## 🚀 Deploy the airdrop contract in Base Mainnet
	cd script && \
	forge script DeployClaimableAirdrop.s.sol \
	--sig "run(string)" base-mainnet \
	--interactive \
	--rpc-url $(BASE_MAINNET_RPC_URL) \
	--broadcast \
	--verify \
	--etherscan-api-key $(ETHERSCAN_API_KEY)


# TODO: broken — reads script-out/deployed_token_addresses.json (never generated by the deploy
# scripts) and treats $(CONFIG) as a file path. Fix or remove.
update_token_proxy:
	@NEW_TOKEN_PROXY=$$(jq -r '.tokenProxy' "script-out/deployed_token_addresses.json") && \
	jq --arg new_proxy "$$NEW_TOKEN_PROXY" '.tokenProxy = $$new_proxy' $(CONFIG) > $(CONFIG).tmp \
	&& mv $(CONFIG).tmp $(CONFIG)

# TODO: broken — runs UpgradeToken.s.sol, which does not exist in script/. Fix or remove
# (see the README "Upgrades" section, which expects the upgrade script to be written ad-hoc).
upgrade-token: ## 🚀 Upgrade the token contract
	cd script && \
	forge script UpgradeToken.s.sol \
	--sig "run(string)" \
	$(CONFIG) \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--rpc-url $(RPC_URL) \
	--broadcast \
	--verbosity 3

# Miscellaneous

enable-claimability: claimable-update-root claimable-update-timestamp approve-claimable claimable-unpause ## 🚀 Run every tx needed to start claiming (set root, set deadline, approve, unpause)

approve-claimable: ## 🚀 Approve the ClaimableAirdrop contract to spend the token
	cast send \
		--rpc-url $(RPC_URL) \
		--private-key $(DISTRIBUTOR_PRIVATE_KEY) \
		$(TOKEN) \
		'approve(address,uint256)' $(AIRDROP) $(APPROVE_AMOUNT)

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

# TODO: broken — script/aligned_token/UpgradeAlignedTokenImplementation.s.sol doesn't exist and
# --sig has a stray leading "function". Fix or remove.
upgrade-aligned-token-implementation: ## 🚀 Upgrade the AlignedToken implementation contract
	cd script/aligned_token && \
	forge script UpgradeAlignedTokenImplementation.s.sol \
	--sig "function run(address,address,uint256,address,address,address,address,uint256)" \
	$(PROXY) $(IMPLEMENTATION) $(VERSION) $(OWNER) $(BENEFICIARY1) $(BENEFICIARY2) $(BENEFICIARY3) $(MINT)\
	--rpc-url $(RPC_URL) \
	--private-key $(DEPLOYER_PRIVATE_KEY) \
	--broadcast

# Test targets

test-token:
	cast call $(ADDRESS) "name()(string)" --rpc-url $(RPC_URL)
	cast call $(ADDRESS) "symbol()(string)" --rpc-url $(RPC_URL)
	cast call $(ADDRESS) "totalSupply()(uint256)" --rpc-url $(RPC_URL)

# TODO: outdated — claim is now claim(uint256 amount,uint256 validFrom,bytes32[] proof) and the
# proof comes from the web app's /api/wallets/<address>, not /api/proof/<claimer>. Fix.
test-claim:
	cast send $(AIRDROP) --private-key $(CLAIMER_PRIVATE_KEY) "claim(uint256,bytes32[])" $(AMOUNT_TO_CLAIM) "$(MERKLE_PROOF_TO_CLAIM)" --rpc-url $(RPC_URL)

# TODO: outdated — hasClaimed is now hasClaimed(bytes32 leaf), not hasClaimed(address). Fix.
test-claimed:
	cast call $(AIRDROP) "hasClaimed(address)(bool)" $(CLAIMER) --rpc-url $(RPC_URL)
	cast balance --erc20 $(TOKEN) $(CLAIMER) --rpc-url $(RPC_URL)

test-airdrop:
	cast call $(AIRDROP) "paused()(bool)" --rpc-url $(RPC_URL)
	cast call $(AIRDROP) "owner()(address)" --rpc-url $(RPC_URL)

# Requires MERKLE_ROOT, TIMESTAMP
deploy-example: deploy-token deploy-claimable-local enable-claimability
