# Batcher

> **Deprecated.** This page documents the Aligned Verification Layer, which is no
> longer active. It is kept for historical reference only. Aligned's active
> product is the [Proof Aggregation Service](../../../2_proof_aggregation_layer/architecture/1_overview.md).

The Batcher receives proofs from different Users, bundles them in a batch of proofs, builds a Merkle Root from these, uploads the batch to a data service (like an S3 bucket), and submits this information to the [Aligned Service Manager](./3_service_manager_contract.md).

To ensure that the User can be certain that their proof was included in a batch, the Batcher will send each User their Merkle Proof (or Merkle Path). With this, the User can rebuild the Merkle Root starting from their proof, thus verifying it was actually included in the batch.

Also, to avoid unnecessary proof submissions, the Batcher performs preliminary verifications of the submitted proofs in to minimize the submission of false proofs in a batch.

However, each proof has a cost of verification, so each batch must contain some sort of payment for it to be verified. To handle the payment for each batch, the Batcher submits the batch through its [Batcher Payment Service](./2_payment_service_contract.md).

To send the batch of proofs to the [Aligned Service Manager](./3_service_manager_contract.md), the Batcher stores the batch of proofs in an S3 for 1 week, and sends the link to the file to the [Aligned Service Manager](./3_service_manager_contract.md).

To view how to submit your own batch, without the use of this Batcher, you may follow [the following guide](../../3_guides/9_submitting_batch_without_batcher.md)


### Max fee priority queue

The batcher queue is now ordered by `max_fee` signed by users in their proof messages - the ones willing to pay more will be prioritized in the batch.

Because of this, a user can't have a proof with higher nonce set with a higher fee included in the batch. For example, consider this situation in a batch. Let the two entries in the batch be from the same address:

	[(nonce: 1, max_fee: 5), (nonce: 2, max_fee: 10)]

This shouldn't happen because it would make the message with higher nonce be processed earlier than the one with a lower nonce, hence it will raise an invalid nonce error.

When a user submits a proof for the first time in the batch, its `max_fee` is stored and set as the `user_min_fee` for the rest of the proofs to be included in the batch. It is called this way because it means the minimum fee a user has sent.
If a later message with a higher `max_fee` is received, the message is rejected and not included in the queue, while if a message with a lower `max_fee` is received,
the message is included in the queue and the `user_min_fee` is updated to that value.
In summary, **no messages with a higher `max_fee` than the `user_min_fee` will be included**.

In case a message has a `max_fee` that is too low - allowing it to be stuck in the batcher's queue, the message can be re-sent (called *replacement message*) to bump its `max_fee` with a higher fee.
For this, a validation is done first. We check that when the `max_fee` for the message with that nonce is updated, there is no message with a lower nonce and a lower `max_fee`, because this would lead to the problem
of messages with higher nonce processed earlier than messages with lower nonce, as discussed earlier.
As an example, consider all these messages in a batch from the same address:

	[(nonce: 1, max_fee: 10), (nonce: 2, max_fee: 5), (nonce: 3, max_fee: 3)]

If the user wants to send a replacement message for the message with nonce 2, updating the max fee to 11, it wouldn't be valid.
But it could update the max fee to, for example, 8.

## Batch finalization algorithm

There are some analogies in the processing of the batch with respect to how Ethereum handles transactions in the mempool and builds blocks.
We can consider the Batcher priority queue as a sort of *mempool* in the Ethereum case. Once certain conditions are met, the Batcher will try to make a *finalized batch*, containing the maximum amount of highest paying messages, which in the analogy is like assembling a block in Ethereum, grabbing the highest valued transactions.

When the conditions to build a batch are met, the Batcher runs the following batch finalization algorithm to create a batch of proofs from the priority queue.

This algorithm starts by calculating the **batch size**, in bytes, by adding the verification data bytes of each proof of the queue. The next step is to build a new **resulting priority queue**, which will store all proofs that where not included in the batch, replacing the current priority queue when this algorithm ends.

In order for the batch to be considered valid, two conditions have to be met:
* The **batch size** in bytes must be less than or equal to a defined limit.
* All proofs found in the batch must have a `max_fee` equal or higher to the calculated **fee per proof** of the batch.

The **fee per proof** indicates the cost of verifying each proof within the batch. It is calculated using a formula that depends on the **batch length**, defined as the number of proofs in the batch:

```
gas_per_proof = (constant_gas_cost + additional_submission_gas_cost_per_proof * batch_len) / batch_len
fee_per_proof = gas_per_proof * gas_price
```

Since the priority queue is sorted in ascending order of `max_fee`, we can be certain that if the proof with the smallest `max_fee` complies with the **fee per proof** rule, then all remaining proofs in the queue will do so

```
priority_queue = [(proof_a, 87), (proof_b, 90), (proof_c, 99)]
```

The algorithm attempts to build new batch by iterating on each proof, starting with the one with the smallest `max_fee` in the queue. On each iteration, the **batch size** and **fee per proof** will be recalculated and both conditions reevaluated. When both conditions are met, all proofs remaining in the queue will be used to build the new batch. The remaining proofs, stored in the **resulting priority queue**, will be candidates to the next batch finalization algorithm execution.

So, instead of the batcher "building" a batch by adding valid messages, it builds a batch by gradually not considering the cheapest ones, until a valid one is found.

There is an edge case for this algorithm: If the fee per proof is too high even for the highest `max_fee` proof, the algorithm will iterate over each proof until the **priority queue** is empty. 
This means no proof allowed enough `max_fee` to be included in a batch.
If this happens, the finalization of the batch is suspended and all the process will start again when a new block is received. 

Let's see a very simple example, the algorithm starts with the following state:

```
priority_queue = [(E, 74), (D, 75), (C, 90), (B, 95), (A, 100)]
resulting_priority_queue = []
max_batch_size = 1000 # Defined constant
```

On the first iteration, the proof with the smallest `max_fee` is taken and the **fee per proof** is calculated

```
priority_queue = [(E, 74), (D, 75), (C, 90), (B, 95), (A, 100)]
resulting_priority_queue = []
current_proof = (E, 74)
fee_per_proof = calculate_fee_per_proof(priority_queue) # Result: 70
batch_size_bytes = calculate_batch_size(priority_queue) # Result: 1150
```

This batch can't be finalized, since it exceeds the maximum batch limit of 1000 bytes. This proof will be discarded for the current batch and stored in the **resulting priority queue**

```
priority_queue = [(D, 75), (C, 90), (B, 95), (A, 100)]
resulting_priority_queue = [(E, 74)]
current_proof = (D, 75)
fee_per_proof = calculate_fee_per_proof(priority_queue) # Result: 76
batch_size_bytes = calculate_batch_size(priority_queue) # Result: 990
```

This batch won't be finalized either, since the **fee per proof** of the batch is higher than the `max_fee` of the current proof. This proof will be discarded for the current batch and stored in the **resulting priority queue**

```
priority_queue = [(C, 90), (B, 95), (A, 100)]
resulting_priority_queue = [(E, 74), (D, 75)]
current_proof = (C, 90)
fee_per_proof = calculate_fee_per_proof(priority_queue) # Result: 90
batch_size_bytes = calculate_batch_size(priority_queue) # Result: 850
```

All proofs in this batch comply with the `max_fee` and **fee per proof** condition, and the batch size is lower than the established limit, so this batch will be finalized!

The execution ends with the following state for the batcher: a new batch is created , and the **priority queue** is replaced by the **resulting priority queue**

```
new_batch = [A, B, C] # Batch to send
priority_queue = [(E, 74), (D, 75)] # New priority queue
```
