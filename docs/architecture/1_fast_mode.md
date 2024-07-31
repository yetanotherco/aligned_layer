## Fast mode in a nutshell

## Architecture
Aligned’s architecture is shown in the figure below:

![Figure 1: Architecture fast mode](../images/aligned_architecture.png)

Here, the validators/AVS operators are the ones responsible for proof verification. They fetch the proof data from the data service and verify it using the different proving systems supported by Aligned.

### Flow for sending a proof and publishing the result on Ethereum

The flow for sending a proof and having the results on Ethereum is as follows:
1. The user uses a provided CLI or SDK to send one or many proofs to the batcher, and waits.
2. The batcher answers with a ValidityResponse for each proof (if proof, nonce and signature are valid).
3. The batcher answers with a BatchInclusionData for each proof.
4. The user invokes the VerifyBatchInclusion function in the ServiceManager contract with this data to check that the proof has been verified in Aligned and is included in the batch.
5. Then, it is checked that the commitment of the proven program matches the expected one.

### Full flow with internals of the proof

1. The user uses a provided CLI or SDK to send one proof or many to the batcher, and waits (Alternatively, the user can run a batcher or interact directly with Ethereum).
2. The batcher accumulates proofs of many users for a small number of blocks (typically 1-3).
3. The batcher creates a Merkle Tree with commitments of all the data submitted by users, uploads the proofs to the Data Service,
  and submits it to the [Batcher Payment Service](./components/2_payment_service_contract.md)
4. The Batcher Payment Service rebuilds the merkle tree, and verifies user signatures and nonces.
5. The Batcher Payment Service sends the batch to the [Aligned Service Manager](./components/3_service_manager_contract.md).
6. The operators, using the data in Ethereum, download the proofs from the DataService. They then verify that the Merkle root is equal to the one in Ethereum, and verifies all the proofs.
7. If the proofs are valid, they sign the root and send this to the BLS signature aggregator.
8. The signature aggregator accumulates the signed responses until reaching the quorum, then sends the aggregated signature to Ethereum.
9. Ethereum verifies the aggregated signatures and changes the state of the batch to verified.
