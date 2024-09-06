# Batcher

The Batcher receives proofs from different Users, bundles them in a batch of proofs, builds a Merkle Root from these, uploads the batch to a data service (like an S3 bucket), and submits this information to the [Aligned Service Manager](./3_service_manager_contract.md).

To ensure that the User is sure that their proof was included in a batch, the Batcher will send each User their Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from their proof, thus verifying it was actually included in the batch.

Also, to avoid unnecessary proof submissions, the Batcher performs preliminary verifications of the submitted proofs in to minimize the submission of false proofs in a batch.

However, each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. To handle the payment for each batch, the Batcher submits the batch through its [Batcher Payment Service](./2_payment_service_contract.md).

To send the batch of proofs to the [Aligned Service Manager](./3_service_manager_contract.md), the Batcher stores the batch of proofs in an S3 for 1 week, and sends the link to the file to the [Aligned Service Manager](./3_service_manager_contract.md).

To view how to submit your own batch, without the use of this Batcher, you may follow [the following guide](../../3_guides/8_submitting_batch_without_batcher.md)
