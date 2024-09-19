# Use cases

- Soft finality for Rollups and Appchains: Aligned provides fast verification of ZK proofs, which can be used to provide soft finality for rollups or other applications.
- Fast bridging: building ZK bridges requires checking a succinct proof to show that current state of a chain is correct and then users need to show that their account state is correct. Many ZK protocols use hash functions such as Poseidon or Rescue Prime, which do not have Precompiles on Ethereum, making both the verification of the chain's state and account expensive. With Aligned, you can show your account state using another ZK proof, and all proofs can be verified cheaply and with low latency in Aligned.
- New settlement layers (use Aligned + EigenDA) for Rollups and Intent based systems.
- P2P protocols based on SNARKs such as payment systems and social networks.
- Alternative L1s interoperable with Ethereum: similar to fast bridging.
- Verifiable Machine Learning (ML): with general-purpose zkvms we can prove code written in Rust, solving part of the problem of using ML. However, most zkVMs use STARK-based proof systems, which leads to high on-chain costs or expensive wrapping. With Aligned, you can directly verify your proof from the zkVM for much less than Ethereum.
- Cheap verification and interoperability for Identity Protocols. 
- ZK Oracles: With ZK oracles we can show that we have a piece of information off-chain and produce a ZK proof doing some computation with that data. Aligned reduces the cost of using those oracles. For more background, see the [following post](https://minaprotocol.com/blog/what-are-zkoracles).
- New credential protocols such as zkTLS based systems: you can create proofs of data shown on your web browser and have the result verified in Ethereum. See the following thread for an [ELI5 on TLS](https://x.com/dabit3/status/1830022029195501799) 
- ZK Coprocessor: ZK allows complex computations to be delegated from the blockchain to a coprocessor. This can retrieve information from the blockchain and perform the computations securely in a more efficient way.  
- Encrypted Mempools using SNARKs to show the correctness of the encryption.
- Protocols against misinformation and fake news: you can generate proofs that an image or audio comes from a given device, and show that a published image is the result of certain transformations performed on the original image.  
- On-chain gaming.

## Projects built using Aligned

- The Mina <> Ethereum bridge (in development) uses Aligned's fast mode for ZK proof verification. See the [github repo](https://github.com/lambdaclass/mina_bridge) for more information.
