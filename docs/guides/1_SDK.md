# Aligned SDK

The Aligned SDK aims to help developers interact with Aligned in a simple way.
Some of its functionalities include submitting and verify proofs through the Aligned batcher, as well as checks the inclusion of the verified proofs on-chain. This guide provides an overview of the SDK, its installation, usage, and API details.

You can check the list of supported verifiers [here](../architecture/0_supported_verifiers.md).

## Installation

To use this SDK in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aligned-sdk = { git = "https://github.com/yetanotherco/aligned_layer" }
```

## API Reference

### submit

Submits a proof to the batcher to be verified and returns an aligned verification data struct.

```rust
pub async fn submit(
    batcher_addr: &str,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError>
```

#### Arguments

- `batcher_addr` - The address of the batcher to which the proof will be submitted.
- `verification_data` - The verification data for the proof.
- `wallet` - The wallet used to sign the proof.

#### Returns

- `Result<Option<AlignedVerificationData>>, SubmitError>` - An aligned verification data or an error.

#### Errors

- `MissingParameter` if the verification data vector is empty.
- `SerdeError` if there is an error serializing the verification data.
- `ConnectionError` if there is an error sending the message to the websocket.

### submit_multiple

Submits multiple proofs to the batcher to be verified and returns an aligned verification data array.

```rust
pub async fn submit_multiple(
    batcher_addr: &str,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError>
```

#### Arguments

- `batcher_addr` - The address of the batcher to which the proof will be submitted.
- `verification_data` - A verification data array.
- `wallet` - The wallet used to sign the proof.

#### Returns

- `Result<Option<Vec<AlignedVerificationData>>>, SubmitError>` - An aligned verification data array or an error.

#### Errors

- `MissingParameter` if the verification data vector is empty.
- `SerdeError` if there is an error serializing the verification data.
- `ConnectionError` if there is an error sending the message to the websocket.

### submit_and_wait

Submits a proof to the batcher to be verified, waits for the verification on ethereum and returns an aligned verification data struct.

```rust
pub async fn submit_and_wait(
    batcher_addr: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &VerificationData,
    wallet: Wallet<SigningKey>,
) -> Result<Option<AlignedVerificationData>, errors::SubmitError>
```

#### Arguments

- `batcher_addr` - The address of the batcher to which the proof will be submitted.
- `eth_rpc_url` - The URL of the Ethereum RPC node.
- `chain` - The chain on which the verification will be done.
- `verification_data` - The verification data for the proof.
- `wallet` - The wallet used to sign the proof.

#### Returns

- `Result<Option<AlignedVerificationData>>, SubmitError>` - An aligned verification data or an error.

### submit_multiple_and_wait

Submits multiple proofs to the batcher to be verified, waits for the verification on Ethereum and returns an aligned verification data array.

```rust
pub async fn submit_multiple_and_wait(
    batcher_addr: &str,
    eth_rpc_url: &str,
    chain: Chain,
    verification_data: &[VerificationData],
    wallet: Wallet<SigningKey>,
) -> Result<Option<Vec<AlignedVerificationData>>, errors::SubmitError>
```

#### Arguments

- `batcher_addr` - The address of the batcher to which the proof will be submitted.
- `eth_rpc_url` - The URL of the Ethereum RPC node.
- `chain` - The chain on which the verification will be done.
- `verification_data` - A verification data array.
- `wallet` - The wallet used to sign the proof.

#### Returns

- `Result<Option<Vec<AlignedVerificationData>>>, SubmitError>` - An aligned verification data array or an error.

#### Errors

- `MissingParameter` if the verification data vector is empty.
- `SerdeError` if there is an error serializing the verification data.
- `ConnectionError` if there is an error sending the message to the websocket.

### verify_proof_onchain

Checks if the proof has been verified with Aligned and is included in the batch on-chain.

```rust
pub async fn verify_proof_onchain(
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

- `Result<bool, VerificationError>` - A boolean indicating whether the proof was verified on-chain and is included in the batch or an error.

#### Errors

- `EthError` if there is an error creating the rpc provider.
- `ParsingError` if there is an error parsing the address of the contract.
- `EthError` if there is an error verifying the proof on-chain.

### get_verification_key_commitment

Generates a keccak256 hash commitment of the verification key.

```rust
pub fn get_verification_key_commitment(
    content: &[u8]
) -> [u8; 32]
```

#### Arguments

- `content` - A byte slice of the verification key.

#### Returns

- `[u8; 32]` - A 32-byte array representing the keccak256 hash of the verification key.
