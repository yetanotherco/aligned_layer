# FAQ

### What is the objective of Aligned?
    
Aligned’s mission is to extend Ethereum’s zero-knowledge capabilities. We are certain the zero-knowledge proofs will have a key role in the future of blockchains and computation. We don’t know what that future will look like, but we are certain it will be in Ethereum. The question we want to share is: If we are certain zero-knowledge proofs are the future of Ethereum but we are not certain which of the many possible zero-knowledge futures will win. How can we build an infrastructure for Ethereum to be compatible with any future zero-knowledge proving system?
    
### What is the throughput of Aligned?
    
Aligned runs the verifier’s code natively. The verification time depends on the proof system, program run, and public input. Generally, most verifiers can be run in the order of ms on consumer-end hardware. We can optimize the code for speed and leverage parallelization by running it natively. Taking 3 ms per proof, Aligned could verify 300 proofs per second and, using parallelization, over 10,000 proofs per second.
    
### How does the throughput of Aligned compare with Ethereum?
    
Ethereum runs on top of the EVM. Each block is limited to 30,000,000 gas. Since the most efficient proof systems take at least 250,000 gas, Ethereum can verify 120 proofs per block. Aligned runs the code natively and leverages parallelization, reaching 10,000 proofs in the same period.
    
### Is Aligned an Ethereum L2?
    
Aligned is related to Ethereum but is not an L2 since it does not produce blocks. It is a decentralized network of verifiers.
    
### Does Aligned compete with L2s?
    
No. Aligned is a decentralized network of verifiers and has proof aggregation. It does not produce blocks or generate proofs of execution. Aligned provides L2s with fast and cheap verification for the proofs they generate, reducing settlement costs and enhancing cross-chain interoperability with quick and cheap bridging.
    
### What are the costs for Aligned?
    
The costs depend on task creation, aggregated signature or proof verification, and reading the results. The cost C per proof by batching N proofs is roughly:
    
$$
  C =\frac{C_{task} + C_{verification}}{N} + C_{read}
$$
    
Batching 1024 proofs using Aligned’s fast mode can cost around 2,100 gas in Ethereum (for a gas price of 8 gwei/gas and ETH = $3000, $0.05). As a helpful comparison, a transaction in Ethereum costs 21,000 gas, so you get proof verification for 1/10th of the transaction cost!
    
### Why do you have a fast and slow mode?
    
The fast mode is designed to offer very cheap verification costs and low latency. It uses crypto-economic guarantees provided by restaking; costs can be as low as 2100 gas. The slow mode works with proof aggregation, with higher fees and latency, and achieves the complete security of Ethereum. We verify an aggregated BLS signature (around 113,000 gas) in the fast mode. We verify an aggregated proof (around 300,000 gas) in the slow mode.
    
### Why don’t you run Aligned on top of a virtual machine?
    
Running on a virtual machine adds complexity to the system and an additional abstraction layer. It can also reduce Aligned's throughput, which is needed to offer really fast and cheap verification.
    
### Why don’t you build Aligned on top of a rollup?
    
The main problem with settling on top of a rollup is that you still need confirmation in Ethereum, which adds latency to the process. Besides, most rollups are not fully decentralized, even if they were, not to the extent of Ethereum. Aligned also achieves an already low verification cost in Ethereum, so it would not be convenient to build Aligned on top of a rollup in terms of latency, costs, and decentralization.
    
An L2 needs to use the EVM to settle in Ethereum. This means that the proofs need to be efficiently verified in the EVM, and their data made available there.
    
The EVM is not designed for ZK Verification, so most verifications are expensive.
    
To solve this, for pairing-based cryptography, Ethereum has added a precompile for verifications using the curve BN254.
    
But technology changes fast. BN254 security was demonstrated to be around 100 bits instead of the expected 128. Fast Starks need efficient hashing for fields. Which is the best field? Mersenne’s? Goldilocks? Binary fields? What about the sumcheck protocol? Is Jolt the endgame? Or is GKR going to be faster?
    
The amount of progress in the field is big, and nobody can predict the endgame.
    
Even more, it would be naive to think that only one optimized prover will exist in the future. In the world of ZK, as in many others, there are trade-offs and systems that solve different problems.
    
Maybe we want faster proving and don't care about proof size. Maybe we want the fastest proof verification and smallest size and can do more work on the prover. The system may be optimized to prove Keccak really fast. Or we can skip the traditional hashes altogether and just optimize for Poseidon, Rescue, or one hash not created yet.
    
Aligned solves all of this. No matter how or what you want to prove, it can be verified efficiently here while still inheriting the security of Ethereum as other L2s.
    
### Is Aligned an aggregation layer?
    
Aligned provides proof aggregation as part of its slow mode, a feature shared with all aggregation layers. However, Aligned offers a unique fast mode designed to provide cheap and low-latency proof verification, leveraging the power of restaking. Aligned is a decentralized network designed to verify zero-knowledge proofs and uses recursive proof aggregation as one of its tools. 
    
### What proof systems do you support?
    
Aligned is designed to support any proof system. Currently supported ones are Groth 16 and Plonk (gnark), SP1, Halo 2 (IPA and KZG)
    
### How hard is it to add new proof systems?
    
Aligned is designed to make adding new proof systems easy. The only thing needed is the verifier function, which is written in a high-level language like Rust. For example, we could integrate Jolt into one of our testnets just a few hours after it was released.
    
