# Aligned Verification SDK

The Aligned Verification SDK facilitates the submission and verification of proofs through the Aligned batcher and checks the inclusion of these verified proofs on-chain. This README provides an overview of the SDK, its installation, usage, and API details.

## Table of Contents
- [Aligned Verification SDK](#aligned-verification-sdk)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [API Reference](#api-reference)

## Installation

To use this SDK in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aligned-sdk = { git = "https://github.com/yetanotherco/aligned_layer" }
```

## API Reference

### submit

Submits the proofs to the batcher to be verified and returns a vector of aligned verification data.

#### Arguments

- `batcher_addr` - The address of the batcher to which the proof will be submitted.
- `verification_data` - The verification data for the proof.
- `keystore_path` - The path to the keystore file used for payment.

#### Returns

- `Result<Option<Vec<AlignedVerificationData>>, SubmitError>` - A vector of aligned verification data or an error.

#### Errors

- `MissingParameter` if the verification data vector is empty.
- `SerdeError` if there is an error serializing the verification data.
- `ConnectionError` if there is an error sending the message to the websocket.

### verify_proof_onchain

Checks if the proof has been verified with Aligned and is included in the batch on-chain.

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

#### Arguments

- `content` - A byte slice of the verification key.

#### Returns

- `[u8; 32]` - A 32-byte array representing the keccak256 hash of the verification key.
