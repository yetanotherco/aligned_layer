# Submitting a batch without using the Batcher

Aligned's infrastructure contains a key element, the [Batcher](../2_architecture/components/1_batcher.md),
which bundles many proofs together to reduce the cost of both submission and verification for each proof.

However, the Batcher is not 100% necessary for a User to submit proofs to Aligned.
This is because, as you may have realized, any account can call `createNewTask(bytes32 batchMerkleRoot, string batchDataPointer)` in [Aligned Service Manager Contract](../2_architecture/components/3_service_manager_contract.md).
This means any Ethereum account can create a new task, as long as it supplies a pointer to where the proofs or the batch can be found, and its merkle root.

## Why would I want to submit a batch without using the Batcher?

The answer lies on each User. It may be because:

* A User does not want to share its batch with another User's proofs.
* A User submitter wants to keep sovereignty over the location and storage of its proofs. (Aligned's Batcher stores them on an AWS S3 bucket in the us-east-2 region)
* A User wants to change the lifetime content of his proofs. (Aligned Batcher stores them for 7 days).

## Tradeoffs

As the Batcher bundles proofs together, it reduces the cost for each proof. 
A User who doesn't want to use the Batcher will need to accumulate a large quantity of proofs. 
Otherwise, they will end up paying more per proof.
The User must make sure his account has enough funds in the Aligned Service Manager Contract,
or his batches submissions or responses could fail for lack of funds.

The User should run a pre-verification to make sure he is not wasting funds in submitting a false proof.

## How-to

If, after this analysis, a User still wants to submit his own Batch, here is how:

1. Collect the proof/s he wants to submit to Aligned, build a merkle tree with them and calculate their merkle root.
2. Upload the proofs to any publicly accessible S3-compatible form of storage, in CBOR serialization format, following the following structure:

   ```
   [
       {
           "proving_system": "GnarkGroth16Bn254",
           "proof": [
               152, 88, 141, 155, 88, 35, 94, 76, ...
           ],
           "verification_key": [
               199, 79, 8, 204, 10, 130, 85, 150, ...
           ],
           "vm_program_code": null,
           "proof_generator_addr": "0xd8da6bf26964af9d7eed9e03e53415d37aa96045"
       },
       ...
   ]
   ```
   
   Note: If it is not publicly accessible, if it is not S3-compatible, or if it does not follow the correct batch structure, operators will not be able to download the proofs to verify them.

3. Call `createNewTask(bytes32 batchMerkleRoot, string batchDataPointer)` on the Aligned Service Manager Contract, from a funded Batcher account.

   * To fund a Batcher account, you must either send funds to the Aligned Service Manager Contract beforehand, or you can send funds within the same `createNewTask` function call.
   * `bytes32 batchMerkleRoot` must be the Merkle Root, in format bytes32, of the tree generated with the submitted proofs. If the Operators find the Merkle Root does not correspond to the downloaded batch, they will not verify the proofs.
   * `string batchDataPointer` must be a string containing the pointer (link, url, etc.) to where the proofs are stored.

4. After `createNewTask` is correctly executed, it will emit Events for the other components of Aligned. The Batch has been submitted.
