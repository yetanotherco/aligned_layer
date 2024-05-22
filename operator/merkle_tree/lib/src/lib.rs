mod types;

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use crate::types::{VerificationBatch, VerificationData};

// TODO: Ten times the size of one proof, could be changed later
const MAX_BATCH_SIZE: usize = 2 * 1024 * 1024 * 10;

#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_bytes: &[u8; MAX_BATCH_SIZE],
    batch_len: usize,
    merkle_root: &[u8; 32]
) -> bool {

    let batch: Vec<VerificationData> = serde_json::from_slice(&batch_bytes[..batch_len]).unwrap();
    let batch_merkle_tree: MerkleTree<VerificationBatch> = MerkleTree::build(&batch);
    let batch_merkle_root = hex::encode(batch_merkle_tree.root);
    let received_merkle_root = hex::encode(merkle_root);
    return batch_merkle_root == received_merkle_root;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn test_verify_merkle_tree_batch_ffi() {
        let path = "./test_files/7a3d9215cfac21a4b0e94382e53a9f26bc23ed990f9c850a31ccf3a65aec1466.json";

        let mut file = File::open(path).unwrap();

        let mut bytes_vec = Vec::new();
        file.read_to_end(&mut bytes_vec).unwrap();

        let mut bytes = [0; MAX_BATCH_SIZE];
        bytes[..bytes_vec.len()].copy_from_slice(&bytes_vec);

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode("7a3d9215cfac21a4b0e94382e53a9f26bc23ed990f9c850a31ccf3a65aec1466").unwrap());

        let result = verify_merkle_tree_batch_ffi(&bytes, bytes_vec.len(), &merkle_root);

        assert_eq!(result, true);
    }
}
