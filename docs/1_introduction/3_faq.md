# FAQ

### What is Aligned's objective?
    
Aligned’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain that zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t exactly know what that future will look like, but we are certain it will be built on Ethereum. 

The question we want to share is: If we are sure that zero-knowledge proofs are the future of Ethereum, but we don't know which of the many possible zero-knowledge futures will win, **then how do we build infrastructure for Ethereum to make it compatible with any future zero-knowledge proving system?**

### What are the security guarantees and trust assumptions of Aligned?

Aligned verifies proofs by having the operators re-execute the verification code for each proof and, if all of the proofs are valid, each of them signs a message containing a commitment to the proof and public input or the root of the batch. The aggregator is responsible for receiving the signatures, checking the quorum, performing the aggregation and sending them to Ethereum.

- 67% of the operators behaving dishonestly to be able to submit false proofs.
- 33% of the operators colluding to censor a batch of proofs or task. However, in the case of a batch, the operators can only censor the whole batch, but not a particular proof included in it.
- The aggregator can censor batches or proofs by not sending the aggregated signature.

### What is the batcher?

We have a service called the batcher that batches enough proofs to send to the AVS in Eigen Layer to reduce on-chain verification costs. Users can submit their proofs to Aligned directly without the batcher. The batcher is fully optional. The batcher is an optimization to reduce on-chain verification costs.

### What are the security guarantees added by the batcher?

A batcher can censor proofs. The user can run their own batcher to avoid censorship or can send a task to verify proofs in Aligned via Ethereum without using the batcher. 
The batcher cannot transfer user's funds to other accounts, only spend them to create verification tasks and pay to the aggregator. We recommend to only deposit enough funds for a few months of operations.

### How do I send proofs without a batcher?

### How do I run my own batcher?

### Why build Aligned on top of Ethereum?
    
Ethereum is the most decentralized and most significant source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.

### What is Aligned's throughput?
    
Aligned runs the verifier’s code natively. Verification time depends on the proof system, program run, and public input. Generally, most verifiers can be run on the order of milliseconds on consumer-grade hardware. We can optimize the code for speed and leverage parallelization by running it natively. Our current testnet can verify more than 4000 proofs per second.
    
### How does Aligned's throughput compare with Ethereum?
    
Ethereum runs on top of the EVM. Each block is limited to 30,000,000 gas. Since the most efficient proof systems take at least 250,000 gas, Ethereum can verify 120 proofs per block at most. Aligned runs the verifier's code natively and leverages parallelization, allowing up to 30,000 proofs to be verified per block.

### Why is Aligned necessary?

The EVM was not designed for ZK proof verification, so most verifications are computationally expensive (which also means they have a high gas cost).
    
To solve this, for pairing-based cryptography, Ethereum has added a precompile for verification using the curve BN254.
    
But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast STARKs need efficient hashing for fields. Which is the best field? Mersenne? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?
    
There is a massive amount of ongoing progress in the field of zero-knowledge cryptography, and nobody can predict the endgame.
    
Moreso, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems. Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or some not-yet-created hash.
    
Aligned solves all of these problems. No matter how or what you want to prove, it can be efficiently verified while still inheriting security from Ethereum.
    
### Is Aligned an Ethereum L2?
    
Aligned is related to Ethereum but is not an L2 since it does not produce blocks. It is a decentralized network of verifiers.
    
### Does Aligned compete with L2s?
    
No. Aligned is a decentralized network of verifiers that also uses proof aggregation. It does not produce blocks or generate proofs of execution. Aligned provides L2s with fast and cheap verification for the proofs they generate, reducing settlement costs. It also enhances cross-chain interoperability by making it possible to build quick and cheap bridging solutions.
    
### What does it cost to verify proofs using Aligned?
    
The cost ($C$) of proof verification using Aligned's **fast mode** is the cost of task creation ($C_{task}$) plus the cost of verifying an aggregated BLS signature ($C_{verification}$), divided by the number of proofs ($N$) in the batch, plus the the cost of reading the results on-chain ($C_{read}$).
    
$$
  C =\frac{C_{task} + C_{verification}}{N} + C_{read}
$$

The cost of task creation and signature verification is amortized across $N$ proofs per batch, meaning that Aligned becomes cheaper to use as more proofs are verified at the same time.

### What are BLS signatures?
    
