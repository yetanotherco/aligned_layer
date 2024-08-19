use mina_curves::pasta::Fp;
use mina_p2p_messages::v2::hash_with_kimchi;
use mina_tree::MerklePath;
use std::fmt::Write;

/// Based on OpenMina's implementation
/// https://github.com/openmina/openmina/blob/d790af59a8bd815893f7773f659351b79ed87648/ledger/src/account/account.rs#L1444
pub fn verify_merkle_proof(merkle_leaf: Fp, merkle_path: Vec<MerklePath>, merkle_root: Fp) -> bool {
    let mut param = String::with_capacity(16);

    let calculated_root =
        merkle_path
            .iter()
            .enumerate()
            .fold(merkle_leaf, |accum, (depth, path)| {
                let hashes = match path {
                    MerklePath::Left(right) => [accum, *right],
                    MerklePath::Right(left) => [*left, accum],
                };

                param.clear();
                write!(&mut param, "MinaMklTree{:03}", depth).unwrap();

                hash_with_kimchi(param.as_str(), &hashes)
            });

    calculated_root == merkle_root
}

#[cfg(test)]
mod test {
    #[test]
    fn test_verify_merkle_proof() {
        todo!();
    }
}
