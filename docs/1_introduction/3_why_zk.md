# Why ZK and how can Aligned help you?

The following is an introduction to zero-knowledge/validity proofs to understand their utility and impact and why they may help you solve problems you can encounter when building your application. Before jumping on the explanation, we give some definitions:
 
- Validity/integrity proofs: These are cryptographic proofs that allow you to check that a computation was carried out correctly, without having to re-run it entirely.
- Zero-knowledge proofs: These are cryptographic that allow you to prove the validity of a statement, without revealing sensitive data.

The two terms are sometimes grouped under ZK, though depending on the use case, you may need it to be zero-knowledge or not. Validity proofs are widely used by ZK-rollups (though not rigorously zero-knowledge), while Schnorr signatures are an example of the second (basically, they let you show to others that you have a secret key, without leaking it).

In blockchains, we need to coordinate between different parties that do not trust each other. How can we agree on whether something has happened? The logical construction involves independently re-executing transactions and reaching consensus. However, the number of transactions we can process is limited by the weakest devices in the network, acting as bottlenecks. Moreover, adding more hardware does not make the system faster (as in web2), only more robust. ZK allows the situation to scale, allowing the system to process more transactions with the same guarantees. ZK proofs allow you to verify a computation much faster than re-execution: we can use more powerful machines to run the transactions and generate the cryptographic proof, and the rest of the network verifies the proof. If the proof is valid, it is the same as if all the nodes had re-executed the transactions, but with less computational effort. More concretely, a ZK-rollup can generate a proof that it processed 10,000 transactions correctly, and submit to Ethereum the proof with the state diff (or other information necessary to update the state), and Ethereum can check very quickly that all those transactions were correct! ZK is also useful whenever you need to show the integrity of some computation, for example, that an image you published in the newspaper is the result of enlarging an image from a real camera. This is where Aligned's stack comes in.

With Aligned's stack, you can generate the proof and then use Ethereum as a settlement layer. To do so, Aligned offers a variety of products to reduce your costs and increase throughput. Once you decide to use ZK for your product and Ethereum, Aligned fits perfectly.

- **Proof Aggregation Service:** Performs recursive proof aggregation of several ZK proofs. The final proof is sent to Ethereum, and if verified, it implies the validity of your proof. Because that final proof is checked by an Ethereum smart contract, it achieves the full security of Ethereum, while the cost of the on-chain verification is amortized across every proof in the batch.
- **Meta Proving Services**: Offers an easy interface for accessing centralized and decentralized proving from external providers.
- **Rollup-as-a-service (RaaS) platform:** Simplifies ZK-rollup deployment, allowing clients to launch L2 chains with just one click, without needing deep blockchain expertise.
- **Interoperability protocol:** An intent-based bridge that will be integrated with our RaaS stack. It will leverage based sequencing and enable developers to create native trust-minimized solutions for users and financial institutions to efficiently move liquidity across chains.
- **Wallet-as-a-Service infrastructure:** Enable developers to easily generate embedded wallets for users, supporting rollups and mobile integrations. It will leverage the latest account abstraction technology, offering seamless wallet services backed by our robust tech stack.

How can you write ZK applications and generate proofs? There are many different technologies and libraries to build ZK applications. Until recently, it was fairly complicated, since you needed some background of math and write circuits (you can think of it as coding in assembly). Luckily, there are several ZK virtual machines (zkVM) that allow you to generate proofs of code written in a higher level language, such as Rust.
You write your code and run it on top of the zkVM, and you get a proof of its correct execution. See [Generating proofs for Aligned](../2_proof_aggregation_layer/guides/2_generating_proofs.md) for how to produce a proof that Aligned can aggregate.

## Projects or ideas using ZK

The list below contains examples or projects using ZK. It is meant to illustrate some use cases, but it is not meant to be exhaustive.

- Rollups
- On-chain gaming
- ZK Machine Learning (zkML)
- ZK-TLS
- Bridges
- Oracles
- Data processors
- Voting
- ZK-Email
- Identity protocols
- IoT
