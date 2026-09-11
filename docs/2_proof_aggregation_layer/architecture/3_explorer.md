# Explorer

{% embed url="https://explorer.alignedlayer.com" %}

The Explorer tracks the aggregated proofs that the [Proof Aggregator](2_deep_dive.md) settles on Ethereum, so you can see which aggregated proof your submission ended up in and follow it back to the settling transaction.

Deployments:

- [Mainnet](https://explorer.alignedlayer.com)
- [Hoodi](https://hoodi.explorer.alignedlayer.com)
- [Sepolia](https://sepolia.explorer.alignedlayer.com)

## Aggregated proofs list

The `/aggregated_proofs` page lists aggregated proofs, most recent first. Each row shows:

- **Merkle root** — the root committed by the aggregated proof, built from the [proof commitments](2_deep_dive.md#proof-commitment) it covers. Links to the detail view.
- **Age** — how long ago the aggregated proof was settled.
- **Block Number** — the Ethereum block the aggregated proof was included in.
- **Blob versioned hash** — the blob carrying the proof commitments, linked to Blobscan.
- **Number of proofs** — how many user proofs were aggregated together.
- **Aggregator** — the aggregation program used, `SP1` or `RISC0`.

## Aggregated proof details

Clicking a Merkle root opens `/aggregated_proofs/:id`, which shows:

- The **Merkle root**, with a copy-to-clipboard button
- The **Aggregator** used
- The **Number of proofs included**
- **Proofs included** — an expandable list of the individual proof commitments in this aggregated proof, each with its own copy button. This is how you confirm your proof made it into a given aggregation.
- **Block Number** and **Transaction Hash** of the settling transaction, linked to Etherscan
- The **Blob versioned hash**, linked to Blobscan

The blob is the same one described in [Data Availability](2_deep_dive.md#data-availability): it keeps the proof commitments publicly available for 18 days, which is what lets you reconstruct the Merkle proof needed to check your proof's inclusion on-chain.

{% hint style="info" %}
The Explorer is for inspection. To verify programmatically that your proof was included, use [`agg_mode_cli verify-on-chain`](../guides/3_cli.md#verify-on-chain) or the SDK's [`check_proof_verification`](../guides/4_sdk.md#check_proof_verification).
{% endhint %}
