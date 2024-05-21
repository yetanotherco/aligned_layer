mod types;

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use crate::types::{VerificationBatch, VerificationData};

const MAX_BATCH_SIZE: usize = 2 * 1024 * 1024 * 10;

#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_bytes: &[u8; MAX_BATCH_SIZE],
    batch_len: usize,
    merkle_root: &[u8; 32]
) -> bool {
    if let Ok(batch) = bincode::deserialize::<Vec<VerificationData>>(&batch_bytes[..batch_len]) {
        let batch_merkle_tree: MerkleTree<VerificationBatch> = MerkleTree::build(&batch);
        let batch_merkle_root = hex::encode(batch_merkle_tree.root);
        let received_merkle_root = hex::encode(merkle_root);
        return batch_merkle_root == received_merkle_root;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn test_verify_merkle_tree_batch_ffi() {
        // Path to the JSON file
        let path = "./test_files/7a3d9215cfac21a4b0e94382e53a9f26bc23ed990f9c850a31ccf3a65aec1466.json";

        // Open the file
        let mut file = File::open(path).unwrap();

        // Read the file contents into a string
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Parse the JSON contents into a Vec<VerificationData>
        let verification_data: Vec<VerificationData> = serde_json::from_str(&contents).unwrap();

        // Convert the Vec<VerificationData> into bytes
        let bytes = bincode::serialize(&verification_data).unwrap();

        // Transform Vec<u8> into a &[u8; MAX_BATCH_SIZE]
        let mut batch_bytes = [0; MAX_BATCH_SIZE];
        batch_bytes[..bytes.len()].copy_from_slice(&bytes);

        // Transform merkle_root into a &[u8; 32]
        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode("7a3d9215cfac21a4b0e94382e53a9f26bc23ed990f9c850a31ccf3a65aec1466").unwrap());

        // Call the FFI function
        let result = verify_merkle_tree_batch_ffi(&batch_bytes, bytes.len(), &merkle_root);

        // Assert that the result is true
        assert_eq!(result, true);
    }
}
