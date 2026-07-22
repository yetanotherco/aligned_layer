# Merkle proof test fixtures

`proofs.json` holds a Merkle root and the proof for every leaf of a small test
tree. It is consumed by `test/ClaimableAirdrop.t.sol` so the Solidity tests
verify the contract against **real proof-generator output** instead of a Merkle
implementation reimplemented in Solidity.

The fixture is produced by `generator/`, a small Rust binary that uses the same
`merkle-tree-rs` revision and the same leaf encoding (`["address","uint256","uint256"]`)
as the production proof generator in
`aligned_airdrop_web/merkle_proof_generator`. This guarantees the proofs match
what real claimants receive.

## Regenerating

From the `claim_contracts/` directory:

```sh
cargo run --release \
  --manifest-path test/fixtures/generator/Cargo.toml \
  -- test/fixtures/proofs.json
```

Edit the `LEAVES` table in `generator/src/main.rs` to change the test data, then
regenerate. Commit the updated `proofs.json`; the Rust toolchain is only needed
to regenerate it, never to run the Solidity tests.
