#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use sp1_aggregation_program::{compute_merkle_root, Input};

pub fn main() {
    let input = sp1_zkvm::io::read::<Input>();

    let mut proofs_commitment: Vec<[u8; 32]> = vec![];

    // Verify the proofs.
    for proof in input.proofs_vk_and_pub_inputs.iter() {
        let vkey = proof.vk;
        let public_values = &proof.public_inputs;
        let public_values_digest = Sha256::digest(public_values);

        proofs_commitment.push(proof.commitment());

        sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
    }

    let merkle_root = compute_merkle_root(proofs_commitment);

    sp1_zkvm::io::commit_slice(&merkle_root);
}
