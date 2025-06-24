# Aggregation Mode Deep Dive

The Aggregation Mode runs **once every 24 hours** and performs the following steps:

1. **Fetch Proofs from the Verification Layer**  
   Queries `NewBatchV3` events from the `AlignedLayerServiceManager` and downloads the batches from `S3`, starting from the last processed block of the previous run.

2. **Filter Proofs**  
   Filters proofs by supported verifiers and proof types.

3. **Aggregate Proofs in the zkVM**  
   Selected proofs are aggregated using a zkVM.

4. **Construct the Blob**  
   A blob is built containing the [commitments](#proof-commitment) of the aggregated proofs.

5. **Send Aggregated Proof**  
   The final aggregated proof and its blob are sent to the `AlignedProofAggregationService` contract for verification.

> [Note]
> Currently if you want your proof to be verified in the `AggregationMode` you need to submit it via the `VerificationLayer`. In the future, users will be able to decide if they want to use any of the modes in particular or both of them

## Aggregators and Supported Proof Types

Two separate aggregators are run every 24 hours:

-   **Risc0**: Aggregates proofs of types `Composite` and `Succinct`.
-   **SP1**: Aggregates proofs of type `Compressed`.

## Proof Commitment

The **proof commitment** is a hash that uniquely identifies a proof. It is defined as the keccak of the proof public inputs + program ID:

-   **For SP1**:  
    The commitment is computed as: `keccak(proof_public_inputs_bytes || vk_hash_bytes)`
-   **For Risc0**:  
    The commitment is computed as: `keccack(receipt_public_inputs_bytes || image_id_bytes)`

## Multilayer Aggregation

To scale aggregation without exhausting zkVM memory, aggregation is split in two programs:

1. **User Proof Aggregator**  
   Processes chunks of `n` user proofs. Each run creates an aggregated proof that commits to a Merkle root of the user proofs inputs. This step is repeated for as many chunks as needed. Usually each chunks contains `256` proofs but it can be lowered based on the machine specs.

2. **Chunk Aggregator**  
   Aggregates all chunk-level proofs into a single final proof. It receives:

    - The chunked proofs
    - The original [proofs commitments](#proof-commitment) included each chunk received

    During verification, it checks that each chunk’s committed Merkle root matches the reconstructed root to ensure input correctness. The final Merkle root, representing all user [proofs commitments](#proof-commitment), is then committed as a public input.

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

When submitting the aggregated proof to Ethereum, we include a **blob** that contains the [commitments](#proof-commitment) of all the individual proofs that were aggregated. This blob serves two main purposes:

-   It makes the [proof commitments](#proof-commitment) publicly available for **18 days**.
-   It allows users to:
    -   Inspect which proofs were aggregated
    -   Get a Merkle proof to verify that their proof is included in the aggregated proof
