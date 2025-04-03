#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use sha3::Keccak256;
use sp1_aggregation_program::{Input, ProofInput};

fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(hash_a);
    hasher.update(hash_b);
    hasher.finalize().into()
}

/// Computes the merkle root for the given proofs using the vk
fn compute_merkle_root(proofs: &[ProofInput]) -> [u8; 32] {
    let mut leaves: Vec<[u8; 32]> = proofs
        .chunks(2)
        .map(|chunk| match chunk {
            [a, b] => combine_hashes(&a.hash(), &b.hash()),
            [a] => combine_hashes(&a.hash(), &a.hash()),
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

pub fn main() {
    let input = sp1_zkvm::io::read::<Input>();

    // Verify the proofs.
    for proof in input.proofs.iter() {
        match proof {
            ProofInput::SP1Compressed(proof) => {
                let vkey = proof.vk;
                let public_values = &proof.public_inputs;
                let public_values_digest = Sha256::digest(public_values);
                sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
            }
        }
    }

    let merkle_root = compute_merkle_root(&input.proofs);

    assert_eq!(merkle_root, input.merkle_root);

    sp1_zkvm::io::commit_slice(&merkle_root);
}
