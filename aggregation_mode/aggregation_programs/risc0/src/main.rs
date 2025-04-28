#![no_main]

use risc0_aggregation_program::{Input, Risc0ImageIdAndPubInputs};
use risc0_zkvm::guest::env;
use tiny_keccak::{Hasher, Keccak};

risc0_zkvm::guest::entry!(main);

fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(hash_a);
    hasher.update(hash_b);

    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    hash
}

/// Computes the merkle root for the given proofs
fn compute_merkle_root(proofs: &[Risc0ImageIdAndPubInputs]) -> [u8; 32] {
    let mut leaves: Vec<[u8; 32]> = proofs
        .chunks(2)
        .map(|chunk| match chunk {
            [a, b] => combine_hashes(&a.commitment(), &b.commitment()),
            [a] => combine_hashes(&a.commitment(), &a.commitment()),
            _ => panic!("Unexpected chunk leaves"),
        })
        .collect();

    while leaves.len() > 1 {
        leaves = leaves
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => combine_hashes(&a, &b),
                [a] => combine_hashes(&a, &a),
                _ => panic!("Unexpected chunk size in leaves"),
            })
            .collect()
    }

    leaves[0]
}

fn main() {
    let input = env::read::<Input>();

    for Risc0ImageIdAndPubInputs {
        image_id,
        public_inputs,
    } in &input.proofs_image_id_and_pub_inputs
    {
        env::verify(image_id.clone(), &public_inputs).expect("proof to be verified correctly");
    }

    let merkle_root = compute_merkle_root(&input.proofs_image_id_and_pub_inputs);

    env::commit_slice(&merkle_root);
}
