# Operator

The Operators verify the ZK Proofs and are the Eigenlayer restakers. They also insert financial security into the system, and leverage Ethereum's security for any AVS they take part in (e.g. Aligned).

Operators read [Aligned Service Manager](./3_service_manager_contract.md)'s new batch events. These have the necessary information to verify a batch, its Merkle Root, and its data pointer. 

With the data pointer, they will download the actual proofs they will need to verify. The first thing they do after this, is verify that the downloaded proofs actually compute the expected Merkle Root. If not, they will regard the batch as corrupted and will not verify it. This avoids malicious [Batchers](./1_batcher.md) from uploading proofs that are different from what [Users](0_user.md) uploaded.

After verifying the Merkle Root of the batch, thus verifying that the downloaded batch matches the one intended to be submitted, the Operator must now verify each one of its proofs. This is done by executing the appropriate verification programs integrated with Aligned.

After verifying the whole batch, Operators sign their response (either true or false depending on whether the batch was completely verified or not) with a BLS signature, and send it to the [Aggregator](./5_aggregator.md).
