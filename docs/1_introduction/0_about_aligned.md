## What is Aligned?

Aligned is a decentralized network that verifies Zero-Knowledge/validity proofs and posts the results to Ethereum. It is designed to provide high throughput, cheap proof verification with low latency.

{% hint style="info" %}
If you are unfamiliar with ZK and why this is useful, see [Why ZK and Aligned?](./4_why_zk.md)
{% endhint %}

## Mission

Our mission is to accelerate Ethereum's roadmap and the adoption of verifiable computation by enabling fast and economical verification of ZK and validity proofs.

## What real value does Aligned bring to the table?

Blockchains are verifiable computers.
We live in a chaotic world where there will be a huge demand for computation that needs to be trusted.

Current blockchain models are inefficient: each node must re-execute each transaction or operation, making the weakest and slowest devices the bottleneck. Unlike in Web2, where adding more hardware can increase performance, in these systems, additional hardware primarily enhances reliability rather than speed. Alternative L1s to Ethereum make trade-offs, accepting bigger hardware or changing the consensus to be faster but with fewer security guarantees. In other words, they reduce costs and increase speed at a great expense of lower security guarantees.

On the other hand, Zero-knowledge/validity proofs (ZK) provide a new framework where we do not have to make such compromises. The basic premise is that a party can generate a very short proof of a computation, and the nodes can quickly verify that proof instead of re-executing the computation. The computation can be quite large, comprising many different transactions. This enables the delegation of execution off-chain with the same guarantees, leading to increased throughput and lower operational costs. This led to the rollup-centric roadmap of Ethereum, featuring multiple L2s, but at the expense of fragmented liquidity and complex user experience. Many of these problems could be solved by ZK bridges, but they remain expensive, and the go-to-market time is high.

Anyhow, nodes still have to verify those proofs, which can be quite expensive since this is done on-chain: current proof systems can cost between 10 and several hundred dollars (which is strongly dependent on network congestion).

In addition, ZK lets us build other verifiable applications where users do not have to trust the party performing the computation, with impact in areas such as artificial intelligence (AI), the Internet of Things (IoT), and fighting misinformation. This is not clear to most people since proving technologies were not mature enough and verification costs were high.

## What limits the development of more complex applications on top of blockchains?

The main limitation for building complex applications on top of blockchains has been that the computation can run only a few milliseconds on chain, and even then, this can be costly. You can't have millions of daily active users using Ethereum or any blockchain at the same time.

ZK solves this, but due to slow and complex-to-use proving and expensive verification, progress has been limited.
In the case of proving, before the development of general-purpose zero-knowledge virtual machines (zkVMs),
users had to express their computation as arithmetic circuits,
making the developer experience something like coding in assembler, error-prone, and complex.
Moreover, proof systems depended on trusted setups,
adding additional trust guarantees, the need to carry out special ceremonies to initialize parameters,
and delaying go-to-market times.
Besides,
having high verification costs (on the order of 10's to 100's of dollars per proof)
meant that only those projects with a huge capital could afford to build such applications.

## Why didn't anybody do it before?

To build Aligned, we needed several pieces in place.
First, we needed EigenLayer,
which allows building services and applications on top of Ethereum without competing for blockspace.
In our case, we could bootstrap the economic security for a decentralized network of verifiers,
avoiding the limitations of running proof verification on-chain.
Second, proving technology had to improve.
We currently have general-purpose zkVMs (which means we can code in Rust and other high-level languages and prove it),
proof systems are faster, and several improvements and developments are on the way.
This makes writing applications easier (providing a higher demand for proof verification),
and enables faster and simpler proof recursion (for proof compression).

## How much can Aligned reduce costs?

Aligned operates using two operation modes: fast and aggregation. The cost reduction depends on throughput, proof system, and mode used. For the least expensive systems, such as Groth16, this can amount to nearly 90%, while STARKs can be nearly 99%. Moreover, Aligned allows the verification of proofs that cannot be currently verified in Ethereum due to cost or size, such as Kimchi or Binius. The verification cost in Aligned is independent of the proof system, giving the choice of the proof system back to developers.

## How does Aligned compare to other solutions?

Aligned has created a new category in crypto: ZK verification layer. It is the only solution for now offering fast and aggregation modes while being linked to Ethereum via EigenLayer. The fast mode provides very high throughput (over two orders of magnitude more than Ethereum), low latency, and lower costs than the aggregation mode. The security is provided by restaking, and this mode is well suited for applications with tight limits on latency and costs. The aggregation mode works using the slower proof recursion strategy, also used in rollups to compress proofs. Another key feature is that Aligned is stateless, simplifying the process greatly.

Other solutions focus on building a separate L1 for proof verification (which sets them apart from Ethereum and requires bootstrapping the economic security, which can be lowered and subject to volatility, as opposed to that provided by restaking) or focus on the aggregation of proofs from a few proof systems. This last approach adds latency, higher operational costs, and constrains the developer's choice of the proof system.

The following table contains costs estimates for Aligned, assuming a batch size of 20 proofs.

| Proof system | Ethereum   | Aligned - Fast mode | Aligned - Aggregation |
| ------------ | ---------- | ------------------- | --------------------- |
| Groth16      | 250,000    | 40,000              | TBD                   |
| STARKs       | >1,000,000 | 40,000              | TBD                   |
| Kimchi-IPA   | ??????     | 40,000              | TBD                   |
| Binius.      | ??????     | 40,000              | TBD                   |

## Why are we building Aligned?

In recent months, we have witnessed the development and enhancement of general proving virtual machines such as Risc0, Valida, Jolt, and SP1. These innovations allow users to write ordinary code in languages like Rust or C and generate proofs demonstrating the integrity of computations. This evolution is poised to transform application development, provided we have verification networks with high throughput and low cost. This is the core vision of Aligned and the reason we are building it: the future belongs to provable applications.

Currently, proof verification in Ethereum is expensive and throughput is limited to around 10 proofs per second. The cost depends on the proof system used, and the availability of precompiles. Groth16 costs around 250,000 gas, STARKs, over 1,000,000, and other proof systems are too expensive to be used in Ethereum.

Proof technology has been evolving over the last decade, with new arguments, fields, commitments and other tools appearing every day. It is hard to try new ideas if verification costs are high, and there is a considerable go-to-market time, as a consequence of development time of new, gas-optimized smart contracts, or the inclusion of new precompiles to make them affordable.

Aligned provides an alternative to reduce costs and increase throughput significantly. This is achieved by two different modes: **fast mode** and **aggregation mode**.

The fast mode works with a subset of Ethereum’s validators via restaking. Validators (also known as Operators) receive proofs, verify them using the verification code written in Rust or any other higher-level language, and then sign messages with BLS signatures. If a two-thirds (2/3) majority agrees, the results are posted in Ethereum.

Since Aligned’s operators only need to run the verification code on bare metal, we have several advantages compared to running it on top of the EVM:

- The code can be optimized for speed, not gas consumption.
- We can leverage parallelization to increase throughput.
- Since the gas limit does not constrain us, we can verify proof systems that are too expensive for Ethereum, such as Kimchi or Binius.
- Adding new proof systems is straightforward.

Preliminary numbers show that Aligned can verify more than 1000 proofs per second, over two orders of magnitude than the EVM at nominal capacity. Using effective batching techniques, we can split the task creation and verification cost between thousands of proofs.

## Future additions

- Propagation of the results to different L2s
