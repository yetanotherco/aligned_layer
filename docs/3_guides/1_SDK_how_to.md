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
