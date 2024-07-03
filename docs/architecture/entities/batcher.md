
# Batcher

Batcher is the Entity in charge of recieving User's proofs.

The Batcher will recieve proofs from different Users, build a merkle root from these proofs, upload the proofs to an S3 (so that the operators can download them), and submit this information to the [Aligned Service Manager](./service_manager.md).

To avoid trust assumptions, the Batcher has some specific interesting mechanisms;

So that the User is 100% sure that his proof was included in a batch, the Batcher will send to each User the Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from his proof, thus verifying his proof was actually included in the batch.

Each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. For this, the batcher has its Batcher Payment Service. It is a simple smart contract where Users must deposit funds. Then, the Batcher will submit the batch, but not directly to the Aligned Service Manager, but through this smart contract, so as to substract the appropriate amount from each User and add it to the batch, to fund this batch's response.
