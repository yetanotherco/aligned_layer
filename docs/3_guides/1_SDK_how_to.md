# Aligned SDK

The Aligned SDK aims to help developers interact with Aligned in a simple way.
Using the Aligned SDK, you can do things like submitting and verifying proofs through the Aligned Batcher, as well as checking the inclusion of the verified proofs on-chain.
This guide provides an overview of the SDK, its installation, usage, and API details.

You can check the list of supported verifiers [here](../2_architecture/0_supported_verifiers.md).

## Installation

To use this SDK in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aligned-sdk = { git = "https://github.com/yetanotherco/aligned_layer", tag="v0.10.1" }
```

To find the latest release tag go to [releases](https://github.com/yetanotherco/aligned_layer/releases) and copy the
version of the release that has the `latest` badge.

## Hello World

To get the SDK up and running in your project, you must first import it

```rust
use aligned_sdk::core::types::{PriceEstimate, AlignedVerificationData, Network, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::{estimate_fee, submit_and_wait, get_next_nonce};
```

And then you can do a simple call of, for example, `get_next_nonce`
```rust
const NETWORK: Network = Network::Holesky;

fn main() {
    let rpc_url = args.rpc_url.clone();
    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");
    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);

    // Call to SDK:
    let nonce = get_next_nonce(&rpc_url, wallet.address(), NETWORK).await
    .expect("Failed to get next nonce");
}
```

Or you can make a more complex call to submit a proof:

(code extract from [ZKQuiz example](../3_guides/2_build_your_first_aligned_application.md#app))

```rust
const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";

fn main() {
    let rpc_url = args.rpc_url.clone();
    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(ELF.to_vec()),
        verification_key: None,
        pub_input: None,
    };
    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");
    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);
    let max_fee = estimate_fee(&rpc_url, PriceEstimate::Instant)
        .await
        .expect("failed to fetch gas price from the blockchain");

    // Call to SDK:
    match submit_and_wait_verification(
        BATCHER_URL,
        &rpc_url,
        Network::Holesky,
        &verification_data,
        max_fee,
        wallet.clone(),
        nonce
    )
    .await
    {
        Ok(maybe_aligned_verification_data) => match maybe_aligned_verification_data {
            Some(aligned_verification_data) => {
                println!(
                    "Proof submitted and verified successfully on batch {}",
                    hex::encode(aligned_verification_data.batch_merkle_root)
                );

            }
            None => {
                println!("Proof submission failed. No verification data");
            }
        },
        Err(e) => {
            println!("Proof verification failed: {:?}", e);
        }
    }
}
```

In the [next section,](./1.2_SDK_api_reference.md) we will dive deeper into what does each argument mean, and what other functions does Aligned SDK contain.
