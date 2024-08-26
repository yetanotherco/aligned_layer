# Aligned SDK

The Aligned SDK aims to help developers interact with Aligned in a simple way.
Using the Aligned SDK, you can do things like submitting and verifying proofs through the Aligned Batcher, as well as checking the inclusion of the verified proofs on-chain.
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

## Hello World

To get the SDK up and running in your project, you must first import it

```rust
use aligned_sdk::core::types::{AlignedVerificationData, Chain, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::{submit_and_wait, get_next_nonce};
```

And then you can do a simple call of, for example, `get_next_nonce`
```rust
fn main() {
    // ...
let nonce = get_next_nonce(&rpc_url, wallet.address(), BATCHER_PAYMENTS_ADDRESS).await
    .expect("Failed to get next nonce");
    /// ...
}
```

Or you can make a more complex call, to submit a proof: 

(code extract from [ZKQuiz](../1_introduction/2_zkquiz.md))

```rust
fn main() {
    /// ...
    match submit_and_wait(
        BATCHER_URL,
        &rpc_url,
        Chain::Holesky,
        &verification_data,
        wallet.clone(),
        nonce
    )
    .await
    {
        Ok(maybe_aligned_verification_data) => match maybe_aligned_verification_data {
            Some(aligned_verification_data) => {
                println!(
                    "Proof submitted and verified successfully on batch {}, claiming prize...",
                    hex::encode(aligned_verification_data.batch_merkle_root)
                );

                if let Err(e) = verify_batch_inclusion(
                    aligned_verification_data.clone(),
                    signer.clone(),
                    args.verifier_contract_address,
                )
                .await
                {
                    println!("Failed to claim prize: {:?}", e);
                }
            }
            None => {
                println!("Proof submission failed. No verification data");
            }
        },
        Err(e) => {
            println!("Proof verification failed: {:?}", e);
        }
    }
    /// ...
}
```
