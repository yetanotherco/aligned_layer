# ALIGN Token on Base L2

Deployment and bridging of the Aligned Token (ALIGN) on Base using the [OP Standard Bridge](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token).

## Setup

```bash
cp .env.example .env
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

Approve + deposit in one command. `AMOUNT` is the token amount with 18 decimals (1000000000000000000 = 1 ALIGN).

```bash
make bridge-l1-to-base-sepolia AMOUNT=1000000000000000000
make bridge-l1-to-base-mainnet AMOUNT=1000000000000000000
```

Tokens appear on Base after ~20 minutes.

To bridge to a different L2 address, use the `TO` parameter:

```bash
make bridge-l1-to-base-sepolia-to AMOUNT=1000000000000000000 TO=0x...
make bridge-l1-to-base-mainnet-to AMOUNT=1000000000000000000 TO=0x...
```

## Withdraw (Base -> L1)

Withdrawals are a [multi-step process](https://docs.optimism.io/app-developers/tutorials/bridging/cross-dom-bridge-erc20#withdraw-tokens). No approval is needed. All three steps use the same `TX_HASH` — the **L2 initiation tx hash** from step 1.

1. **Initiate** on L2 (burns tokens on Base):

   ```bash
   make withdraw-base-to-l1-sepolia AMOUNT=1000000000000000000
   make withdraw-base-to-l1-mainnet AMOUNT=1000000000000000000
   ```

   To withdraw to a different L1 address, use the `TO` parameter:

   ```bash
   make withdraw-base-to-l1-sepolia-to AMOUNT=1000000000000000000 TO=0x...
   make withdraw-base-to-l1-mainnet-to AMOUNT=1000000000000000000 TO=0x...
   ```

   Save the tx hash from this step — it's needed for prove and finalize.

2. **Prove** on L1 — wait ~1 hour for the L2 output to be proposed, then prove:

   ```bash
   make prove-withdrawal-sepolia TX_HASH=<L2 initiation tx hash>
   make prove-withdrawal-mainnet TX_HASH=<L2 initiation tx hash>
   ```

3. **Finalize** on L1 — wait 7 days challenge period (shorter on testnet), then finalize:

   ```bash
   make finalize-withdrawal-sepolia TX_HASH=<L2 initiation tx hash>
   make finalize-withdrawal-mainnet TX_HASH=<L2 initiation tx hash>
   ```

   > **Note:** Prove and finalize use `viem` + `viem/op-stack`. Run `npm install` first.

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
| Mainnet | `0x50614cc8e44f7814549c223aa31db9296e58057c` | `0x53f39e5C53EE40bbc3Da97C3B47BD2968d110a8D` |

## References

- [OP Standard Bridge Standard Token Tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token)
- [OP Bridge ERC-20 Tutorial (withdraw flow)](https://docs.optimism.io/app-developers/tutorials/bridging/cross-dom-bridge-erc20)
- [Base Contracts](https://docs.base.org/chain/base-contracts)
