#![no_main]
ziskos::entrypoint!(main);

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use zisk_aggregation_program::{ChunkAggregatorInput, Hash32};

// Generated with `make proof_aggregator_write_program_ids` and copied from program_ids.json
pub const USER_PROOFS_AGGREGATOR_PROGRAM_ROM_ROOT: [u64; 4] = [
    6589631844296419412,
    9245669750987062479,
    9069898615149755662,
    9755939384656322398,
];

pub fn main() {
    let input = ziskos::read_input_slice();
    let input = bincode::deserialize::<ChunkAggregatorInput>(&input).unwrap();

    let mut leaves = vec![];

    // Verify the proofs.
    for (proof, leaves_commitment) in input.proofs_and_leaves_commitment {
        let proof_words = bytemuck::cast_slice::<u8, u64>(&proof.proof);

        // Reading public inputs as done in the verify of the lib at https://github.com/0xPolygonHermez/zisk/blob/maint/checkouts/pil2-proofman-3d49384e4e2f0af7/78497c5/verifier/src/verifier.rs#L66-L73

        // First 64 bits are the length of public outputs so we skip it
        let mut proof_idx = 1;

        // Next 4 entries are the program rom-key
        let mut rom_vkey: [u64; 4] = [0_u64; 4];
        for i in 0..4 {
            rom_vkey[i as usize] = proof_words[proof_idx];
            proof_idx += 1;
        }

        // Next entry is the number of public inputs set by the program with 'ziskos::set_output'
        // we should end up with a vector of length 4 as the public input is a 256 bits digest
        let mut publics = Vec::new();
        let n_public_inputs = proof_words[proof_idx];
        proof_idx += 1;
        for _ in 0..n_public_inputs {
            publics.push(proof_words[proof_idx]);
            proof_idx += 1;
        }

        // Ensure the aggregated chunk originates from the user proofs aggregation program.
        // This validation step guarantees that the proof was genuinely verified
        // by this program. Without this check, a different program using the
        // same public inputs could bypass verification.
        assert!(rom_vkey == USER_PROOFS_AGGREGATOR_PROGRAM_ROM_ROOT);

        let mut merkle_root = [0u8; 32];
        for (idx, word) in publics.iter().enumerate() {
            let start = idx * 4;
            merkle_root[start..start + 4].copy_from_slice(&(*word as u32).to_le_bytes());
        }

        // Reconstruct the merkle tree and verify that the roots match
        let leaves_commitment: Vec<Hash32> = leaves_commitment.into_iter().map(Hash32).collect();
        let merkle_tree: MerkleTree<Hash32> = MerkleTree::build(&leaves_commitment).unwrap();
        assert!(merkle_tree.root == merkle_root);

        leaves.extend(leaves_commitment);

        proofman_verifier::verify(&proof.proof, &input.vk);
    }

    // Finally, compute the final merkle root with all the leaves
    let merkle_tree: MerkleTree<Hash32> = MerkleTree::build(&leaves).unwrap();

    merkle_tree
        .root
        .chunks_exact(4)
        .enumerate()
        .for_each(|(idx, bytes)| {
            ziskos::set_output(idx, u32::from_le_bytes(bytes.try_into().unwrap()))
        });
}
