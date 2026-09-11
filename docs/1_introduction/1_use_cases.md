# Use cases

Aligned’s ZK verification layer for Ethereum makes proof verification affordable and scalable through our recursive [Proof Aggregation Service](../2_proof_aggregation_layer/architecture/1_overview.md). Use Aligned when you need to verify many proofs, expensive proofs, or proofs that aren’t economical in the EVM, and you still want results settled to Ethereum with Ethereum's own security.

The ZK verification layer can be useful anytime ZK is used with Ethereum, but some clear use cases are described in this section.

## ZK-rollups and Ethereum scaling

Rollups produce ZK proofs of state transitions and need to verify (settle) those results on Ethereum. Verification can cost ZK-rollup operators millions per year, and infrequent verification to save on gas leads to longer exit windows and worse UX.

### **How Aligned helps:**

- Aggregates many rollup proofs into a single recursive proof verified on Ethereum, so the cost of on-chain verification is amortized across the whole batch.
- Keeps full L1 security and adds no cryptoeconomic trust assumptions: the aggregated proof is checked by an Ethereum smart contract like any other ZK proof.

{% hint style="success" %}
Our RaaS makes launching [based ZK-rollups](https://blog.alignedlayer.com/aligned-raas-based-rollups-to-build-the-future-of-ethereum/) (Ethrex stack) possible in one click and is integrated with our ZK verification layer to reduce costs.
{% endhint %}

## Fast, trust-minimized bridging & interoperability

Bridges and cross-chain systems verify source chain state/account proofs on the destination chain. We are also developing an intents and ZK-based fast interoperability protocol in-house to take full advantage of the ZK Verification Layer and to complement our RaaS platform.

### **How Aligned helps:**

- Makes verifying proofs economical at scale, so bridges can post consolidated checkpoints to L1 far more often than the raw gas cost would otherwise allow.

{% hint style="success" %}
The Mina ↔ Ethereum bridge uses Aligned to verify Mina’s Kimchi proofs on Ethereum ([blog post](https://blog.alignedlayer.com/mina-to-ethereum-bridge/)).
{% endhint %}

## zkTLS & web2-to-web3 data credentials

Generate proofs about data fetched over TLS (bank account balances, KYC, social graphs) and use them onchain without revealing the raw data.

### **How Aligned helps:**

- Turns high-volume zkTLS attestations into a practical UX by amortizing verification cost across many attestations.
- Keeps Ethereum as the root of trust, so a credential accepted onchain carries Ethereum's full security.

## Verifiable AI (zkML / LLM inference proofs)

To make AI verifiable, prove that model inference (or parts of a pipeline) ran correctly, or that an output meets policy constraints—then settle the result to Ethereum for auditability or onchain automation.

### **How Aligned helps:**

- zkVM proofs from systems like SP1 are supported today, so you can move from POCs to production-grade verification economics.
- High-volume inference checks (micro-payments, agent marketplaces, model-usage attestations) become affordable once verification cost is shared across a batch, and compliance events settle with L1-final proofs.

{% hint style="success" %}
See our [blog post](https://blog.alignedlayer.com/the-era-of-ai-needs-ethereum-and-zk/) that expands on why we believe ZK and Ethereum will play a major role in the future of AI.
{% endhint %}

## Identity & verifiable credentials

Issue/verify privacy-preserving credentials (DID/VC, age/eligibility proofs, reputation). Governments and enterprises can adopt ZK without forcing users to leak data.

### **How Aligned helps:**

- Makes credential verification cheap enough for mainstream UX while keeping Ethereum as the root of trust and avoiding the compromise of verifying proofs on a blockchain with lower security.
- Already powering real identity stacks (e.g. [Sovra’s Digital Trust Stack](https://blog.alignedlayer.com/aligned-sovra-partner-to-power-digital-trust-in-latin-america/)).

## ZK coprocessors & oracles

Off-chain processes fetch data or perform heavy computations and return a ZK proof that onchain logic can trust. Allows offchain computation to be secured by onchain trust, giving smart contracts greater expressivity.

### **How Aligned helps:**

- Lets you run richer computations and affordably verify them onchain.
- A natural way to add computational trust and reduce L1 gas bottlenecks for oracle-like systems, at the highest available security level.

## Onchain gaming & interactive apps

Games and interactive dapps that prove game validity, scores/results, or anti-cheat logic with ZK.

### **How Aligned helps:**

- Sharing verification cost across many proofs makes regular and frequent proving economical.
- We’ve highlighted early builders in our [hackathon spotlights](https://blog.alignedlayer.com/tag/hackathons/).

{% hint style="success" %}
_**Coming soon:**_ Our **ZK Arcade** will let users verify proofs of game results using Aligned.
{% endhint %}

## How it works

Developers submit their ZK proofs to the Proof Aggregation Service, where they are aggregated into a single recursive proof that is then verified on Ethereum. By aggregating many proofs into one, the cost of on-chain verification is amortized across the batch — you effectively pay a fraction of the full price, plus a small aggregation fee.

Because the final proof is verified by an Ethereum smart contract, this carries the full cryptographic security of Ethereum. The tradeoff is latency: proofs are settled within the aggregation window, which defaults to 24 hours.

See the [architecture overview](../2_proof_aggregation_layer/architecture/1_overview.md) and the [deep dive](../2_proof_aggregation_layer/architecture/2_deep_dive.md) for the details.

## Supported proof systems

Aligned currently aggregates SP1 compressed proofs, with more zkVMs on the roadmap. See [Supported Verifiers](../2_proof_aggregation_layer/architecture/4_supported_verifiers.md).

## Featured projects & posts

- [All the proof aggregation solutions will use RISC-V zkVMs](https://blog.alignedlayer.com/all-the-proof-aggregation-solutions-will-use-risc-v-zkvms/)
- [Aligned RaaS: Based ZK-rollups using Ethrex](https://blog.alignedlayer.com/why-is-aligned-using-ethrex-for-based-zk-rollups/)
- [Mina ↔ Ethereum bridge](https://blog.alignedlayer.com/mina-to-ethereum-bridge/)
- [Why based rollups?](https://blog.alignedlayer.com/aligned-raas-based-rollups-to-build-the-future-of-ethereum/)

## Future additions

- Use cases: Aligned RaaS
- Use cases: Meta-proving services
- Use cases: Aligned Wallets-as-a-Service
- Use cases: Aligned Interoperability protocol
