## What is Aligned?

Aligned is a decentralized network that verifies zero-knowledge/validity proofs and posts the results to Ethereum. It is designed to provide high throughput, cheap proof verification with low latency.

## Mission

Our mission is to accelerate Ethereum's roadmap and the adoption of verifiable computation by enabling fast and economical verification of ZK and validity proofs. 

## Why are we building Aligned?

In recent months, we have witnessed the development and enhancement of general proving virtual machines such as Risc0, Valida, Jolt, and SP1. These innovations allow users to write ordinary code in languages like Rust or C and generate proofs demonstrating the integrity of computations. This evolution is poised to transform application development, provided we have verification networks with high throughput and low cost. This is the core vision of Aligned and the reason we are building it: the future belongs to provable applications.

Currently, proof verification in Ethereum is expensive and throughput is limited to around 10 proofs per second. The cost depends on the proof system used, and the availability of precompiles. Groth16 costs around 250,000 gas, STARKs, over 1,000,000, and other proof systems are too expensive to be used in Ethereum. 

Proof technology has been evolving over the last decade, with new arguments, fields, commitments and other tools appearing every day. It is hard to try new ideas if verification costs are high and there is a considerable go-to-market time, as a consequence of development time of new, gas-optimized smart contracts, or the inclusion of new precompiles to make them affordable.

Aligned provides an alternative to reduce costs and increase throughput significantly. This is achieved by two different modes: **fast mode** and **aggregation mode**. 

The fast mode works with a subset of Ethereum’s validators via restaking. Validators (also known as Operators) receive proofs, verify them using the verification code written in Rust or any other higher-level language, and then sign messages with BLS signatures. If a two-thirds (2/3) majority agrees, the results are posted in Ethereum. 

Since Aligned’s operators only need to run the verification code on bare metal, we have several advantages compared to running it on top of the EVM:

- The code can be optimized for speed, not gas consumption.
- We can leverage parallelization to increase throughput.
- Since the gas limit does not constrain us, we can verify proof systems that are too expensive for Ethereum, such as Kimchi or Binius.
- Adding new proof systems is straightforward.

Preliminary numbers show that Aligned can verify more than 1000 proofs per second, over two orders of magnitude faster than the EVM at nominal capacity. Using effective batching techniques, we can split the task creation and verification cost between thousands of proofs.
