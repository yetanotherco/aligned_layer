#![no_main]

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use risc0_aggregation_program::{ChunkAggregatorInput, Hash32};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

// Generated with `make proof_aggregator_write_program_ids` and copied from program_ids.json
pub const USER_PROOFS_AGGREGATOR_PROGRAM_IMAGE_ID: [u8; 32] = [
    47, 25, 48, 145, 237, 110, 250, 163, 139, 45, 130, 27, 50, 5, 205, 150, 88, 149, 37, 56, 39,
    72, 111, 218, 142, 90, 33, 124, 230, 176, 137, 143,
];

fn main() {
    let input = env::read::<ChunkAggregatorInput>();

    let mut leaves: Vec<Hash32> = vec![];

    for (proof, leaves_commitment) in input.proofs_and_leaves_commitment {
        let image_id = proof.image_id;

        // Ensure the aggregated chunk originates from the L1 aggregation program.
        // This validation step guarantees that the proof was genuinely verified
        // by this program. Without this check, a different program using the
        // same public inputs could bypass verification.
        assert!(image_id == USER_PROOFS_AGGREGATOR_PROGRAM_IMAGE_ID);

        // Ensure the committed root matches the root of the provided leaves
        let merkle_root: [u8; 32] = proof
            .public_inputs
            .clone()
            .try_into()
            .expect("Public input to be the chunk merkle root");

        let leaves_commitment: Vec<Hash32> = leaves_commitment.into_iter().map(Hash32).collect();
        let merkle_tree = MerkleTree::<Hash32>::build(&leaves_commitment).unwrap();
        assert!(merkle_root == merkle_tree.root);

        leaves.extend(leaves_commitment);

        // finally verify the proof
        env::verify(image_id, &proof.public_inputs).expect("proof to be verified correctly");
    }

    let merkle_tree = MerkleTree::<Hash32>::build(&leaves).unwrap();

    env::commit_slice(&merkle_tree.root);
}
