# Multiple OOB in aligned-batcher halo2_ipa verification 

**Author(s):** Nabih Benazzouz [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

## **Executive Summary**

While auditing and fuzzing some verification functions, we discovered that the `verify_halo2_ipa` function manipulates arrays without performing size verification. For instance, it uses the code snippet `let cs_len_buf: [u8; 4] = verification_key[..4]` without checking the size of the array.

## Vulnerability Details

- **Severity:** High

- **Affected Component:** batcher/aligned-batcher

- **Permalink:** [GitHub Source](https://github.com/yetanotherco/aligned_layer/blob/81e1ae4ff6cdddca28808ac071d4b1ae72793346/batcher/aligned-batcher/src/halo2/ipa/mod.rs#L23)

## Environment

- **Distro Version:** 6.9.9-1-MANJARO
- **Additional Environment Details:** rustc 1.79.0-nightly (1cec373f6 2024-04-16)

## Steps to Reproduce

To demonstrate the issue, we provide a Proof of Concept (POC) to show that the bug can be triggered.

1. Start the batcher:

    ```sh
    make anvil_start_with_block_time
    make batcher_start
    ```

2. Trigger the bug with this command:

    ```sh
    ./batcher/target/release/aligned submit --proving_system Halo2IPA --proof ./scripts/test_files/sp1/sp1_fibonacci.proof --public_input scripts/test_files/sp1/sp1_fibonacci.elf --vk scripts/test_files/sp1/sp1_fibonacci.elf
    ```

## Root Cause Analysis

The `verify_halo2_ipa` function is vulnerable because it does not verify the length of any field or arguments before manipulating arrays or slices. This oversight allows for potential out-of-bounds errors.

Example:

```rust
...
let cs_len_buf: [u8; 4] = verification_key[..4];
...
cs_buffer[..cs_len].clone_from_slice(&verification_key[cs_offset..(cs_offset + cs_len)]);
...
vk_buffer[..vk_len].clone_from_slice(&verification_key[vk_offset..(vk_offset + vk_len)]);
...
and more

```

Here is a small harness that can help you trigger all the crashes in this function.

```rust
extern crate honggfuzz;

use aligned_batcher::halo2::ipa::verify_halo2_ipa;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct InputData {
    proof: Vec<u8>,
    public_input: Vec<u8>,
    verification_key: Vec<u8>,
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            if let Ok(input) = InputData::arbitrary(&mut arbitrary::Unstructured::new(data)) {
                let _ =
                    verify_halo2_ipa(&input.proof, &input.public_input, &input.verification_key);
            }
        });
    }
}

```

Create the file under `batcher/aligned-batcher/src/fuzzing/verify_halo2_ipa_fuzz.rs`

Update the `cargo.toml` with:

```sh
[dependencies]
...
honggfuzz = "*"
arbitrary = {version = "1.0", features = ["derive"]}
...

[[bin]]
name ="verify_halo2_ipa_fuzz"
path = "src/fuzzing/verify_halo2_ipa_fuzz.rs"
```

Run it with:

```sh
cargo hfuzz run verify_halo2_ipa_fuzz

```

## Detailed Behavior

```sh
...
...
[2024-08-01T11:55:40Z DEBUG hyper::client::pool] pooling idle connection for ("http", localhost:8545)
thread 'tokio-runtime-worker' panicked at aligned-batcher/src/halo2/ipa/mod.rs:31:14:
range end index 1179403647 out of range for slice of length 2048
note: run with `RUST_BACKTRACE=1` environment variable to display a back
...
...

```

## Recommendations
Implement strict input validation to ensure all input fields are of expected size before processing.
