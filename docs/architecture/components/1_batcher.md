# Batcher

Batcher is the Component in charge of recieving User's proofs.

The Batcher will recieve proofs from different Users, build a merkle root from these proofs, upload the proofs to an S3 (so that the operators can download them), and submit this information to the [Aligned Service Manager](./3_service_manager_contract.md).

To avoid trust assumptions, the Batcher has some specific interesting mechanisms;

So that the User is 100% sure that his proof was included in a batch, the Batcher will send to each User the Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from his proof, thus verifying his proof was actually included in the batch.

Also, to avoid unnecesarry proof submitions, the Batcher does a preliminary verification of the submitted proofs, to avoid as much as possible the submition of false proofs in a batch.

But, of course each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. For this, the Batcher will submit the batch, but not directly to the Aligned Service Manager, but through its [Batcher Payment Service](./2_payment_service_contract.md).
