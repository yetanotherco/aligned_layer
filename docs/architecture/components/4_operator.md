# Operator

The Operators are responsible for verifying the ZK Proofs. They are the Eigenlayer restakers. They are the ones who insert the financial security to the system, and leverage Ethereum's security into any AVS they take part of (in this case, Aligned).

Operators read [Aligned Service Manager](./3_service_manager_contract.md) new batch Events. In them, they have the necesarry information to verify a batch, its merkle root and its data pointer. 

With the data pointer, they will download the actual proofs they will need to verify. The first thing they do after this, is verifying the downloaded proofs actually compute the expected Merkle Root. If not, they will regard the batch as corrupted and will nor verify it. This avoids malicious [Batchers](./1_batcher.md) from uploading proofs that are different from what [Users](0_user.md) uploaded.

After verifying the merkle root of batch, thus verifying the batch downloaded is the same as the batch that was intended to be submitted, the Batcher must now verify each one of its proofs. This is done via executing the appropriate verification programs, for example using the SP1 go package to verify an SP1 proof.

After verifying the whole batch, Operators sign their response (either true or false if the batch was completely verified or not) with a BLS signature, and sends it to the [Aggregator](./5_aggregator.md).
