use crate::aggregators::AlignedProof;
use sha3::{Digest, Keccak256};

pub fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(hash_a);
    hasher.update(hash_b);
    hasher.finalize().into()
}

/// Returns (merkle_root, leaves)
pub fn compute_proofs_merkle_root(proofs: &[AlignedProof]) -> ([u8; 32], Vec<[u8; 32]>) {
    let leaves: Vec<[u8; 32]> = proofs.iter().map(|proof| proof.hash()).collect();

    let mut root = leaves.clone();

    while root.len() > 1 {
        root = root
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => combine_hashes(&a, &b),
                [a] => combine_hashes(&a, &a),
                _ => panic!("Unexpected chunk size in leaves"),
            })
            .collect()
    }

    (root[0], leaves)
}
