# Overview

## Aligned's Proof Aggregation Service in a nutshell

Aligned's Proof Aggregation Service introduces a scalable solution: compressing multiple proofs into one using recursion, drastically reducing verification costs while maintaining Ethereum-level security. It is designed to give developers flexible, cost-efficient infrastructure for proving systems.

Developers submit their ZK proofs directly to the Aggregation Service, where they are aggregated in a single recursive proof that attests to the validity of all the proofs. This final proof is then submitted and verified on Ethereum. By aggregating many proofs into one, the cost of on-chain verification is amortized across the batch—developers effectively pay a fraction of the full price, plus a small aggregation fee.

![Figure 1: Proof Aggregation Service](../../images/aligned_proof_aggregation_service.png)

The system is powered by recursive proving. In simple terms, the aggregation process proves that the verification of multiple proofs was correctly executed. This meta-proof is cryptographically valid and can be verified on Ethereum just like any standard ZK proof.
