## Fast mode in a nutshell

## Architecture
Aligned’s architecture is shown in the figure below:

![Figure 1: Architecture fast mode](../images/aligned_architecture.png)

Here, the validators/AVS operators are the ones responsible for proof verification. They fetch the proof data from the data service and verify it using the different proving systems supported by Aligned.

### Flow for sending a proof and publishing the result on Ethereum
The flow for sending a proof and having the results on Ethereum is as follows:
1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits.
2. The batcher answers with a BatchInclusionData for each proof.
3. The user invokes the VerifyBatchInclusion function in the ServiceManager contract with this data to check that the proof has been verified in Aligned and is included in the batch.
4. Then, it is checked that the commitment of the proven program matches the expected one.

### Full flow with internals of the proof

1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum).
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments of all the data submitted by users, uploads the proofs to the Data Service, and creates the verification task in the ServiceManager.
4. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verifies all the proofs.
5. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
6. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
7. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
