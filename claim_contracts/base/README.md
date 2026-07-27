# ALIGN Token on Base L2

Deployment and bridging of the Aligned Token (ALIGN) on Base using the [OP Standard Bridge](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token).

## Setup

```bash
cp .env.example .env
```

`.env` holds the token/bridge addresses and, for the **testnet** targets, `DEPLOYER_PRIVATE_KEY` and
`USER_PRIVATE_KEY`. The **mainnet** targets ignore those and sign with `--interactive`, prompting for
the key instead — see [Mainnet is operated from safes](#mainnet-is-operated-from-safes).

## Deploy

The L2 token is an `OptimismMintableERC20` created by a factory — no custom contract is needed. The
factory differs per network:

| Network | `OptimismMintableERC20Factory` | |
| --- | --- | --- |
| Base Sepolia | `0x4200000000000000000000000000000000000012` | the OP predeploy |
| Base mainnet | `0xF10122D428B4bc8A9d050D06a2037259b4c4B83B` | Base-specific, avoids address conflicts with Optimism ([Base docs](https://docs.base.org/chain/base-contracts)) |

Both addresses exist on Base mainnet, so using the predeploy there would create the token from the
wrong factory. The Makefile already picks the right one per target:

```bash
make deploy-base-sepolia   # BaseSepolia
make deploy-base-mainnet   # BaseMainnet
```

> [!IMPORTANT]
> The L1 token address passed at creation becomes the L2 token's `REMOTE_TOKEN`, and it is
> **immutable**. Get it wrong and the only fix is creating a new L2 token — the bridge will never
> connect the two. Run the verification below before doing anything else with the address.

## Verify

Required after every deploy, not optional:

```bash
make verify L2_TOKEN=<address> RPC_URL=https://mainnet.base.org
```

Check that `REMOTE_TOKEN()` is exactly the intended L1 token and `BRIDGE()` is
`0x4200000000000000000000000000000000000010`.

`totalSupply()` will be **0**. That is expected: an `OptimismMintableERC20` is minted only by the
bridge, so the L2 supply is precisely what has been bridged. Until then any transfer of the token on
Base — including a claim — reverts.

## Mainnet is operated from safes

On mainnet the token holder is a Safe, and **the Ethereum safe and the Base safe may be different
addresses**. Two consequences:

- The `cast send` targets here are the **testnet / EOA** path. A Safe cannot run them; submit the
  equivalent call from the Safe instead, using the parameters given below.
- Never use the plain `bridge-l1-to-base-*` or `withdraw-base-to-l1-*` form from a Safe. They call
  `depositERC20` / `withdraw`, which credit **`msg.sender` on the destination chain** — for a Safe
  that is an address it does not control there. Use the `-to` variants, naming the destination
  chain's safe.

## Bridge (L1 -> Base)

`AMOUNT` is the token amount with 18 decimals (1000000000000000000 = 1 ALIGN). Tokens appear on Base
after ~2 minutes.

### From a safe (mainnet)

Two transactions, both from the **Ethereum distributor safe**:

1. On the **L1 ALIGN token** — let the bridge move the tokens:
   `approve(0x3154Cf16ccdb4C6d922629664174b904d80F2C35, <AMOUNT>)`
2. On the **L1StandardBridge** `0x3154Cf16ccdb4C6d922629664174b904d80F2C35`:
   `depositERC20To(<L1_TOKEN>, <L2_TOKEN>, <BASE distributor safe>, <AMOUNT>, 200000, 0x)`

The third argument is the destination on Base. It must be the **Base** safe, not the Ethereum one.

Confirm the deposit landed:

```bash
cast call <L2_TOKEN> 'balanceOf(address)(uint256)' <base-safe> --rpc-url https://mainnet.base.org
```

### From an EOA (testnet)

Approve + deposit in one command:

```bash
make bridge-l1-to-base-sepolia AMOUNT=1000000000000000000
make bridge-l1-to-base-mainnet AMOUNT=1000000000000000000
```

To bridge to a different L2 address, use the `TO` parameter:

```bash
make bridge-l1-to-base-sepolia-to AMOUNT=1000000000000000000 TO=0x...
make bridge-l1-to-base-mainnet-to AMOUNT=1000000000000000000 TO=0x...
```

## Withdraw (Base -> L1)

Withdrawals are a [multi-step process](https://docs.optimism.io/app-developers/tutorials/bridging/cross-dom-bridge-erc20#withdraw-tokens). No approval is needed. All three steps use the same `TX_HASH` — the **L2 initiation tx hash** from step 1.

> [!IMPORTANT]
> Only step 1 needs the safe. From the **Base distributor safe**, call on the L2StandardBridge
> `0x4200000000000000000000000000000000000010`:
> `withdrawTo(<L2_TOKEN>, <ETHEREUM distributor safe>, <AMOUNT>, 200000, 0x)`.
> Steps 2 and 3 credit the address recorded in step 1 no matter who submits them, so they run from
> any funded EOA — that is what `USER_PRIVATE_KEY` in `.env` is for. Save the step 1 tx hash before
> going further; without it you cannot prove or finalize, and the 7-day clock only starts once.
> Rehearse the whole flow on Sepolia first: a mainnet withdrawal is a week-long commitment.

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

The canonical ALIGN token per network. Test or throwaway deployments do not belong in this table.

| Network | L1 Token (Ethereum) | L2 Token (Base) |
|---------|---------------------|-----------------|
| Sepolia | `0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A` | `0x4AAcFbc2C31598a560b285dB20966E00B73F9F81` |
| Mainnet | `0x50614cc8e44f7814549c223aa31db9296e58057c` | `0x53f39e5C53EE40bbc3Da97C3B47BD2968d110a8D` |

These are also the `tokenProxy` values in `../script-config/config.*.json`, which the claim contract
deploys read — see [`../README.md`](../README.md).

## References

- [OP Standard Bridge Standard Token Tutorial](https://docs.optimism.io/app-developers/tutorials/bridging/standard-bridge-standard-token)
- [OP Bridge ERC-20 Tutorial (withdraw flow)](https://docs.optimism.io/app-developers/tutorials/bridging/cross-dom-bridge-erc20)
- [Base Contracts](https://docs.base.org/chain/base-contracts)
