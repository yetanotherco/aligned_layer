#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use sha2::{Digest, Sha256};
use sp1_aggregation_program::{ChunkAggregatorInput, Hash32};

// Generated with `make agg_mode_write_program_ids` and copied from program_ids.json
pub const USER_PROOFS_AGGREGATOR_PROGRAM_VK_HASH: [u32; 8] = [
    684911098, 272834847, 1514192666, 1104122402, 1853418149, 488480116, 2005139814, 1901405498,
];

pub fn main() {
    let input = sp1_zkvm::io::read::<ChunkAggregatorInput>();

    let mut leaves = vec![];

    // Verify the proofs.
    for (proof, leaves_commitment) in input.proofs_and_leaves_commitment {
        let vkey = proof.vk;
        let public_values_digest = Sha256::digest(&proof.public_inputs);

        // Ensure the aggregated chunk originates from the user proofs aggregation program.
        // This validation step guarantees that the proof was genuinely verified
        // by this program. Without this check, a different program using the
        // same public inputs could bypass verification.
        assert!(proof.vk == USER_PROOFS_AGGREGATOR_PROGRAM_VK_HASH);

        let merkle_root: [u8; 32] = proof
            .public_inputs
            .clone()
            .try_into()
            .expect("Public input to be the hash of the chunk tree");

        // Reconstruct the merkle tree and verify that the roots match
        let leaves_commitment: Vec<Hash32> =
            leaves_commitment.into_iter().map(|el| Hash32(el)).collect();
        let merkle_tree: MerkleTree<Hash32> = MerkleTree::build(&leaves_commitment).unwrap();
        assert!(merkle_tree.root == merkle_root);

        leaves.extend(leaves_commitment);

        sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
    }

    // Finally, compute the final merkle root with all the leaves
    let merkle_tree: MerkleTree<Hash32> = MerkleTree::build(&leaves).unwrap();

    sp1_zkvm::io::commit_slice(&merkle_tree.root);
}