### What are BLS signatures?
    
[Boneh-Lynn-Shacham](https://en.wikipedia.org/wiki/BLS_digital_signature) is a cryptographic signature that allows a user to verify that a signer is authentic. It relies on elliptic curve pairings and is used by Ethereum due to its aggregation properties.
    
### How does Aligned work?
    
The flow for fast verification is as follows:
    
1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum)
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments to all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verify all the proofs. 
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
   
### What is restaking?
    
EigenLayer introduced the concept of Restaking. It allows Ethereum’s validators to impose additional slashing conditions on their staked ETH to participate in Actively Validated Services (AVS) and earn additional rewards. This creates a marketplace where applications can rent Ethereum's trust without competing for blockspace. Aligned is an example of an AVS.
    
### How can I verify proofs in Aligned?
    
You can verify proofs in Aligned using our CLI.
    
### Can you provide an estimate of Aligned’s savings?
    
In Ethereum (does not include access cost): 
    
- Groth 16 proofs: 250,000 gas
- Plonk/KZG proofs: >300,000 gas
- STARKs: >1,000,000 gas
- Binius/Jolt: too expensive to run!
    
In Aligned, fast mode:
    
- Just one proof (any!): 120,000 gas
- Batching 1024 proofs: 120 gas + reading cost
    
It’s over 99% savings!
    
### I want to verify just one proof. Can I use Aligned for cheap and fast verification?
    
Yes!
    
### Is Aligned open-source?
    
Yes!
    
### What are the goals of Aligned?
    
Aligned is an infrastructure that offers fast and cheap verification for zero-knowledge and validity proofs. It can take any proof system and verify it cheaply and fast.
    
This means that what Aligned wants to achieve is to allow anyone to build zk applications. This can only be achieved by:
    
- Reducing operational costs when maintaining a zk application -> anyone can afford to build zk apps.
- Offering more options so developers can choose how they want to build their protocols -> everyone can choose their tools.
- Offer the latest zk that allows anyone to build zk applications by just proving rust -> anyone can code a zk application.
    
### What’s the role of Aligned in Ethereum?
    
Aligned’s role is to help advance the adoption of zero-knowledge proofs in Ethereum, increase verification throughput, and reduce on-chain verification time and costs. Aligned can easily incorporate proof systems without any further changes in Ethereum. In a more straightforward analogy, Aligned is like a GPU for Ethereum. 
    
### What is proof recursion?
    
Zero-knowledge proofs let you generate proofs that show the correct execution of programs. If a program is the verification of a proof, then we will be getting a proof that we verified the proof and the result was valid. The validity of the second proof implies the validity of the original proof. This is the idea behind proof recursion, and it can be used with two main goals:
    
1. Convert one proof type to another (for example, a STARK proof to a Plonk proof) either to reduce the proof size, have efficient recursion, or because the proof system cannot be verified where we want.
2. Proof aggregation: if we have to verify N proofs on-chain, we can generate a single proof that we verified the N proofs off-chain and just check the single proof in Ethereum.
    
Proof recursion is the primary tool of Aligned’s slow mode.
    
### What are the use cases of Aligned?
    
Among the possible use cases of Aligned, we have:
    
Soft finality for Rollups and Appchains, fast bridging, new settlement layers (use Aligned + EigenDA) for Rollups and Intent-based systems, P2P protocols based on SNARKs such as payment systems and social networks, alternative L1s interoperable with Ethereum, Verifiable Machine Learning, cheap verification and interoperability for Identity Protocols, ZK Oracles, new credential protocols such as zkTLS based systems, ZK Coprocessor, encrypted Mempools using SNARKs to show the correctness of the encryption, protocols against misinformation and fake news, and on-chain gaming.
    
### Why build Aligned on top of Ethereum?
    
Ethereum is the most decentralized and most significant source of liquidity in the crypto ecosystem. We believe it is the most ambitious and long-term project on the internet. Aligned is being built to help Ethereum achieve its highest potential, and we believe this is only possible through validity/zero-knowledge proofs.
    
### Why EigenLayer?
    
We believe Ethereum is the best settlement layer, and zero-knowledge will play a key role in helping it become the settlement layer of the internet. We want to build a verification layer that helps Ethereum achieve this goal. This layer needs to have a decentralized group of validators that will just re-execute the verification of different proofs, but how can we build such a decentralized network that will help Ethereum? Creating a new L1 doesn’t benefit Ethereum because it will add new trust assumptions to the Ethereum protocols relying on it. So, if we must have:
1. A decentralized network of verifiers
2. A similar economic security level that can be easily measured in Ethereum
3. Part of the Ethereum ecosystem
4. Flexible enough to support many current and future proving systems

### How does it compare to the Polygon aggregation layer?

Aligned is just a network of decentralized verifiers renting security from Ethereum. On the other hand, the Polygon aggregation layer, in essence, is a rollup verifying multiple proofs. That is not the case for Aligned, which just executes a rust binary from different verifiers directly in multiple Ethereum validators.

### Why do we need a ZK verification layer?

Verifiable computation allows developers to build applications that help Ethereum scale or even create applications that were not possible before, with enhanced privacy properties. We believe the future of Ethereum will be shaped by zero-knowledge proofs and help it increase its capabilities.