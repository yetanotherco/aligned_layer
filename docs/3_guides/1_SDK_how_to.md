# Aligned SDK

The Aligned SDK aims to help developers interact with Aligned in a simple way.
Using the Aligned SDK, you can do things like submitting and verifying proofs through the Aligned Batcher, as well as checking the inclusion of the verified proofs on-chain.
This guide provides an overview of the SDK, its installation, usage, and API details.

You can check the list of supported verifiers [here](../2_architecture/0_supported_verifiers.md).

## Installation

To use this SDK in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
aligned-sdk = { git = "https://github.com/yetanotherco/aligned_layer", tag="v0.6.0" }
```

To find the latest release tag go to [releases](https://github.com/yetanotherco/aligned_layer/releases) and copy the
version of the release that has the `latest` badge.

## Hello World

To get the SDK up and running in your project, you must first import it

```rust
use aligned_sdk::sdk::get_next_nonce;
use std::path::PathBuf;
use ethers::signers::LocalWallet;
```

And then you can do a simple call of, for example, `get_next_nonce`
```rust
const BATCHER_PAYMENTS_ADDRESS: &str = "0x815aeCA64a974297942D2Bbf034ABEe22a38A003";
const RPC_URL: &str = "https://ethereum-holesky-rpc.publicnode.com";

fn main() {
    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");
    let keystore_path = Some(PathBuf::from("./keystore.json"));
    let wallet = LocalWallet::decrypt_keystore(keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);

    // Call to SDK:
    let nonce = get_next_nonce(RPC_URL, wallet.address(), BATCHER_PAYMENTS_ADDRESS).await
    .expect("Failed to get next nonce");
}
```

Or you can make a more complex call to submit a proof:

(code extract from [ZKQuiz](../1_introduction/2_zkquiz.md))

```rust
const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const BATCHER_PAYMENTS_ADDRESS: &str = "0x815aeCA64a974297942D2Bbf034ABEe22a38A003";
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let keystore_password = rpassword::prompt_password("Enter keystore password: ")
        .expect("Failed to read keystore password");

    let wallet = LocalWallet::decrypt_keystore(args.keystore_path, &keystore_password)
        .expect("Failed to decrypt keystore")
        .with_chain_id(17000u64);

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        proof_generator_addr: wallet.address(),
        vm_program_code: Some(ELF.to_vec()),
        verification_key: None,
        pub_input: None,
    };

    // Call to SDK:
    let nonce = get_next_nonce(&rpc_url, wallet.address(), BATCHER_PAYMENTS_ADDRESS)
        .await
        .expect("Failed to get next nonce");

    match submit_and_wait_verification(
        BATCHER_URL,
        &rpc_url,
        Chain::Holesky,
        &verification_data,
        wallet.clone(),
        nonce,
    )
    .await
    {
        Ok(aligned_verification_data) => {
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
        Err(e) => {
            println!("Proof verification failed: {:?}", e);
        }
    }
}
```

In the [next section,](./1.2_SDK_api_reference.md) we will dive deeper into what does each argument mean, and what other functions does Aligned SDK contain.
