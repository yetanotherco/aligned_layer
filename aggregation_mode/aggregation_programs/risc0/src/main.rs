#![no_main]

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use risc0_aggregation_program::{Input, Risc0ImageIdAndPubInputs};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input = env::read::<Input>();

    for Risc0ImageIdAndPubInputs {
        image_id,
        public_inputs,
    } in &input.proofs_image_id_and_pub_inputs
    {
        env::verify(image_id.clone(), &public_inputs).expect("proof to be verified correctly");
    }

    let merkle_tree: MerkleTree<Risc0ImageIdAndPubInputs> =
        MerkleTree::build(&input.proofs_image_id_and_pub_inputs).unwrap();

    env::commit_slice(&merkle_tree.root);
}
