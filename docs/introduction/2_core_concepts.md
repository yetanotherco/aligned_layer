## Core Concepts

- Proof: cryptographic data that attests to the validity of a given computation. The concrete data and size depend on the proof system and program used.
- Verification: an algorithm that takes the proof, and associated public data and verification key/program and outputs whether the proof is valid or not.
- Proof batch: a collection of verification tasks to be carried out by the operators.
- User CLI: used to interact with the verifier task batcher. Sends proof and public input data and receives the verification data in Aligned.
- Verifier Task Batcher: Receives tasks from users, creates batches of tasks, publishes the proof and public data in the Data service, and sends the batches’ data to Ethereum. This service is permissionless, meaning users have the option to run their own batcher.
- Service Manager (Ethereum smart contract): Receives the batches’ data and signatures from the BLS signature aggregator. This contract provides information to validators and light clients on the batches/tasks.
- Data service: temporarily stores the data for the proof and public input.
- Operators: responsible for performing the verification of the proofs in each batch and signing messages with the results.
- BLS signature aggregator: receives the signatures from the operators, checks that there is quorum, and performs the aggregation of the signatures.
- Light client: samples random tasks from the Service Manager, checks the proofs, and compares against the results posted to Ethereum by Aligned. If there are differences, it can trigger an L1 verification via the proof service. In case of malicious behavior by Aligned’s operators this would lead to slashing.
- Proof service: receives results from the light clients; in case there are differences with the results posted by Aligned, it triggers a re-verification on Ethereum. Note that this re-verification can also be triggered by any user.
- Proof aggregator: Once tasks have been verified by Aligned’s operators, this component performs recursive proof verification to create one proof that will attest to the validity of all proofs contained in the batch. This proof is verified on-chain.
