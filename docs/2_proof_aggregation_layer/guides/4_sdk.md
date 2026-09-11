# Aggregation Mode SDK

The Aggregation Mode SDK lets you submit proofs to Aligned's Proof Aggregation Service and check their verification on-chain from Rust, without going through the CLI.

## Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
agg_mode_sdk = { git = "https://github.com/yetanotherco/aligned_layer", branch = "testnet" }

# Needed to build the arguments the SDK takes:
alloy = { version = "1.1.1", features = ["signer-keystore"] }
sp1-sdk = "5.0.0"
bincode = "1.3.3"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

{% hint style="info" %}
The SDK is not yet part of a tagged release, so the dependency tracks the `testnet` branch. Pin it to a tag once one is published.
{% endhint %}

## Hello World

Submitting an SP1 proof takes a signer and the gateway provider:

```rust
use agg_mode_sdk::{gateway::provider::AggregationModeGatewayProvider, types::Network};
use alloy::signers::local::PrivateKeySigner;
use sp1_sdk::{SP1ProofWithPublicValues, SP1VerifyingKey};

#[tokio::main]
async fn main() {
    let proof: SP1ProofWithPublicValues =
        bincode::deserialize(&std::fs::read("./my_proof.proof").unwrap()).unwrap();
    let vk: SP1VerifyingKey =
        bincode::deserialize(&std::fs::read("./my_vk.bin").unwrap()).unwrap();

    let signer: PrivateKeySigner = "<YOUR_PRIVATE_KEY>".parse().unwrap();
    let provider = AggregationModeGatewayProvider::new_with_signer(Network::Hoodi, signer)
        .expect("failed to initialize gateway client");

    let response = provider
        .submit_sp1_proof(&proof, &vk)
        .await
        .expect("failed to submit proof");

    println!("Proof submitted. Task ID: {}", response.data.task_id);
}
```

