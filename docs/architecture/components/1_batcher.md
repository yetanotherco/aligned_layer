# Batcher

The Batcher receives proofs from different Users, bundles them in a batch of proofs, builds a Merkle Root from these, uploads the batch to a data service (like an S3 bucket), and submits this information to the [Aligned Service Manager](./3_service_manager_contract.md).


To ensure that the User is sure that their proof was included in a batch, the Batcher will send each User the Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from their proof, thus verifying it was actually included in the batch.

Also, to avoid unnecessary proof submissions, the Batcher performs a preliminary verification of the submitted proofs, to minimize the submission of false proofs in a batch.

However, each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. To handle the payment for each batch, the Batcher submits the batch through its [Batcher Payment Service](./2_payment_service_contract.md).
