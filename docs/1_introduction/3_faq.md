# FAQ

### What is Aligned's objective?

Aligned is creating the foundation for a trustless, verifiable internet. Our vertically integrated stack empowers developers to build applications across finance, AI, and other sectors with one-click solutions for wallets, rollups, and zero-knowledge services on Ethereum. We’re focused on enabling provable execution in a world where institutional trust is increasingly fragile.

By providing the tools for trust to be integrated into every layer of application infrastructure, we’re enabling developers to create verifiable systems that can be trusted across a wide range of use cases. Aligned is here to enable a future where trust is not assumed, but proven by design.

### What are the security guarantees and trust assumptions of Aligned's Proof Verification Layer?

Aligned's Proof Verification Layer verifies proofs by having the operators re-execute the verification code for each proof and, if all of the proofs are valid, each of them signs a message containing a commitment to the proof and public input or the root of the batch. The aggregator is responsible for receiving the signatures, checking the quorum, performing the aggregation and sending them to Ethereum.

- 67% of the operators behaving dishonestly to be able to submit false proofs.
- 33% of the operators colluding to censor a batch of proofs or task. However, in the case of a batch, the operators can only censor the whole batch, but not a particular proof included in it.
- The aggregator can censor batches or proofs by not sending the aggregated signature.

### What is the batcher?

We have a service called the Batcher that batches enough proofs to send to the AVS in EigenLayer to reduce on-chain verification costs. Users can submit their proofs directly to Aligned’s Proof Verification Layer without using the Batcher. The Batcher is fully optional and serves as an optimization to further reduce on-chain verification costs.

### What are the security guarantees added by the batcher?

A batcher can censor proofs. The user can run their own batcher to avoid censorship or can send a task to verify proofs in our Proof Verification Layer via Ethereum without using the batcher. The batcher cannot transfer user's funds to other accounts, only spend them to create verification tasks and pay to the aggregator. We recommend depositing only enough funds for a few months of operations.

### How do I send proofs without a batcher?

You can see the steps to do it in [this guide](../3_guides/8_submitting_batch_without_batcher.md).

### How do I run my own batcher?

To-do

### Why build Aligned on top of Ethereum?
    
Ethereum is the most decentralized and most significant source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

