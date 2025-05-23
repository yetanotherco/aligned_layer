#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use sha2::{Digest, Sha256};
use sp1_aggregation_program::{SP1VkAndPubInputs, UserProofsAggregatorInput};

pub fn main() {
    let input = sp1_zkvm::io::read::<UserProofsAggregatorInput>();

    // Verify the proofs.
    for proof in input.proofs_vk_and_pub_inputs.iter() {
        let vkey = proof.vk;
        let public_values = &proof.public_inputs;
        let public_values_digest = Sha256::digest(public_values);

        sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
    }

    let merkle_tree =
        MerkleTree::<SP1VkAndPubInputs>::build(&input.proofs_vk_and_pub_inputs).unwrap();

    sp1_zkvm::io::commit_slice(&merkle_tree.root);
}
