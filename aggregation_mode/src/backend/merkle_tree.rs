use crate::aggregators::AlignedProof;
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;

/// Returns (merkle_root, leaves)
pub fn compute_proofs_merkle_root(
    proofs: &[AlignedProof],
) -> Option<(MerkleTree<AlignedProof>, Vec<[u8; 32]>)> {
    let merkle_tree: MerkleTree<AlignedProof> = MerkleTree::build(proofs)?;
    let leaves: Vec<[u8; 32]> = proofs.iter().map(|proof| proof.commitment()).collect();

    Some((merkle_tree, leaves))
}
