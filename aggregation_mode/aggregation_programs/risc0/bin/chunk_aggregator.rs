#![no_main]

use risc0_aggregation_program::{compute_merkle_root, Input, Risc0ImageIdAndPubInputs};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input = env::read::<Input>();

    let mut proofs_hash: Vec<[u8; 32]> = vec![];

    for proof in &input.proofs_image_id_and_pub_inputs {
        proofs_hash.push(proof.commitment());
        env::verify(proof.image_id.clone(), &proof.public_inputs)
            .expect("proof to be verified correctly");
    }

    let merkle_root = compute_merkle_root(proofs_hash);

    env::commit_slice(&merkle_root);
}
