#![no_main]

use risc0_aggregation_program::{compute_merkle_root, Input, Risc0ImageIdAndPubInputs};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input = env::read::<Input>();

    let mut chunks_merkle_root: Vec<[u8; 32]> = vec![];

    for Risc0ImageIdAndPubInputs {
        image_id,
        public_inputs,
    } in &input.proofs_image_id_and_pub_inputs
    {
        let merkle_root: [u8; 32] = public_inputs
            .clone()
            .try_into()
            .expect("Public input to be the chunk merkle root");
        chunks_merkle_root.push(merkle_root);
        env::verify(image_id.clone(), &public_inputs).expect("proof to be verified correctly");
    }

    let merkle_root = compute_merkle_root(chunks_merkle_root);

    env::commit_slice(&merkle_root);
}
