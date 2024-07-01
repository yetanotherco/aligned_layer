## Fast mode in an nutshell

## Architecture
Aligned’s architecture is shown in the figure below:

![Figure 1: Architecture fast mode](../images/aligned_architecture.png)

The validators/AVS operators are responsible for proof verification. We also provide a light client to sample and check proofs at random. If there is a difference between Aligned’s results and the light clients, a proof service can trigger re-execution of the proof of Ethereum, leading to slashing if malicious behavior is detected.

### Flow for sending a proof and publishing the result on Ethereum (Fast Mode)
The flow for sending a proof and having the results on Ethereum is as follows:
1. Using our CLI or SDK, the user sends one proof (or many) to the batcher.
2. The batcher answers with a ProofVerificationData for each proof.
3. The user invokes an IsVerified function with this data in Solidity to check that the proof is valid.
4. ( Optional ) The user checks that the commitment to the proven program matches the one it expects.

### Full flow with internals of the proof (Fast Mode)

1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum)
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments to all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verifies all the proofs.
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
