use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Serialize, Deserialize)]
pub struct SP1VkAndPubInputs {
    pub vk: [u32; 8],
    pub public_inputs: Vec<u8>,
}

impl SP1VkAndPubInputs {
    pub fn commitment(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk {
            hasher.update(word.to_be_bytes());
        }
        hasher.update(&self.public_inputs);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub proofs_vk_and_pub_inputs: Vec<SP1VkAndPubInputs>,
}

fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(hash_a);
    hasher.update(hash_b);
    hasher.finalize().into()
}

/// Computes the merkle root for the given proofs
pub fn compute_merkle_root(mut leaves_hash: Vec<[u8; 32]>) -> [u8; 32] {
    while leaves_hash.len() > 1 {
        leaves_hash = leaves_hash
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => combine_hashes(&a, &b),
                [a] => combine_hashes(&a, &a),
                _ => panic!("Unexpected chunk size in leaves"),
            })
            .collect()
    }

    leaves_hash[0]
}
