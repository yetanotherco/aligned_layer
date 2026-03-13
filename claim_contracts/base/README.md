# ALIGN Token on Base L2

Deployment and bridging of the Aligned Token (ALIGN) on Base using the [OP Standard Bridge](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token).

## Setup

```bash
cp .env.example .env
# Fill in DEPLOYER_PRIVATE_KEY and USER_PRIVATE_KEY
```

## Deploy

The L2 token is created via the `OptimismMintableERC20Factory` predeploy at `0x4200000000000000000000000000000000000012`. No custom contract is needed.

```bash
make deploy-base-sepolia   # BaseSepolia
make deploy-base-mainnet   # BaseMainnet
```

## Verify

```bash
make verify L2_TOKEN=<address> RPC_URL=https://sepolia.base.org
```

## Bridge (L1 -> Base)

Approve + deposit in one command. `AMOUNT` is in wei (1e18 = 1 ALIGN).

```bash
make bridge-l1-to-base-sepolia AMOUNT=1000000000000000000
make bridge-l1-to-base-mainnet AMOUNT=1000000000000000000
```

Tokens appear on Base after ~20 minutes.

## Bridge Addresses

Source: [Base Contracts](https://docs.base.org/chain/base-contracts)

| Network | L1StandardBridge | L2StandardBridge |
|---------|------------------|------------------|
| Sepolia | [`0xfd0Bf71F60660E2f608ed56e1659C450eB113120`](https://sepolia.etherscan.io/address/0xfd0Bf71F60660E2f608ed56e1659C450eB113120) | [`0x4200000000000000000000000000000000000010`](https://sepolia.basescan.org/address/0x4200000000000000000000000000000000000010) |
| Mainnet | [`0x3154Cf16ccdb4C6d922629664174b904d80F2C35`](https://etherscan.io/address/0x3154Cf16ccdb4C6d922629664174b904d80F2C35) | [`0x4200000000000000000000000000000000000010`](https://basescan.org/address/0x4200000000000000000000000000000000000010) |

## Deployed Addresses

| Network | L1 Token (Ethereum) | L2 Token (Base) |
|---------|---------------------|-----------------|
| Sepolia | `0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A` | `0x4AAcFbc2C31598a560b285dB20966E00B73F9F81` |
| Mainnet | TBD | TBD |

## References

- [OP Standard Bridge Standard Token Tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token)
- [Base Contracts](https://docs.base.org/chain/base-contracts)