For further reading on this subject, you can refer to [this Aligned blog post](https://blog.alignedlayer.com/why-ethereum/), which explains why we chose Ethereum.

### What is the throughput of Aligned’s Proof Verification Layer?
    
Aligned’s Proof Verification Layer runs the verifier’s code natively. The verification time depends on the proof system, program run, and public input. Generally, most verifiers can be run in the order of ms on consumer-end hardware. We can optimize the code for speed and leverage parallelization by running it natively. Current testnet can verify more than 2500 proofs per second.

### How does the throughput of Aligned’s Proof Verification Layer compare with Ethereum?

Ethereum runs on top of the EVM. Each block is limited to 45,000,000 gas. Since the most efficient proof systems take at least 250,000 gas, Ethereum can verify 180 proofs per block. Aligned's Proof Verification Layer runs the code natively and leverages parallelization, reaching 30,000 proofs in the same period.

### Why is Aligned buiding its stack?

Aligned is building its stack to provide the infrastructure for a trustless, verifiable internet. Its vertically integrated stack enables developers to create applications across industries like finance and AI, with one-click solutions for rollups, wallets, and several zero-knowledge (ZK) services. Innovations in proving virtual machines, like Risc0, Jolt, SP1, and Valida, allow users to generate proof of computation integrity using languages like Rust or C. However, Ethereum’s proof verification is costly and slow, limiting innovation. Aligned addresses this with its Proof Verification Layer and Proof Aggregation Service, reducing verification costs and increasing throughput. In addition, Aligned offers a Wallet-as-a-Service infrastructure for simplifying wallet generation, and a Rollup-as-a-Service (RaaS) platform for easy ZK-rollup deployment. Its Interoperability Protocol also supports trust-minimized cross-chain liquidity movement. Together, these innovations aim to build a scalable, secure, and trustless internet.

### Is Aligned an Ethereum L2?
    
No. Aligned is a vertically integrated stack for building applications on a verifiable internet. However, our stack offers one-click solutions for rollups, as well as wallets, and several ZK services.
    
### How much it cost to verify proofs using Aligned's Proof Verification Layer?
    
The cost ($C$) of proof verification using Aligned's Proof Verification Layer is the cost of task creation ($C_{task}$) plus the cost of verifying an aggregated BLS signature ($C_{verification}$), divided by the number of proofs ($N$) in the batch, plus the the cost of reading the results on-chain ($C_{read}$).
    
$$
  C =\frac{C_{task} + C_{verification}}{N} + C_{read}
$$

The cost of task creation and signature verification is amortized across $N$ proofs per batch, meaning that our verification layer becomes cheaper to use as more proofs are verified at the same time.

### What are BLS signatures?
    
[Boneh-Lynn-Shacham (BLS)](https://en.wikipedia.org/wiki/BLS_digital_signature) signatures are a cryptographic signature that allows a user to verify that a signer is authentic. It relies on elliptic curve pairings and is used by Ethereum due to its aggregation properties.

### Why do you have a Proof Verification Layer and a Proof Aggregation Service?

The Proof Verification Layer is designed to offer very cheap verification costs and low latency. It uses crypto-economic guarantees provided by restaking; costs can be as low as 2100 gas. The Proof Aggregation Service uses recursive proof aggregation, achieving the complete security of Ethereum, but with slightly higher fees and latency. We verify an aggregated BLS signature (around 113,000 gas) in our Proof Verification Layer. We verify an aggregated proof (around 300,000 gas) in our Proof Aggregation Service. Together, these two services form our ZK verification layer which offers developers different options for reducing proof verification costs on Ethereum, depending on their scale and security requirements.

### What is proof recursion?
    
Zero-knowledge proofs let you generate proofs that show the correct execution of programs. If a program is the verification of a proof, then we will be getting a proof that we verified the proof and the result was valid. The validity of the second proof implies the validity of the original proof. This is the idea behind proof recursion, and it can be used with two main goals:
    
1. Convert one proof type to another (for example, a STARK proof to a Plonk proof) either to reduce the proof size, have efficient recursion, or because the proof system cannot be verified where we want.
2. Proof aggregation: if we have to verify $N$ proofs on-chain, we can generate a single proof that we verified the $N$ proofs off-chain and just check the single proof on Ethereum.
    
Proof recursion is the primary tool used by Aligned’s aggregation mode.

### What is restaking?
    
EigenLayer introduced the concept of Restaking. It allows Ethereum’s validators to impose additional slashing conditions on their staked ETH to participate in Actively Validated Services (AVS) and earn additional rewards. This creates a marketplace where applications can rent Ethereum's trust without competing for blockspace. Aligned's fast mode is an AVS.

### Is Aligned an aggregation layer?
    
Aligned provides proof aggregation as part of its Proof Aggregation Service, a feature shared with all aggregation layers. However, Aligned offers a unique Proof Verification Layer designed to provide cheap and low-latency proof verification, secured by restaked ETH.

### What proof systems do you support?
    
Aligned’s stack is designed to support any proof system. [Currently supported ones](../2_architecture/0_supported_verifiers.md) are Groth16 and Plonk (gnark), SP1, Risc0, and Circom.

### How does Aligned's Proof Verification Layer work?
    
The flow is as follows:
    
1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum)
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments to all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verify all the proofs. 
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.

### How can I verify proofs using Aligned's stack?
    
You can verify proofs with our stack using our CLI or Rust SDK.

### Can you provide an estimate of Aligned’s savings?
    
Proof verification directly on Ethereum (not including the cost of accessing/reading) costs: 
    
- Groth 16 proofs: >250,000 gas
- Plonk/KZG proofs: >300,000 gas
- STARKs: >1,000,000 gas
- Binius/Jolt: too expensive to run!
    
With Aligned's Proof Verification Layer:
    
- Just one proof (any!): 350,000 gas
- Batching 1024 proofs: 350 gas + reading cost
    
This means verifying proofs using Aligned's Verification Layer can be 99% cheaper than directly on Ethereum.

### I want to verify just one proof. Can I use your products and services for cheap and fast verification?
    
Yes! And all proofs cost the same to verify using our stack.

### Is Aligned open-source?
    
Yes! We are 100% open-source from day one.

### What’s the role of Aligned in Ethereum?

Aligned's role is to provide a full stack of vertically integrated infrastructure that simplifies the process of launching, operating, and scaling on the Ethereum network, enabling teams to build applications across finance, AI, and other sectors with one-click solutions for wallets, rollups, and zero-knowledge services.

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and our goal is to help increase its capabilities.

### What are the use cases for Aligned's stack?
    
We believe that there are many things that will be built using Aligned's stack that we have not even imagined yet. For some possible use cases please see [this page](../1_introduction/2_use_cases.md).

### Why EigenLayer?
    
We believe Ethereum is the best settlement layer, and ZK will play a key role in helping it become the settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs: so how do we build such a decentralized network that will help Ethereum? 

Creating a new L1 doesn’t benefit Ethereum because it will add new trust assumptions to the Ethereum protocols relying on it. EigenLayer enables Aligned to have the following properties:

1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in ETH
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### What about TEEs?

Aligned's stack will also verify remote attestations made by Trusted Execution Environments (TEEs). We believe TEEs provide an alternative to ZK in some applications where ZK is too computationally intensive or as an additional mechanism to provide integrity (_e.g._ in multiproofs).
