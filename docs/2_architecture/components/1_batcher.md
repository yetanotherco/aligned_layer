# Batcher

The Batcher receives proofs from different Users, bundles them in a batch of proofs, builds a Merkle Root from these, uploads the batch to a data service (like an S3 bucket), and submits this information to the [Aligned Service Manager](./3_service_manager_contract.md).

To ensure that the User is sure that their proof was included in a batch, the Batcher will send each User their Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from their proof, thus verifying it was actually included in the batch.

Also, to avoid unnecessary proof submissions, the Batcher performs preliminary verifications of the submitted proofs in to minimize the submission of false proofs in a batch.

However, each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. To handle the payment for each batch, the Batcher submits the batch through its [Batcher Payment Service](./2_payment_service_contract.md).

To send the batch of proofs to the [Aligned Service Manager](./3_service_manager_contract.md), the Batcher stores the batch of proofs in an S3 for 1 week, and sends the link to the file to the [Aligned Service Manager](./3_service_manager_contract.md).

To view how to submit your own batch, without the use of this Batcher, you may follow [the following guide](../../3_guides/8_submitting_batch_without_batcher.md)


### Max fee priority queue

The batcher queue is now ordered by max fee signed by users in their proof messages - the ones willing to pay more will be prioritized in the batch.

Because of this, a user can't have a proof with higher nonce set with a higher fee included in the batch. For example, consider this situation in a batch. Let the two entries in the batch be from the same address:

	$[(nonce: 1, max\_fee: 5), (nonce: 2, max\_fee: 10)]$

This cannot happen because it will make the message with higher nonce be processed earlier than the one with a lower nonce, hence raising an invalid nonce error.

When a user submits a proof for the first time in the batch, its max fee is cached and set as the user min fee for the rest of the proofs to be included in the batch.
If a later message with a higher max fee is received, the message is rejected and not included in the queue, while if a message with a lower max fee is received,
the message is included in the queue and the user min fee is updated to that value. In summary, **no messages with a higher max fee than this user's min fee will be included**.

In case a message has a max fee that is too low - making it to be stuck in the batcher's *mempool*,  the message can be re-sent and be replaced with a higher fee.
To this end, a validation is done first. We check that when the max fee for the message with that nonce is updated, there is no message with a lower nonce and a lower max fee too, because this would lead to the problem
of messages with higher nonce processed earlier than messages with higher nonce, as discussed earlier.
As an example, consider all these messages in a batch from the same address:

	$[(nonce: 1, max\_fee: 10), (nonce: 2, max\_fee: 5), (nonce: 3, max\_fee: 3)]$

If the user wants to send a replace message for the one with nonce 2, updating the max fee to something greater than 10, that would not be valid.
But it could update the max fee to, for example, a value of 8 and that will work.

## Batch finalization algorithm

There are some analogies in the processing of the batch with respect to how Ethereum handles transactions in the mempool and builds blocks.
We can consider the Batcher priority queue as a sort of *mempool* in the Ethereum case. Once certain conditions are met, the Batcher will start grabbing proofs and try to make a *finalized batch*, which in the analogy is like assembling a block in Ethereum.

When each new block is received, the priority queue's state is queried to know if a finalized batch can start being built.

When the conditions for start building a batch are met, a copy of the priority queue in that moment is created. An empty vector is created to start building the batch and push entries of the priority queue copy there.

Each entry of the priority queue is analyzed one by one, starting from the first element, which is the one with the higher max gas price.
The entry is serialized, its size in bytes is added to the finalized batch size up to the moment. If this size exceeds the max size for posting the batch, the iteration is broken.

If the max size was not reached, then the **gas per proof** is calculated, considering the number **N** of proofs in this pending finalized batch. Then, the Ethereum gas price is queried and multiplied by the gas per proof, obtaining the **fee per proof**.

The obtained fee per proof is compared to the max fee of the priority queue's entry we are analyzing. If it is lower, then the entry is added to the batch, and a boolean is set marking that the batch could be sent if there is no other entry that can be added to the batch.

The whole process is repeated for each entry. The cut conditions for building the batch are either:
* The batch has exceeded its size limit (In a reasonable environment, this should be the most common case)
* One entry is found that is not willing to pay for the calculated price per proof. Due to the ordering in the priority queue, this would mean that all the remaining entries are not willing to pay for that price too.

**Edge case:** If the fee per proof is too high even for the first entry, the algorithm will iterate over every entry and adding it to the finalized batch. If the number of proofs is sufficiently high, making the fee per proof lower and at least one entry is found willing to pay for that price, then the batch can be finalized with all those entries.
If not, the finalization of the batch is suspended and all the process will start again when a new block is received.
