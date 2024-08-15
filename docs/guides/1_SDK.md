# Aligned SDK

The Aligned SDK aims to help developers interact with Aligned in a simple way.
Some of its functionalities include submitting and verifying proofs through the Aligned Batcher, as well as checking the
inclusion of the verified proofs on-chain.
This guide provides an overview of the SDK, its installation, usage, and API details.

You can check the list of supported verifiers [here](../architecture/0_supported_verifiers.md).

## Installation

To use this SDK in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aligned-sdk = { git = "https://github.com/yetanotherco/aligned_layer", tag="v0.4.0" }
```

To find the latest release tag go to [releases](https://github.com/yetanotherco/aligned_layer/releases) and copy the
version of the release that has the `latest` badge.

## API Reference

### `submit`

Submits a proof to the batcher to be verified and returns an aligned verification data struct.

```rust
pub async fn submit(
    batcher_url: &str,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError>
```

#### Arguments

- `batcher_url` - The url of the batcher to which the proof will be submitted.
- `verification_data` - The verification data for the proof.
- `wallet` - The wallet used to sign the proof.
- `nonce` - The nonce of the submitter address. See `get_next_nonce`.

#### Returns

- `Result<Option<AlignedVerificationData>>, SubmitError>` - An aligned verification data or an error.

#### Errors

- `MissingRequiredParameter` if the verification data vector is empty.
- `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
- `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
- `SerializationError` if there is an error deserializing the message sent from the batcher.
- `WebSocketConnectionError` if there is an error connecting to the batcher.
- `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
- `InvalidSignature` if the signature is invalid.
- `InvalidNonce` if the nonce is invalid.
- `InvalidProof` if the proof is invalid.
- `ProofTooLarge` if the proof is too large.
- `InsufficientBalance` if the sender balance is not enough or unlocked
- `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
- `GenericError` if the error doesn't match any of the previous ones.

### `submit_multiple`

Submits multiple proofs to the batcher to be verified and returns an aligned verification data array.

```rust
pub async fn submit_multiple(
    batcher_url: &str,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError>
```

#### Arguments

- `batcher_url` - The url of the batcher to which the proof will be submitted.
- `verification_data` - A verification data array.
- `wallet` - The wallet used to sign the proof.
- `nonce` - The nonce of the submitter address. See `get_next_nonce`.

#### Returns

- `Result<Option<Vec<AlignedVerificationData>>>, SubmitError>` - An aligned verification data array or an error.

#### Errors

- `MissingRequiredParameter` if the verification data vector is empty.
- `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
- `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
- `SerializationError` if there is an error deserializing the message sent from the batcher.
- `WebSocketConnectionError` if there is an error connecting to the batcher.
- `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
- `InvalidSignature` if the signature is invalid.
- `InvalidNonce` if the nonce is invalid.
- `InvalidProof` if the proof is invalid.
- `ProofTooLarge` if the proof is too large.
- `InsufficientBalance` if the sender balance is not enough or unlocked
- `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
- `GenericError` if the error doesn't match any of the previous ones.

### `submit_and_wait_verification`

Submits a proof to the batcher to be verified, waits for the verification on ethereum and returns an aligned
verification data struct.

```rust
pub async fn submit_and_wait_verification(
    batcher_url: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError>
```

#### Arguments

- `batcher_url` - The url of the batcher to which the proof will be submitted.
- `eth_rpc_url` - The URL of the Ethereum RPC node.
- `chain` - The chain on which the verification will be done.
- `verification_data` - The verification data for the proof.
- `wallet` - The wallet used to sign the proof.
- `nonce` - The nonce of the submitter address. See `get_next_nonce`.

#### Returns

- `Result<Option<AlignedVerificationData>>, SubmitError>` - An aligned verification data or an error.

#### Errors

- `MissingRequiredParameter` if the verification data vector is empty.
- `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
- `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
- `SerializationError` if there is an error deserializing the message sent from the batcher.
- `WebSocketConnectionError` if there is an error connecting to the batcher.
- `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
- `EthereumProviderError` if there is an error in the connection with the RPC provider.
- `HexDecodingError` if there is an error decoding the Aligned service manager contract address.
- `BatchVerificationTimeout` if there is a timeout waiting for the batch verification.
- `InvalidSignature` if the signature is invalid.
- `InvalidNonce` if the nonce is invalid.
- `InvalidProof` if the proof is invalid.
- `ProofTooLarge` if the proof is too large.
- `InsufficientBalance` if the sender balance is not enough or unlocked
- `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
- `GenericError` if the error doesn't match any of the previous ones.

### `submit_multiple_and_wait_verification`

Submits multiple proofs to the batcher to be verified, waits for the verification on Ethereum and returns an aligned
verification data array.

```rust
pub async fn submit_multiple_and_wait_verification(
    batcher_url: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
    nonce: U256,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError>
```

#### Arguments

- `batcher_url` - The url of the batcher to which the proof will be submitted.
- `eth_rpc_url` - The URL of the Ethereum RPC node.
- `chain` - The chain on which the verification will be done.
- `verification_data` - A verification data array.
- `wallet` - The wallet used to sign the proof.
- `nonce` - The nonce of the submitter address. See `get_next_nonce`.

#### Returns

- `Result<Option<Vec<AlignedVerificationData>>>, SubmitError>` - An aligned verification data array or an error.

#### Errors

- `MissingRequiredParameter` if the verification data vector is empty.
- `ProtocolVersionMismatch` if the version of the SDK is lower than the expected one.
- `UnexpectedBatcherResponse` if the batcher doesn't respond with the expected message.
- `SerializationError` if there is an error deserializing the message sent from the batcher.
- `WebSocketConnectionError` if there is an error connecting to the batcher.
- `WebSocketClosedUnexpectedlyError` if the connection with the batcher is closed unexpectedly.
- `EthereumProviderError` if there is an error in the connection with the RPC provider.
- `HexDecodingError` if there is an error decoding the Aligned service manager contract address.
- `BatchVerificationTimeout` if there is a timeout waiting for the batch verification.
- `InvalidSignature` if the signature is invalid.
- `InvalidNonce` if the nonce is invalid.
- `InvalidProof` if the proof is invalid.
- `ProofTooLarge` if the proof is too large.
- `InsufficientBalance` if the sender balance is not enough or unlocked
- `ProofQueueFlushed` if there is an error in the batcher and the proof queue is flushed.
- `GenericError` if the error doesn't match any of the previous ones.

### `is_proof_verified`

Checks if the proof has been verified with Aligned and is included in the batch on-chain.

```rust
pub async fn is_proof_verified(
    aligned_verification_data: AlignedVerificationData,
    chain: Chain,
    eth_rpc_url: &str,
) -> Result<bool, errors::VerificationError>
```

#### Arguments

- `aligned_verification_data` - The aligned verification data obtained when submitting the proofs.
- `chain` - The chain on which the verification will be done.
- `eth_rpc_url` - The URL of the Ethereum RPC node.

#### Returns

- `Result<bool, VerificationError>` - A boolean indicating whether the proof was verified on-chain and is included in
  the batch or an error.

#### Errors

- `EthereumProviderError` if there is an error in the connection with the RPC provider.
- `EthereumCallError` if there is an error in the Ethereum call.
- `HexDecodingError` if there is an error decoding the Aligned service manager contract address.

### `get_commitment`

Generates a keccak256 hash commitment of the verification key.

```rust
pub fn get_commitment(
    content: &[u8]
) -> [u8; 32]
```

#### Arguments

- `content` - A byte slice of the verification key.

#### Returns

- `[u8; 32]` - A 32-byte array representing the keccak256 hash of the verification key.

### `get_next_nonce`

Returns the nonce to use for a given address.

```rust
pub async fn get_next_nonce(
    eth_rpc_url: &str,
    submitter_addr: Address,
    payment_service_addr: &str,
) -> Result<U256, errors::NonceError>
```

#### Arguments

- `eth_rpc_url` - The URL of the Ethereum RPC node.
- `submitter_addr` - The address of the proof submitter for which the nonce will be retrieved.
- `payment_service_addr` - The address of the batcher payment service contract.

#### Returns

- `Result<U256, NonceError>` - The nonce to use or an error.

#### Errors

- `EthereumProviderError` if there is an error in the connection with the RPC provider.
- `EthereumCallError` if there is an error in the Ethereum call.
