## Key Terms

### Proof

A **Proof** is cryptographic data that can attest to the validity of a given computation. The concrete data and size depend on the proof system and program used.

### Verification

The **Verification** is an algorithm that takes the proof, and associated public data and verification key/program and outputs whether the proof is valid or not.

### Proof Batch

A **Proof batch** is a collection of verification tasks to be carried out by the Operators.

### User CLI

The **User CLI** is used to interact with the Verifier Task Batcher. It sends proof and public input data and receives the verification data in Aligned.

### Verifier Task Batcher

The **Verifier Task Batcher** receives tasks from users, creates batches of tasks, publishes the proof and public data in the Data service, and sends the batches’ data to Ethereum. This service is permissionless, meaning users have the option to run their own batcher.

### Service Manager

The **Service Manager (Ethereum smart contract)** receives the batches’ data and signatures from the BLS signature aggregator. This contract provides information to validators and Light Clients on the batches/tasks.

### Data Service

The **Data Service** temporarily stores the data for the proof and public input.

### Operators

The **Operators** are responsible for performing the verification of the proofs in each batch and signing messages with the results.

### BLS Signature Aggregator

The **BLS Signature Aggregator** receives the signatures from the Operators, checks if a quorum is reached, if so, it performs the aggregation of the signatures.

### Light Clients

The **Light Client** samples random tasks from the Service Manager, checks the proofs, and compares against the results posted to Ethereum by Aligned. If there are differences, it can trigger an L1 verification via the proof service. In case of malicious behavior by Aligned’s Operators, this would lead to slashing.

### Proof Service

The **Proof Service** receives results from the Light Clients; in case there are differences with the results posted by Aligned, it triggers a re-verification on Ethereum. Note that this re-verification can also be triggered by any user.

### Proof Aggregator

The **Proof aggregator**, once tasks have been verified by Aligned’s Operators, performs recursive proof verification to create one proof that will attest to the validity of all proofs contained in the batch. This proof is verified on-chain.
