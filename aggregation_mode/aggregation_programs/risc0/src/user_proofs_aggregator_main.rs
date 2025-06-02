#![no_main]

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use risc0_aggregation_program::{Risc0ImageIdAndPubInputs, UserProofsAggregatorInput};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    let input = env::read::<UserProofsAggregatorInput>();

    for proof in &input.proofs_image_id_and_pub_inputs {
        env::verify(proof.image_id.clone(), &proof.public_inputs)
            .expect("proof to be verified correctly");
    }

    let merkle_tree =
        MerkleTree::<Risc0ImageIdAndPubInputs>::build(&input.proofs_image_id_and_pub_inputs)
            .unwrap();

    env::commit_slice(&merkle_tree.root);
}
