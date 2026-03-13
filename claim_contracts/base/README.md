# ALIGN Token on Base L2

Deployment of the Aligned Token (ALIGN) on Base L2 using the OptimismMintableERC20Factory.

Based on the [OP Standard Bridge Standard Token tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token).

## Overview

Base is an OP Stack chain. The ALIGN token on Base is created via the
`OptimismMintableERC20Factory` predeploy at `0x4200000000000000000000000000000000000012`.
This produces a standard `OptimismMintableERC20` token that is automatically compatible
with the OP Standard Bridge for L1 <-> L2 token transfers.

No custom Solidity contract is deployed — the factory handles everything.

## Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation) (`cast` CLI)
- `jq` for parsing transaction receipts
- An account with ETH on the target Base network (for gas)
- The L1 ALIGN token proxy address (deployed on Ethereum Sepolia or Mainnet)

## Setup

1. Copy `.env.example` to `.env` and fill in the values:

   ```bash
   cp .env.example .env
   ```

2. Set the L1 token addresses. The Makefile defaults are:
   - `L1_TOKEN_SEPOLIA=0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A`
   - `L1_TOKEN_MAINNET=` (set when ready)

   You can override them via `.env` or by passing them to `make`.

## Deployment

The deploy targets call `createOptimismMintableERC20` on the factory and extract the
deployed L2 token address from the `OptimismMintableERC20Created` event logs, following
the [OP tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token#create-an-l2-erc-20-token).

### BaseSepolia (Testnet)

```bash
source .env
make deploy-base-sepolia
```

### BaseMainnet (Production)

```bash
source .env
make deploy-base-mainnet
```

The output will print the deployed L2 token address.

## Verification

After deployment, verify the token was created correctly:

```bash
make verify-base-sepolia L2_TOKEN=<deployed_l2_token_address>
```

Expected output:

| Check | Expected |
|-------|----------|
| `name()` | `"Aligned Token"` |
| `symbol()` | `"ALIGN"` |
| `decimals()` | `18` |
| `REMOTE_TOKEN()` | L1 token proxy address |
| `BRIDGE()` | `0x4200000000000000000000000000000000000010` |
| `totalSupply()` | `0` (before any bridging) |

## Bridging Tokens (L1 -> Base)

After the L2 token is deployed, tokens can be bridged from L1 to Base using the
OP Standard Bridge. The Makefile handles the two-step process (approve + deposit).

### Bridge Addresses

Source: [Base Contracts](https://docs.base.org/chain/base-contracts)

| Network              | L1StandardBridge | L2StandardBridge |
|----------------------|------------------|------------------|
| Sepolia / BaseSepolia | [`0xfd0Bf71F60660E2f608ed56e1659C450eB113120`](https://sepolia.etherscan.io/address/0xfd0Bf71F60660E2f608ed56e1659C450eB113120) | [`0x4200000000000000000000000000000000000010`](https://sepolia.basescan.org/address/0x4200000000000000000000000000000000000010) |
| Mainnet / Base        | [`0x3154Cf16ccdb4C6d922629664174b904d80F2C35`](https://etherscan.io/address/0x3154Cf16ccdb4C6d922629664174b904d80F2C35) | [`0x4200000000000000000000000000000000000010`](https://basescan.org/address/0x4200000000000000000000000000000000000010) |

### BaseSepolia

```bash
make bridge-l1-to-base-sepolia USER_PRIVATE_KEY=0x... AMOUNT=1000000000000000000
```

### BaseMainnet

```bash
make bridge-l1-to-base-mainnet USER_PRIVATE_KEY=0x... AMOUNT=1000000000000000000
```

`USER_PRIVATE_KEY` is the private key of the account holding ALIGN tokens on L1.
`AMOUNT` is in wei (the example above bridges 1 ALIGN token = 1e18 wei).
Both can also be set in the `.env` file.

The command will:
1. Approve the L1StandardBridge to spend `AMOUNT` ALIGN tokens on L1
2. Call `depositERC20` on the L1StandardBridge to initiate the bridge

Tokens will appear on Base after the L1 transaction is included and the
message is relayed (~20 minutes).

### L2 -> L1 (Withdrawal)

Users call `withdraw` on the L2StandardBridge. After the challenge period
(7 days on mainnet), prove and finalize the withdrawal on L1 to unlock tokens.

## Deployed Addresses

| Network              | L1 Token (Ethereum) | L2 Token (Base) |
|----------------------|---------------------|-----------------|
| Sepolia / BaseSepolia | `0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A` | `0x4AAcFbc2C31598a560b285dB20966E00B73F9F81` |
| Mainnet / Base        | TBD                 | TBD             |

## References

- [OP Standard Bridge Standard Token Tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token)
- [Superchain Token List](https://github.com/ethereum-optimism/ethereum-optimism.github.io)
