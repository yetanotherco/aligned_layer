use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

#[derive(Serialize, Deserialize)]
pub struct Risc0ImageIdAndPubInputs {
    pub image_id: [u8; 32],
    pub public_inputs: Vec<u8>,
}

impl Risc0ImageIdAndPubInputs {
    pub fn commitment(&self) -> [u8; 32] {
        let mut hasher = Keccak::v256();
        for &word in &self.image_id {
            hasher.update(&word.to_be_bytes());
        }
        hasher.update(&self.public_inputs);

        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs_image_id_and_pub_inputs: Vec<Risc0ImageIdAndPubInputs>,
}

fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(hash_a);
    hasher.update(hash_b);

    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);
    hash
}

/// Computes the merkle root for the given proofs
pub fn compute_merkle_root(mut leaves: Vec<[u8; 32]>) -> [u8; 32] {
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
