## Aggregation Mode deep dive

The aggregation mode runs every 24hs and it consists of the following:

1. Fetching the proofs from the Verification Layer: queries the batches from the `VerificationLayer` (a.k.a FastMode), starting from the last processed block in the previous iteration.
2. Filtering the proofs by the supported verifiers and proof types.
3. Aggregating the proofs in the zkvm.
4. Constructing the blob with the proofs commitments.
5. Sending the final aggregated proof to be verified on the `AlignedProofAggregationService` contract along with the blob.

### Aggregators and supported proof types

Every 24hs, we run two aggregators:

-   Risc0: aggregates Risc0 proofs of type `Composite` and `Succinct`
-   SP1 aggregates SP1 proofs of type `Compressed`.

### Multilayer aggregation

To be able to aggregate more proofs, we have to split the aggregation in various chunks so that the vm does not run out of memory. For this, we perform the aggregation in two steps or two programs:

1. First we run the user proofs aggregator: takes `n` proofs binaries and generates an aggregated proof that commits the merkle root composed of the proofs that it aggregated. This is run as much times as chunks needed.
2. After all user proofs have been aggregated. The `chunk_aggregator` takes all the aggregated chunks and aggregates them into what becomes the final proof. This program takes the chunked proofs + the proofs each chunked proof took. Then at the moment of verifying each chunked proof we also verify that the merkle root it has committed matches the one we reconstruct to make sure the inputs are correct. It is necessary to receive the proofs as the final step of the program consists of constructing the merkle root composed of all the user proofs and commit it as a public input. This merkle root is the one stored in the contract and users use to verify their proof has been aggregated.

**_You might wonder, why is this necessary?_**

The problem is that the zkvms don't perform the recursion taking into account the memory allocation, so we have to apply this limits on the number of proofs to aggregate to able to scale the proof aggregation further.

### Verification

Once the proof is aggregated it is sent to verify on Ethereum to the `AlignedProofAggregationService` contract. Depending of the proving system used in the aggregation it will call:

-   `verifySP1`
-   `verifyRisc0`

This function take the proof public inputs + the proof binary. Then the program id is hardcoded on the contract to make sure only trusted programs can run. In our case, the proof `chunk_aggregator` program.

If the verification goes alright, then a new aggregated proof is added to the `aggregatedProofs` map in the contract storage.

A proof can be verified on-chain by passing the proof bytes + program id and the merkle proof. Then you would compute the merkle root and verify it exists on the `ProofAggregationServiceContract`. This can be done calling `verifyProofInclusion` in the `ProofAggregationServiceContract`.

### Data availability

When sending the proof to Ethereum, we attach a blob with the commitments of all the proofs that were aggregated. This is available in Ethereum for 18 days. This blob is where user would read the proofs that have been aggregated in the final aggregated proof and obtain the merkle proof for their proof to prove that their proof was verified.

Currently the blob capacity is at `FIELD_ELEMENTS_PER_BLOB` \* `BYTES_PER_FIELD_ELEMENT` = `4096 * 32` = `131.072` where `FIELD_ELEMENTS_PER_BLOB` = `4096` and `BYTES_PER_FIELD_ELEMENT` = `32`. But in KZG the bytes are divided in 32 bytes and encoded to a bls12_381 point which has a modulus that takes a bit less than `2^256` (`2^255` to be exact). This means that the 32 bytes can't suprass the `BLS_MODULUS`. The common way to bypass this is to pad with a `0x0` byte at the start. This way, we are left with only `31` usable bytes, so the blob capacity is actually: `4096 * 31` = `126.976`.

Since each proof commitment takes 32 bytes, for each blob we can post as much proofs as: `126.976 / 32` = `3968`.

Currently this is Aligned Proof Aggregation main bottleneck when it comes to scaling, we can aggregate as much `3968` per run. The way to bypass this is to:

1. Add logic to send more than one blob per transaction: we can send as much as 6 blob per transaction, so `23.808` in total, more than we can process.
2. Run the aggregator more frequently.

Note: We are not using implementing any proof of equivalence protocol to prove that the blob data points to the one used in the prover.
