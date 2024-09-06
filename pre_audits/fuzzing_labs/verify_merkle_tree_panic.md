# `verify_merkle_tree_batch_ffi` can panic 

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we found that a malformed JSON input can
cause the `verify_merkle_tree_batch_ffi` to panic due to a subtract with overflow. This can be triggered by an attacker
to cause a denial of service.

### Vulnerability Details

- **Severity:** Medium
- **Affected Component:** `operator/merkle_tree/lib/src/lib.rs`


## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** rustc 1.81.0-nightly (24d2ac0b5 2024-07-15)

## Steps to Reproduce

To demonstrate the issue, we provide a Proof of Concept (POC) to show that the bug can be triggered.

1. Add this test to `operator/merkle_tree/lib/src/lib.rs`:
    ```rust
    #[test]
    fn test_panic_verify_merkle_tree_batch_ffi() {
        let path = "./test_files/panic-sp1-proof.json";

        let mut file = File::open(path).unwrap();

        let mut bytes_vec = Vec::new();
        file.read_to_end(&mut bytes_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode("5ba2f046e3c1072b96f55728a67d73b4e246a6c27960b0c52d7fafb77981bcb0").unwrap());

        let result = verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);
    }
    ```
2. Create a malformed JSON file `operator/merkle_tree/lib/src/test_files/panic-sp1-proof.json`:
   ```json
   [
     {
       "proving_system": "SP1",
       "proof": [0],
       "pub_input": null,
       "verification_key": null,
       "vm_program_code": null,
       "proof_generator_addr": "0x66f9664f97f2b50f62d13ea064982f936de76657"
     }
   ]
   ```
3. Run the test:
   ```sh
   cd operator/merkle_tree/lib
   cargo test
   ```

## Root Cause Analysis

The root cause is located in `lambdaworks-crypto-0.7.0\src\merkle_tree\utils.rs:63`.

```rust
...
// ! CAUTION !
// Make sure n=nodes.len()+1 is a power of two, and the last n/2 elements (leaves) are populated with hashes.
// This function takes no precautions for other cases.
pub fn build<B: IsMerkleTreeBackend>(nodes: &mut [B::Node], leaves_len: usize)
where
    B::Node: Clone,
{
    let mut level_begin_index = leaves_len - 1;
    ...
        level_end_index = level_begin_index - 1;
    ...
}
```

If the `leaves_len` is 1, the `level_begin_index` will be set to `0` which will cause the panic when trying to subtract 
1 from it since it's an unsigned integer.

## Detailed Behavior

```sh
...
...
attempt to subtract with overflow
thread 'tests::test_panic_verify_merkle_tree_batch_ffi' panicked at .cargo/registry/src/index.crates.io-6f17d22bba15001f/lambdaworks-crypto-0.7.0/src/merkle_tree/utils.rs:63:27:
...
...

```

## Recommendations

- Input Validation: Ensure that the `proof` field in the JSON input is properly validated before processing it.