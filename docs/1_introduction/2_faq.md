# FAQ

### What is Aligned's objective?

Aligned is creating the foundation for a trustless, verifiable internet. Our vertically integrated stack empowers developers to build applications across finance, AI, and other sectors with one-click solutions for wallets, rollups, and zero-knowledge services on Ethereum. We’re focused on enabling provable execution in a world where institutional trust is increasingly fragile.

By providing the tools for trust to be integrated into every layer of application infrastructure, we’re enabling developers to create verifiable systems that can be trusted across a wide range of use cases. Aligned is here to enable a future where trust is not assumed, but proven by design.

### What are the security guarantees and trust assumptions of Aligned's Proof Aggregation Service?

The Proof Aggregation Service produces a single recursive proof attesting that every proof it aggregated was verified correctly. That proof is verified by an Ethereum smart contract, so a valid on-chain verification implies the validity of each individual proof. There is no separate validator set to trust and no cryptoeconomic assumption: the security is Ethereum's own.

The aggregator can censor a proof by not including it in an aggregation, but it cannot cause an invalid proof to be accepted.

### Why build Aligned on top of Ethereum?

Ethereum is the most decentralized and most significant source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

For further reading on this subject, you can refer to [this Aligned blog post](https://blog.alignedlayer.com/why-ethereum/), which explains why we chose Ethereum.

### How much does it cost to verify proofs using Aligned?

The cost of verifying the aggregated proof on Ethereum is amortized across every proof it contains. The more proofs are aggregated together, the smaller each one's share of that cost, plus a small aggregation fee and the cost of reading the result on-chain.

### How long does it take?

Proofs are aggregated and settled to Ethereum within the aggregation window, which defaults to 24 hours. This is the tradeoff for getting Ethereum's full cryptographic security on a per-proof cost that would otherwise be impractical.

### Why is Aligned building its stack?

Aligned is building its stack to provide the infrastructure for a trustless, verifiable internet. Its vertically integrated stack enables developers to create applications across industries like finance and AI, with one-click solutions for rollups, wallets, and several zero-knowledge (ZK) services. Innovations in proving virtual machines, like Risc0, Jolt, SP1, and Valida, allow users to generate proof of computation integrity using languages like Rust or C. However, Ethereum’s proof verification is costly and slow, limiting innovation. Aligned addresses this with its Proof Aggregation Service, reducing verification costs and increasing throughput. In addition, Aligned offers a Wallet-as-a-Service infrastructure for simplifying wallet generation, and a Rollup-as-a-Service (RaaS) platform for easy ZK-rollup deployment. Its Interoperability Protocol also supports trust-minimized cross-chain liquidity movement. Together, these innovations aim to build a scalable, secure, and trustless internet.

### Is Aligned an Ethereum L2?

No. Aligned is a vertically integrated stack for building applications on a verifiable internet. However, our stack offers one-click solutions for rollups, as well as wallets, and several ZK services.

### What is proof recursion?

Zero-knowledge proofs let you generate proofs that show the correct execution of programs. If a program is the verification of a proof, then we will be getting a proof that we verified the proof and the result was valid. The validity of the second proof implies the validity of the original proof. This is the idea behind proof recursion, and it can be used with two main goals:

1. Convert one proof type to another (for example, a STARK proof to a Plonk proof) either to reduce the proof size, have efficient recursion, or because the proof system cannot be verified where we want.
2. Proof aggregation: if we have to verify $N$ proofs on-chain, we can generate a single proof that we verified the $N$ proofs off-chain and just check the single proof on Ethereum.

Proof recursion is the primary tool used by Aligned’s Proof Aggregation Service.

### Is Aligned an aggregation layer?

Aligned provides proof aggregation as part of its Proof Aggregation Service, a feature shared with all aggregation layers. Aligned's approach is stateless and settles directly to Ethereum, so proofs verified through it inherit Ethereum's security rather than that of a separate chain.

### What proof systems do you support?

Aligned’s stack is designed to support any proof system. The Proof Aggregation Service currently accepts SP1 compressed proofs, with more zkVMs on the roadmap. See [Supported Verifiers](../2_proof_aggregation_layer/architecture/4_supported_verifiers.md).

### How does Aligned's Proof Aggregation Service work?

The flow is as follows:

1. The user deposits into the payment service contract to fund their proof submission quota.
2. The user submits a proof to the Gateway using the CLI or SDK.
3. The Gateway validates the submission against the user's quota and stores the proof.
4. The Proof Aggregator picks up pending proofs and recursively aggregates them into a single proof, committing to a Merkle root of all the proof commitments.
5. The aggregated proof is submitted to Ethereum and verified by the `AlignedProofAggregationService` contract, along with a blob containing the individual proof commitments.
6. The user checks their proof's inclusion by providing a Merkle proof against the stored root.

See the [deep dive](../2_proof_aggregation_layer/architecture/2_deep_dive.md) for the full details.

### How can I verify proofs using Aligned's stack?

You can verify proofs with our stack using our [CLI](../2_proof_aggregation_layer/guides/3_cli.md) or [Rust SDK](../2_proof_aggregation_layer/guides/4_sdk.md).

### Can you provide an estimate of Aligned’s savings?

Proof verification directly on Ethereum (not including the cost of accessing/reading) costs:

- Groth 16 proofs: >250,000 gas
- Plonk/KZG proofs: >300,000 gas
- STARKs: >1,000,000 gas
- Binius/Jolt: too expensive to run!

With Aligned's Proof Aggregation Service, a single aggregated proof (around 300,000 gas to verify) covers every proof in the batch, so the per-proof cost falls as the batch grows.

### I want to verify just one proof. Can I use your products and services for cheap and fast verification?

Yes. Your proof is aggregated together with everyone else's, so you still benefit from sharing the on-chain verification cost.

### Is Aligned open-source?

Yes! We are 100% open-source from day one.

### What’s the role of Aligned in Ethereum?

Aligned's role is to provide a full stack of vertically integrated infrastructure that simplifies the process of launching, operating, and scaling on the Ethereum network, enabling teams to build applications across finance, AI, and other sectors with one-click solutions for wallets, rollups, and zero-knowledge services.

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and our goal is to help increase its capabilities.

### What are the use cases for Aligned's stack?

We believe that there are many things that will be built using Aligned's stack that we have not even imagined yet. For some possible use cases please see [this page](1_use_cases.md).

### What about TEEs?

Aligned's stack will also verify remote attestations made by Trusted Execution Environments (TEEs). We believe TEEs provide an alternative to ZK in some applications where ZK is too computationally intensive or as an additional mechanism to provide integrity (_e.g._ in multiproofs).
