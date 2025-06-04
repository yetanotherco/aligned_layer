# Aggregation Mode Deep Dive

The **Aggregation Mode** runs every 24 hours and performs the following steps:

1. **Fetch Proofs from the Verification Layer**  
   Queries `NewBatchV3` events from the `AlignedLayerServiceManager` and downloads the batches from `S3`, starting from the last processed block of the previous run.

2. **Filter Proofs**  
   Filters proofs by supported verifiers and proof types.

3. **Aggregate Proofs in the zkVM**  
   Selected proofs are aggregated using a zkVM.

4. **Construct the Blob**  
   A blob is built containing the commitments of the aggregated proofs.

5. **Send Aggregated Proof**  
   The final aggregated proof and its blob are sent to the `AlignedProofAggregationService` contract for verification.

## Aggregators and Supported Proof Types

Two separate aggregators are run every 24 hours:

-   **Risc0**: Aggregates proofs of types `Composite` and `Succinct`.
-   **SP1**: Aggregates `Compressed` proofs.

## Multilayer Aggregation

To scale aggregation without exhausting zkVM memory, aggregation is split in two programs:

1. **User Proof Aggregator**  
   Processes batches of `n` user proofs. Each run creates an aggregated proof that commits to a Merkle root of the user proofs inputs. This step is repeated for as many chunks as needed. Usually each chunks contains `256` proofs but it can be lowered based on the machine specs.

2. **Chunk Aggregator**  
   Aggregates all chunk-level proofs into a single final proof. It receives:

    - The chunked proofs
    - The original proofs commitments included each chunk received

    During verification, it checks that each chunk’s committed Merkle root matches the reconstructed root to ensure input correctness. The final Merkle root—representing all user proofs—is then committed as a public input.

## Verification

Once aggregated, the proof is sent to Ethereum and verified via the `AlignedProofAggregationService` contract. Depending on the proving system, the contract invokes:

-   `verifySP1` for SP1 proofs
-   `verifyRisc0` for Risc0 proofs

Each function receives:

-   The public inputs
-   The proof binary

The program ID is hardcoded in the contract to ensure only trusted aggregation programs (`chunk_aggregator`) are accepted.

If verification succeeds, the new proof is added to the `aggregatedProofs` map in contract storage.

### Proof Inclusion Verification

To verify a user’s proof on-chain, the following must be provided:

-   The proof bytes
-   The proof public inputs
-   The program ID
-   A Merkle proof

The Merkle root is computed and checked for existence in the contract using the `verifyProofInclusion` function of the `ProofAggregationServiceContract`, which:

1. Computes the merkle root
2. Returns `true` or `false` depending if there exists an `aggregatedProof` with the computed root.

## Data Availability

When submitting the aggregated proof to Ethereum, we include a **blob** that contains the commitments of all the individual proofs that were aggregated. This blob serves two main purposes:

-   It makes the proof commitments publicly available for **18 days**.
-   It allows users to:
    -   Inspect which proofs were aggregated
    -   Get a Merkle proof to verify that their proof is included in the aggregated proof

### Blob capacity

Each blob can hold:

-   `FIELD_ELEMENTS_PER_BLOB = 4096`
-   `BYTES_PER_FIELD_ELEMENT = 32`

Which results in a total theoretical capacity of:

`FIELD_ELEMENTS_PER_BLOB * BYTES_PER_FIELD_ELEMENT` = `4096 * 32` = `131.072 bytes`

However, this full capacity can't be used due to how KZG bytes to elliptic curve points are encoded. Specifically:

-   Ethereum uses the BLS12-381 curve, whose scalar field modulus is slightly less than `2^256`, in fact, it's closer to `2^255`.
-   That means the 32-byte field elements can't represent arbitrary 256-bit values.
-   To stay within the field modulus, we **pad the value with a leading `0x00` byte**, ensuring it's below the modulus.
-   This reduces the usable payload to **31 bytes per field element**.

So the _actual usable capacity_ per blob becomes:

`4096 * 31` = `126.976 bytes`

### Current Bottleneck

Since each proof commitment is exactly **32 bytes**, the maximum number of proof commitments that can fit in a single blob is:

`126.976 / 32` = `3968 proofs`

This is the **current upper limit** on how many proofs we can include in a single aggregation run.

## Scaling Beyond Current Limits

To increase throughput we can:

1. **Send Multiple Blobs per Transaction**  
   Up to **6 blobs** can be included per transaction, supporting up to **23,808 proofs per run**, which is more than we can aggregate in one day.

2. **Run Aggregation More Frequently**  
   Reducing the interval between aggregation runs can also increase throughput.
