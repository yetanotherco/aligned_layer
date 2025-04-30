#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use sp1_aggregation_program::{compute_merkle_root, Input};

pub const LEAVES_AGG_PROGRAM_VK_HASH: [u32; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

pub fn main() {
    let input = sp1_zkvm::io::read::<Input>();

    let mut proofs_hash: Vec<[u8; 32]> = vec![];

    // Verify the proofs.
    for proof in input.proofs_vk_and_pub_inputs.iter() {
        let vkey = proof.vk;
        let public_values_digest = Sha256::digest(&proof.public_inputs);

        // Ensure the aggregated chunk originates from the L1 aggregation program.
        // This validation step guarantees that the proof was genuinely verified
        // by this program. Without this check, a different program using the
        // same public inputs could bypass verification.
        assert!(proof.vk == LEAVES_AGG_PROGRAM_VK_HASH);

        let merkle_root: [u8; 32] = proof
            .public_inputs
            .clone()
            .try_into()
            .expect("Public input to be the hash of the chunk tree");
        proofs_hash.push(merkle_root);

        sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
    }

    let merkle_root = compute_merkle_root(proofs_hash);

    sp1_zkvm::io::commit_slice(&merkle_root);
}