Your account needs quota before submitting — see the [`deposit` command](3_cli.md#deposit) of the Aggregation Mode CLI.

Once the proof has been aggregated and settled on L1, check its inclusion with
[`check_proof_verification`](#check_proof_verification).

## API Reference

### `AggregationModeGatewayProvider`

The gateway provider handles communication with the Aggregation Mode Gateway for submitting proofs and querying proof status.

---

### `AggregationModeGatewayProvider::new`

Creates a new gateway provider without a signer. Useful for read-only operations like checking nonces or receipts.

```rust
pub fn new(network: Network) -> Result<Self, GatewayError>
```

#### Arguments

- `network` - The network to connect to (`Devnet | Hoodi | Mainnet`)

#### Returns

- `Result<AggregationModeGatewayProvider, GatewayError>` - A gateway provider instance or an error.

---

### `AggregationModeGatewayProvider::new_with_signer`

Creates a new gateway provider with a signer. Required for submitting proofs.

```rust
pub fn new_with_signer(network: Network, signer: S) -> Result<Self, GatewayError>
```

#### Arguments

- `network` - The network to connect to (`Devnet | Hoodi | Mainnet`)
- `signer` - A signer implementing the `Signer` trait (e.g., `PrivateKeySigner`)

#### Returns

- `Result<AggregationModeGatewayProvider, GatewayError>` - A gateway provider instance or an error.

---

### `get_nonce_for`

Retrieves the current nonce for a given address.

```rust
pub async fn get_nonce_for(
    &self,
    address: String,
) -> Result<GatewayResponse<NonceResponse>, GatewayError>
```

#### Arguments

- `address` - The Ethereum address to query the nonce for.

#### Returns

- `Result<GatewayResponse<NonceResponse>, GatewayError>` - The nonce response or an error.

---

### `get_receipts_for`

Retrieves proof submission receipts for a given address.

```rust
pub async fn get_receipts_for(
    &self,
    address: String,
    nonce: Option<u64>,
) -> Result<GatewayResponse<ReceiptsResponse>, GatewayError>
```

#### Arguments

- `address` - The Ethereum address to query receipts for.
- `nonce` - Optional nonce to filter receipts by a specific submission.

#### Returns

- `Result<GatewayResponse<ReceiptsResponse>, GatewayError>` - The receipts response or an error.

---

### `submit_sp1_proof`

Submits an SP1 proof to the Aggregation Mode Gateway.

```rust
pub async fn submit_sp1_proof(
    &self,
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Result<GatewayResponse<SubmitProofResponse>, GatewayError>
```

#### Arguments

- `proof` - The SP1 proof with public values.
- `vk` - The SP1 verifying key.

#### Returns

- `Result<GatewayResponse<SubmitProofResponse>, GatewayError>` - The submission response containing a task ID, or an error.

#### Errors

- `SignerNotConfigured` - If the provider was created without a signer.
- `ProofSerialization` - If there is an error serializing the proof or verifying key.
- `MessageSignature` - If there is an error signing the submission message.
- `Request` - If there is a network error communicating with the gateway.
- `Api` - If the gateway returns an error response.

---

### `ProofAggregationServiceProvider`

The blockchain provider handles verification of proofs on-chain by querying the AlignedProofAggregationService contract.

---

### `ProofAggregationServiceProvider::new`

Creates a new blockchain provider for verifying proofs on-chain.

```rust
pub fn new(network: Network, rpc_url: String, beacon_client_url: String) -> Self
```

#### Arguments

- `network` - The network to connect to (`Devnet | Hoodi | Mainnet`)
- `rpc_url` - The Ethereum RPC provider URL.
- `beacon_client_url` - The Beacon chain client URL.

#### Returns

- `ProofAggregationServiceProvider` - A blockchain provider instance.

---

### `check_proof_verification`

Checks if a proof has been verified on-chain.

```rust
pub async fn check_proof_verification(
    &self,
    from_block: Option<u64>,
    verification_data: AggregationModeVerificationData,
) -> Result<ProofStatus, ProofVerificationAggModeError>
```

#### Arguments

- `from_block` - Optional block number to start searching from. Defaults to current block minus 7500 blocks (~25 hours).
- `verification_data` - The verification data containing the proving system type, verifying key hash, and public inputs.

#### Returns

- `Result<ProofStatus, ProofVerificationAggModeError>` - The proof status or an error.

#### Proof Status

- `Verified { merkle_path, merkle_root }` - The proof was found and verified in an aggregated batch.
- `Invalid` - The proof was found but Merkle root verification failed.
- `NotFound` - The proof was not found in any aggregated batch.

#### Errors

- `EthereumProviderError` - If there is an error communicating with the Ethereum RPC.
- `BeaconClient` - If there is an error communicating with the Beacon chain.
- `EventDecoding` - If there is an error decoding the on-chain event data.

---

## Types

### `Network`

```rust
pub enum Network {
    Devnet,   // Chain ID: 31337
    Hoodi,    // Chain ID: 560048
    Mainnet,  // Chain ID: 1
}
```

### `AggregationModeVerificationData`

```rust
pub enum AggregationModeVerificationData {
    SP1 {
        vk: [u8; 32],
        public_inputs: Vec<u8>,
    },
    Risc0 {
        image_id: [u8; 32],
        public_inputs: Vec<u8>,
    },
}
```

{% hint style="info" %}
Proof submission is currently SP1-only: the gateway exposes `submit_sp1_proof` and no Risc0 equivalent.
{% endhint %}

### `GatewayError`

```rust
pub enum GatewayError {
    Request(String),
    Api { status: u16, message: String },
    SignerNotConfigured,
    ProofSerialization(String),
    MessageSignature(String),
}
```

### `ProofStatus`

```rust
pub enum ProofStatus {
    Verified {
        merkle_path: Vec<[u8; 32]>,
        merkle_root: [u8; 32],
    },
    Invalid,
    NotFound,
}
```
