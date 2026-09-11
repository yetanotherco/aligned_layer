# Key Terms

### Proof

A **Proof** is cryptographic data that can attest to the validity of a given computation. The concrete data and size depend on the proof system and program used.

### Verification

The **Verification** is an algorithm that takes the proof, and associated public data and verification key/program and outputs whether the proof is valid or not.

### Program ID

The **Program ID** identifies the program a proof was generated for. For SP1 it is the hash of the verifying key; for Risc0 it is the image ID.

### Proof Commitment

The **Proof commitment** is a hash that uniquely identifies a submitted proof. It is computed as the keccak of the proof's public inputs concatenated with the program ID.

### Gateway

The **Gateway** receives proofs from users, validates them against the sender's quota, and stores them until they are picked up for aggregation.

### Payment Service

The **AggregationModePaymentService** is the Ethereum contract users deposit into to fund their proof submission quota.

### Payments Poller

The **Payments Poller** watches the payment service contract for deposits and credits the corresponding quota to the user.

### Proof Aggregator

The **Proof Aggregator** performs recursive proof verification to create one proof that attests to the validity of all the proofs it received. This aggregated proof is what gets verified on-chain.

### Aggregation Window

The **Aggregation window** is how long a submitted proof may wait before being aggregated and settled on Ethereum. It defaults to 24 hours.

### Merkle Root and Proof Inclusion

Each aggregated proof commits to a **Merkle root** built from the proof commitments it contains. To show that a specific proof was verified, a user provides a **Merkle proof** against that root, which the `AlignedProofAggregationService` contract checks.

### Data Availability Blob

When the aggregated proof is submitted to Ethereum, the proof commitments it covers are published in a **blob**. This makes them publicly available for 18 days so that users can inspect what was aggregated and build their Merkle proofs.