[Boneh-Lynn-Shacham (BLS)](https://en.wikipedia.org/wiki/BLS_digital_signature) signatures are a cryptographic signature that allows a user to verify that a signer is authentic. It relies on elliptic curve pairings and is used by Ethereum due to its aggregation properties.

### Why does Aligned have a fast mode and an aggregation mode?
    
Aligned's fast mode is designed to offer very cheap verification with low latency. It uses cryptoeconomic guarantees provided by restaking, thus deriving its security from Ethereum. With 10 proofs being verified in a batch the per-proof verification cost is approximately 30,000 gas. 

The aggregation mode works using recursive proof aggregation. This results in higher fees and latency, but has the complete security of Ethereum.

### What is proof recursion?
    
Zero-knowledge proofs let you generate proofs that show the correct execution of programs. If a program is the verification of a proof, then we will be getting a proof that we verified the proof and the result was valid. The validity of the second proof implies the validity of the original proof. This is the idea behind proof recursion, and it can be used with two main goals:
    
1. Convert one proof type to another (for example, a STARK proof to a Plonk proof) either to reduce the proof size, have efficient recursion, or because the proof system cannot be verified where we want.
2. Proof aggregation: if we have to verify $N$ proofs on-chain, we can generate a single proof that we verified the $N$ proofs off-chain and just check the single proof on Ethereum.
    
Proof recursion is the primary tool used by Aligned’s aggregation mode.

### What is restaking?
    
EigenLayer introduced the concept of Restaking. It allows Ethereum’s validators to impose additional slashing conditions on their staked ETH to participate in Actively Validated Services (AVS) and earn additional rewards. This creates a marketplace where applications can rent Ethereum's trust without competing for blockspace. Aligned's fast mode is an AVS.

### Is Aligned an aggregation layer?
    
Aligned provides recursive proof aggregation as part of its aggregation mode, a feature shared with all aggregation layers. However, Aligned offers a unique fast mode designed to provide cheap and low-latency proof verification, leveraging the power of restaking via EigenLayer. Aligned is a decentralized network designed to verify zero-knowledge proofs and uses recursive proof aggregation as one of its tools. 
    
### What proof systems do you support?
    
Aligned is designed to support any proof system. We [currently support](../2_architecture/0_supported_verifiers.md) Groth16 and Plonk (gnark), SP1 and Risc0.
    
### How easy is it to add new proof systems?
    
Aligned is designed to make adding new proof systems easy. The only thing needed is the verifier function, which is written in a high-level language like Rust. For example, we could integrate Jolt into one of our testnets just a few hours after it was released.
    
### How does Aligned work?
    
The flow for fast mode verification is as follows:
    
1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum)
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments to all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verify all the proofs. 
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
    
### How can I verify proofs in Aligned?
    
You can verify proofs in Aligned using our CLI or Rust SDK.
    
### Can you provide an estimate of Aligned’s savings?
    
Proof verification directly on Ethereum (not including the cost of accessing/reading) costs: 
    
- Groth 16 proofs: >250,000 gas
- Plonk/KZG proofs: >300,000 gas
- STARKs: >1,000,000 gas
- Binius/Jolt: too expensive to run!
    
With Aligned's fast mode:
    
- Just one proof (any!): 350,000 gas
- Batching 1024 proofs: 350 gas + reading cost
    
This means verifying proofs using Aligned can be 99% cheaper than directly on Ethereum.
    
### I want to verify just one proof. Can I use Aligned for cheap and fast verification?
    
Yes! And all proofs cost the same to verify using Aligned.
    
### Is Aligned open-source?
    
Yes! We are 100% open-source from day one.
    
### What are the goals of Aligned?
    
Aligned is infrastructure that offers fast and cheap verification for zero-knowledge and validity proofs on Ethereum. It can take any proof system and verify it cheaply and quickly, thus accelerating Ethereum's roadmap and its capabilities as a settlement layer for ZK proofs.
    
Aligned aims to make it possible for anyone to build ZK applications. This can only be achieved by:
    
- Reducing operational costs when maintaining a ZK application -> anyone can afford to build ZK apps.
- Offering more options so developers can choose how they want to build their protocols -> everyone can choose their tools.
- Offer the latest ZK technology that allows anyone to build ZK applications by just proving rust -> anyone can code a ZK application.
    
### What’s the role of Aligned in Ethereum?
    
Aligned’s role is to help advance the adoption of zero-knowledge proofs on Ethereum, increase verification throughput, and reduce on-chain verification time and costs. Aligned can easily incorporate proof systems without any further protocol changes to Ethereum. Aligned future-proofs Ethereum for ZK.

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and our goal is to help increase its capabilities.

### What are the use cases for Aligned?
    
We believe that there are many things that will be built using Aligned that we have not even imagined yet. For some possible use cases please see [this page](../1_introduction/2_use_cases.md).

### Why don’t you run Aligned on top of a virtual machine?
    
Running on a virtual machine adds complexity to the system and an additional abstraction layer. It can also reduce Aligned's throughput, which is needed to offer really fast and cheap verification.

### Why don’t you build Aligned on top of a rollup?
    
The main problem with settling on top of a rollup is that you still need confirmation in Ethereum, which adds latency to the process. Besides, most rollups are not fully decentralized; even if they were, it would not be to the same extent as Ethereum. Aligned also already achieves a very low verification cost on Ethereum, so it would not be convenient to build Aligned on top of a rollup in terms of latency, cost, or decentralization. Rollups also need to use the EVM to settle on Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.
    
We are currently focused on bringing Aligned's **fast mode** to Ethereum mainnet but may support verifications on L2 networks in the future.

### Why EigenLayer?
    
We believe Ethereum is the best settlement layer, and ZK will play a key role in helping it become the settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs: so how do we build such a decentralized network that will help Ethereum? 

Creating a new L1 doesn’t benefit Ethereum because it will add new trust assumptions to the Ethereum protocols relying on it. EigenLayer enables Aligned to have the following properties:

1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in ETH
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### How does it compare to the Polygon aggregation layer?

Aligned is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.

### What about TEEs?

Aligned will also verify remote attestations made by Trusted Execution Environments (TEEs). We believe TEEs provide an alternative to ZK in some applications where ZK is too computationally intensive or as an additional mechanism to provide integrity (_e.g._ in multiproofs).
