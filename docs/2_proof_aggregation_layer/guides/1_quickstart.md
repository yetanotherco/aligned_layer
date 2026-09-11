# Quickstart

In this tutorial you will submit an SP1 proof to Aligned's Proof Aggregation Service and check that it was verified on Ethereum.

{% hint style="warning" %}
This tutorial uses the Hoodi testnet. The same steps work on Mainnet by passing `--network mainnet` and a Mainnet RPC URL.
{% endhint %}

## Quickstart

We will use a previously generated SP1 proof that ships with the repository, submit it for aggregation, and then read the result from Ethereum.

### 1. Install the Aggregation Mode CLI

```bash
git clone https://github.com/yetanotherco/aligned_layer.git
cd aligned_layer
make agg_mode_install_cli
```

Check the installation:

```bash
agg_mode_cli --help
```

### 2. Create a keystore

If you do not already have one, create a local keystore with `cast` and import it:

```bash
cast wallet new-mnemonic
cast wallet import --interactive <path_to_keystore.json>
```

Fund the account with Hoodi ETH from [this faucet](https://cloud.google.com/application/web3/faucet/ethereum/hoodi). You need funds both for gas and for your proof submission quota.

### 3. Fund your submission quota

Deposit into the Aggregation Mode payment service:

```bash
agg_mode_cli deposit \
  --keystore-path <KEYSTORE_PATH> \
  --network hoodi \
  --rpc-url https://ethereum-hoodi-rpc.publicnode.com
```

Each call deposits a fixed 0.0035 ether.

### 4. Submit the proof

From the root of the repository:

```bash
agg_mode_cli submit sp1 \
  --proof scripts/test_files/sp1/sp1_fibonacci_5_0_0.proof \
  --vk scripts/test_files/sp1/sp1_fibonacci_5_0_0_vk.bin \
  --keystore-path <KEYSTORE_PATH> \
  --network hoodi
```

On success the CLI prints a task ID:

```
INFO Submitting SP1 proof to Hoodi
INFO Proof submitted successfully. Task ID: 3f9a1c4e-...
```

### 5. Wait for aggregation

Your proof is batched with others and aggregated into a single recursive proof, which is then verified on Ethereum. This happens within the aggregation window — **24 hours by default** — so this step is not instant.

You can follow aggregated proofs as they land in the [Hoodi explorer](https://hoodi.explorer.alignedlayer.com).

### 6. Check that your proof was verified

```bash
agg_mode_cli verify-on-chain \
  --network hoodi \
  --rpc-url https://ethereum-hoodi-rpc.publicnode.com \
  --beacon-url https://ethereum-hoodi-beacon-api.publicnode.com \
  --proving-system SP1 \
  --vk-hash scripts/test_files/sp1/sp1_fibonacci_5_0_0.vk \
  --public-inputs scripts/test_files/sp1/sp1_fibonacci_5_0_0.pub
```

This reads the result of the aggregated proof verification from Ethereum and checks your proof's inclusion in it.

{% hint style="info" %}
If your proof is not found, pass an earlier `--from-block` to search further back. The default only looks at roughly the last 25 hours, and the blob carrying the proof commitments expires after 18 days.
{% endhint %}

## Next steps

- [Generating proofs for Aligned](2_generating_proofs.md) — produce these files from your own SP1 program
- [Integration example](5_integration_example.md) — a toy L2 built on the Proof Aggregation Layer
- [Aggregation Mode CLI](3_cli.md) and [SDK](4_sdk.md) references
- [Architecture overview](../architecture/1_overview.md) — how aggregation works under the hood
