// Generates the Merkle proof fixture used by the Solidity test suite.
//
// It builds an OpenZeppelin StandardMerkleTree with the SAME library
// (merkle-tree-rs) and SAME leaf encoding (["address","uint256","uint256"]) as
// the production proof generator, then writes the root and every leaf's proof to
// a JSON file. The Solidity tests load that file and verify the contract accepts
// the proofs, so the on-chain verifier is tested against real generator output
// rather than a Solidity reimplementation.
//
// Usage: cargo run -- <output_path>

use merkle_tree_rs::standard::{LeafType, StandardMerkleTree};
use std::{env, fs};

// (address, amount (wei), valid_from). The first three leaves share one account
// with distinct (amount, valid_from) pairs so the batch-claim path can be tested.
const LEAVES: &[(&str, &str, &str)] = &[
    ("0x00000000000000000000000000000000000000a1", "100000000000000000000", "0"),
    ("0x00000000000000000000000000000000000000a1", "200000000000000000000", "1000"),
    ("0x00000000000000000000000000000000000000a1", "300000000000000000000", "2000"),
    ("0x00000000000000000000000000000000000000b2", "400000000000000000000", "0"),
    ("0x00000000000000000000000000000000000000c3", "500000000000000000000", "0"),
    ("0x00000000000000000000000000000000000000d4", "600000000000000000000", "0"),
    ("0x00000000000000000000000000000000000000e5", "700000000000000000000", "0"),
    ("0x00000000000000000000000000000000000000f6", "800000000000000000000", "0"),
];

fn main() {
    let out_path = env::args()
        .nth(1)
        .expect("usage: merkle_fixture_gen <output_path>");

    let values: Vec<Vec<String>> = LEAVES
        .iter()
        .map(|(addr, amount, valid_from)| {
            vec![addr.to_string(), amount.to_string(), valid_from.to_string()]
        })
        .collect();

    let tree = StandardMerkleTree::of(
        &values,
        &[
            "address".to_string(),
            "uint256".to_string(),
            "uint256".to_string(),
        ],
    )
    .expect("failed to build merkle tree");

    let leaves_json: Vec<serde_json::Value> = LEAVES
        .iter()
        .enumerate()
        .map(|(i, (addr, amount, valid_from))| {
            let proof = tree
                .get_proof(LeafType::Number(i))
                .expect("failed to get proof");
            serde_json::json!({
                "account": addr,
                "amount": amount,
                "validFrom": valid_from,
                "proof": proof,
            })
        })
        .collect();

    let fixture = serde_json::json!({
        "root": tree.root(),
        "count": LEAVES.len(),
        "leaves": leaves_json,
    });

    fs::write(&out_path, serde_json::to_string_pretty(&fixture).unwrap())
        .expect("failed to write fixture file");

    println!("wrote {} leaves to {}", LEAVES.len(), out_path);
    println!("root: {}", tree.root());
}
