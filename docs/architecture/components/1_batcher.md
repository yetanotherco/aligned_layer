# Batcher

The Batcher recieves proofs from different Users, bundle them in a batch of proofs, build a merkle root from these, upload the batch to a data service (like an S3 bucket), and submit this information to the [Aligned Service Manager](./3_service_manager_contract.md).

To avoid trust assumptions, the Batcher has some specific mechanisms;

So that the User is sure that his proof was included in a batch, the Batcher will send to each User the Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from his proof, thus verifying his proof was actually included in the batch.

Also, to avoid unnecesarry proof submissions, the Batcher does a preliminary verification of the submitted proofs, to avoid as much as possible the submission of false proofs in a batch.

But, of course each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. To handle this payment of each batch, the Batcher will submit the batch, but not directly to the Aligned Service Manager, but through its [Batcher Payment Service](./2_payment_service_contract.md).

