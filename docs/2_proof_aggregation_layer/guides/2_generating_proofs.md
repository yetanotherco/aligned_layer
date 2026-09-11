# Generating Proofs For Aligned

Aligned's Proof Aggregation Service currently accepts **SP1 compressed proofs**. This guide shows how to produce the files the [Aggregation Mode CLI](3_cli.md) and [SDK](4_sdk.md) expect.

## Dependencies

This guide assumes that:

- The SP1 prover is installed ([instructions](https://docs.succinct.xyz/docs/sp1/getting-started/install))
- You have an SP1 project to generate proofs from ([instructions](https://docs.succinct.xyz/docs/sp1/getting-started/quickstart))
- The Aggregation Mode CLI is installed ([instructions](3_cli.md#installation))

## Generating a compressed proof

{% hint style="warning" %}
Only **compressed** SP1 proofs can be aggregated. In your proving script, the proof must be built with `.compressed()` — a default or Groth16/Plonk proof will be rejected by the aggregator.
{% endhint %}

In your SP1 script, generate the proof and write out the four files Aligned works with:

```rust
use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Stdin};

const ELF: &[u8] = include_elf!("your-program");

fn main() {
    let mut stdin = SP1Stdin::new();
    // ...write your program inputs into `stdin`...

    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(ELF);

    // `.compressed()` is required — Aligned aggregates compressed proofs only.
    let proof = client.prove(&pk, &stdin).compressed().run().unwrap();
    client.verify(&proof, &vk).expect("verification failed");

    // 1. The proof itself — passed to `--proof` when submitting.
    proof.save("./my_proof.proof").expect("saving proof failed");

    // 2. The serialized verifying key — passed to `--vk` when submitting.
    std::fs::write("./my_vk.bin", bincode::serialize(&vk).unwrap()).unwrap();

    // 3. The verifying key hash (32 bytes) — passed to `--vk-hash` when verifying on-chain.
    std::fs::write("./my_vk_hash.bin", vk.hash_bytes()).unwrap();

    // 4. The public values — passed to `--public-inputs` when verifying on-chain.
    std::fs::write("./my_public_inputs.bin", proof.public_values.clone()).unwrap();
}
```

Then run it from your script directory:

```bash
cargo run --release
```

A complete working example lives at `scripts/test_files/sp1/fibonacci_proof_generator/`, and the files it produces are checked into `scripts/test_files/sp1/`.

### Which file goes where

| File | Produced by | Used for |
|---|---|---|
| `my_proof.proof` | `proof.save(...)` | `agg_mode_cli submit sp1 --proof` |
| `my_vk.bin` | `bincode::serialize(&vk)` | `agg_mode_cli submit sp1 --vk` |
| `my_vk_hash.bin` | `vk.hash_bytes()` | `agg_mode_cli verify-on-chain --vk-hash` |
| `my_public_inputs.bin` | `proof.public_values` | `agg_mode_cli verify-on-chain --public-inputs` |

## Submitting the proof to Aligned

Fund your submission quota first, then submit:

```bash
agg_mode_cli deposit \
  --keystore-path <KEYSTORE_PATH> \
  --network hoodi \
  --rpc-url https://ethereum-hoodi-rpc.publicnode.com

agg_mode_cli submit sp1 \
  --proof ./my_proof.proof \
  --vk ./my_vk.bin \
  --keystore-path <KEYSTORE_PATH> \
  --network hoodi
```

The command returns a task ID. Your proof is aggregated with others and settled on Ethereum within the aggregation window (24 hours by default).

## Checking verification

Once the aggregated proof has been settled, check that yours was included:

```bash
agg_mode_cli verify-on-chain \
  --network hoodi \
  --rpc-url https://ethereum-hoodi-rpc.publicnode.com \
  --beacon-url https://ethereum-hoodi-beacon-api.publicnode.com \
  --proving-system SP1 \
  --vk-hash ./my_vk_hash.bin \
  --public-inputs ./my_public_inputs.bin
```

See the [Aggregation Mode CLI reference](3_cli.md) for every option, and the [integration example](5_integration_example.md) for a full application walkthrough.
