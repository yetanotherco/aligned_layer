# Proof Aggregation Service Deep Dive

## Architecture Overview

The Proof Aggregation Service consists of three main components that work together to aggregate user proofs and submit them on-chain.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PROOF AGGREGATION SERVICE                           │
└─────────────────────────────────────────────────────────────────────────────┘

  ┌──────────┐         ┌───────────────────────────────┐
  │   User   │         │ AggregationModePaymentService │
  │          │         │          (Contract)           │
  └────┬─────┘         └───────────────┬───────────────┘
       │                               │
       │  1. Deposit ETH               │
       │──────────────────────────────>│
       │                               │
       │                               │  Payment Event
       │                               │
  ┌────┴─────┐         ┌───────────────┴───────────────┐
  │   User   │         │       Payments Poller         │
  │          │         │                               │
  └────┬─────┘         └───────────────┬───────────────┘
       │                               │
       │                               │  2. Monitor payments
       │                               │     Update quotas
       │                               │
       │               ┌───────────────┴───────────────┐
       │               │         PostgreSQL DB         │
       │               │                               │
       │               └───────────────┬───────────────┘
       │                               │
  ┌────┴─────┐         ┌───────────────┴───────────────┐
  │   User   │         │           Gateway             │
  │          │         │                               │
  └────┬─────┘         └───────────────┬───────────────┘
       │                               │
       │  3. Submit proof              │
       │──────────────────────────────>│
       │                               │
       │                               │  4. Verify & Store
       │                               │     proof in DB
       │                               │
       │               ┌───────────────┴───────────────┐
       │               │       Proof Aggregator        │
       │               │                               │
       │               └───────────────┬───────────────┘
       │                               │
       │                               │  5. Fetch pending proofs
       │                               │  6. Aggregate in zkVM
       │                               │
       │               ┌───────────────┴───────────────┐
       │               │ AlignedProofAggregationService│
       │               │          (Contract)           │
       │               └───────────────────────────────┘
       │                               │
       │                               │  7. Submit aggregated
       │                               │     proof on-chain
       │                               │
  └──────────┘
```

## Workflow

### Step 1: User Payment

Users deposit ETH into the **AggregationModePaymentService** contract to obtain quota for submitting proofs.

```
┌──────────┐                    ┌─────────────────────────────────┐
│   User   │───── deposit() ───>│  AggregationModePaymentService  │
└──────────┘                    └─────────────────────────────────┘
```

### Step 2: Payments Poller Updates Quotas

The **Payments Poller** monitors the `AggregationModePaymentService` contract for deposit events and updates user quotas in the database.

```
┌─────────────────────────────────┐         ┌──────────────┐         ┌──────────┐
│ AggregationModePaymentService   │──event─>│   Payments   │──update─>│    DB    │
│                                 │         │    Poller    │  quota   │          │
└─────────────────────────────────┘         └──────────────┘         └──────────┘
```

### Step 3: User Submits Proof to Gateway

Users submit their proofs to the **Gateway** via HTTP API. The gateway:

1. Verifies the user has sufficient quota
2. Validates the proof format and signature
3. Verifies the proof is valid (SP1 compressed proof)

```
┌──────────┐                    ┌──────────────┐
│   User   │───── submit() ────>│   Gateway    │
│          │<──── receipt ──────│              │
└──────────┘                    └──────────────┘
```

### Step 4: Gateway Stores Proof in Database

After validation, the gateway stores the proof in PostgreSQL for later aggregation.

```
┌──────────────┐                ┌──────────────┐
│   Gateway    │───── store ───>│  PostgreSQL  │
│              │     proof      │      DB      │
└──────────────┘                └──────────────┘
```

### Step 5: Proof Aggregator Processes Proofs

The **Proof Aggregator** runs periodically (every 24 hours) and:

1. Fetches pending proofs from the database
2. Filters proofs by supported verifiers
3. Aggregates proofs using the zkVM
4. Constructs a blob with proof commitments
5. Submits the aggregated proof on-chain

```
┌──────────────┐         ┌──────────────┐         ┌─────────────────────────────────┐
│  PostgreSQL  │──fetch─>│    Proof     │──submit─>│ AlignedProofAggregationService  │
│      DB      │         │  Aggregator  │          │                                 │
└──────────────┘         └──────────────┘         └─────────────────────────────────┘
```

## Supported Proof Types

The aggregation service currently supports:

-   **SP1**: Aggregates proofs of type `Compressed`

## Proof Commitment

The **proof commitment** is a hash that uniquely identifies a proof. It is defined as the keccak of the proof public inputs + program ID:

-   **For SP1**:
    The commitment is computed as: `keccak(proof_public_inputs_bytes || vk_hash_bytes)`

## Multilayer Aggregation

To scale aggregation without exhausting zkVM memory, aggregation is split into two programs:

```
                        User Proofs (n per chunk)
                                  │
          ┌───────────────────────┼───────────────────────┐
          │                       │                       │
          ▼                       ▼                       ▼
   ┌─────────────┐         ┌─────────────┐         ┌─────────────┐
   │   Chunk 1   │         │   Chunk 2   │         │   Chunk N   │
   │  Aggregator │         │  Aggregator │         │  Aggregator │
   └──────┬──────┘         └──────┬──────┘         └──────┬──────┘
          │                       │                       │
          │    Aggregated Proofs + Merkle Roots           │
          │                       │                       │
          └───────────────────────┼───────────────────────┘
                                  │
                                  ▼
                        ┌─────────────────┐
                        │     Chunk       │
                        │   Aggregator    │
                        └────────┬────────┘
                                 │
                                 ▼
                        ┌─────────────────┐
                        │  Final Proof +  │
                        │   Merkle Root   │
                        └─────────────────┘
```

1. **User Proof Aggregator**
   Processes chunks of `n` user proofs. Each run creates an aggregated proof that commits to a Merkle root of the user proofs inputs. This step is repeated for as many chunks as needed. Usually each chunk contains `256` proofs but it can be lowered based on the machine specs.

2. **Chunk Aggregator**
   Aggregates all chunk-level proofs into a single final proof. It receives:

    - The chunked proofs
    - The original [proofs commitments](#proof-commitment) included in each chunk received

    During verification, it checks that each chunk's committed Merkle root matches the reconstructed root to ensure input correctness. The final Merkle root, representing all user [proofs commitments](#proof-commitment), is then committed as a public input.

## Verification

Once aggregated, the proof is sent to Ethereum and verified via the `AlignedProofAggregationService` contract. The contract invokes `verifySP1` which receives:

-   The public inputs
-   The proof binary

The program ID is hardcoded in the contract to ensure only trusted aggregation programs (`chunk_aggregator`) are accepted.

If verification succeeds, the new proof is added to the `aggregatedProofs` map in contract storage.

### Proof Inclusion Verification

To verify a user's proof on-chain, the following must be provided:

-   The proof bytes
-   The proof public inputs
-   The program ID (vk hash)
-   A Merkle proof

The Merkle root is computed and checked for existence in the contract using the `verifyProofInclusion` function of the `AlignedProofAggregationService` contract, which:

1. Computes the merkle root
2. Returns `true` or `false` depending on whether there exists an `aggregatedProof` with the computed root.

## Data Availability

When submitting the aggregated proof to Ethereum, we include a **blob** that contains the [commitments](#proof-commitment) of all the individual proofs that were aggregated. This blob serves two main purposes:

-   It makes the [proof commitments](#proof-commitment) publicly available for **18 days**.
-   It allows users to:
    -   Inspect which proofs were aggregated
    -   Get a Merkle proof to verify that their proof is included in the aggregated proof
