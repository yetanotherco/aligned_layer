# Use cases

- Soft finality for Rollups and Appchains: Aligned provides fast verification of ZK proofs, which can be used to provide soft finality for rollups or other applications.
- Fast bridging: building ZK bridges requires checking a succinct proof to show that current state of a chain is correct and then users need to show that their account state is correct. Many ZK protocols use hash functions such as Poseidon or Rescue Prime, which do not have Precompiles on Ethereum, making both the verification of the chain's state and account expensive. With Aligned, you can show your account state using another ZK proof, and all proofs can be verified cheaply and with low latency in Aligned.
- New settlement layers (use Aligned + EigenDA) for Rollups and Intent based systems.
- P2P protocols based on SNARKs such as payment systems and social networks.
- Alternative L1s interoperable with Ethereum: similar to fast bridging.
- Verifiable Machine Learning (ML): with general-purpose zkvms we can prove code written in Rust, solving part of the problem of using ML. However, most zkvms use STARK-based proof systems, which leads to high on-chain costs or expensive wrapping. With Aligned, you can directly verify your proof from the zkvm for much less than Ethereum.
- Cheap verification and interoperability for Identity Protocols. 
- ZK Oracles.
- New credential protocols such as zkTLS based systems. 
- ZK Coprocessor.  
- Encrypted Mempools using SNARKs to show the correctness of the encryption.
- Protocols against misinformation and fake news.  
- On-chain gaming.
