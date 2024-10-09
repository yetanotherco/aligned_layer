use mina_bridge_core::proof::account_proof::MerkleNode;
use mina_curves::pasta::Fp;
use mina_p2p_messages::v2::hash_with_kimchi;
use std::fmt::Write;

/// Based on OpenMina's implementation
/// https://github.com/openmina/openmina/blob/d790af59a8bd815893f7773f659351b79ed87648/ledger/src/account/account.rs#L1444
pub fn verify_merkle_proof(merkle_leaf: Fp, merkle_path: Vec<MerkleNode>, merkle_root: Fp) -> bool {
    let mut param = String::with_capacity(16);

    let calculated_root =
        merkle_path
            .iter()
            .enumerate()
            .fold(merkle_leaf, |accum, (depth, path)| {
                let hashes = match path {
                    MerkleNode::Left(right) => [accum, *right],
                    MerkleNode::Right(left) => [*left, accum],
                };

                param.clear();
                write!(&mut param, "MinaMklTree{:03}", depth).unwrap();

                hash_with_kimchi(param.as_str(), &hashes)
            });
    calculated_root == merkle_root
}

#[cfg(test)]
mod test {
    use ark_serialize::CanonicalDeserialize;

    use super::*;

    #[test]
    fn test_verify_merkle_proof() {
        let merkle_leaf = Fp::from(0);
        let merkle_path = vec![
            MerkleNode::Left(Fp::from(0)),
            MerkleNode::Right(Fp::from(0)),
        ];
        let merkle_root = Fp::deserialize(
            &[
                140u8, 130, 39, 24, 215, 108, 36, 34, 181, 80, 10, 131, 110, 152, 243, 145, 144,
                175, 100, 161, 62, 28, 236, 143, 184, 143, 185, 114, 129, 4, 63, 47,
            ][..],
        )
        .unwrap();
        assert!(verify_merkle_proof(merkle_leaf, merkle_path, merkle_root))
    }
}
