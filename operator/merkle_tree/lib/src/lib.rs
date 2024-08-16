use aligned_sdk::core::types::{
    VerificationCommitmentBatch, VerificationData, VerificationDataCommitment,
};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;

#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_ptr: *const u8,
    batch_len: usize,
    merkle_root: &[u8; 32],
) -> bool {
    if batch_ptr.is_null() || batch_len == 0 {
        return false;
    }

    let batch_bytes = unsafe { std::slice::from_raw_parts(batch_ptr, batch_len) };

    let reader = std::io::Cursor::new(batch_bytes);
    let batch = match ciborium::from_reader::<Vec<VerificationData>, _>(reader) {
        Ok(batch) => batch,
        Err(_e) => {
            // try json
            let batch: Vec<VerificationData> = match serde_json::from_slice(batch_bytes) {
                Ok(batch) => batch,
                Err(_e) => return false,
            };

            batch
        }
    };

    let batch_data_comm: Vec<VerificationDataCommitment> =
        batch.into_iter().map(|v| v.into()).collect();

    let computed_batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
        MerkleTree::build(&batch_data_comm);

    return computed_batch_merkle_tree.root == *merkle_root;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_verify_merkle_tree_batch_ffi() {
        let path =
            "./test_files/a3cf9e0284d77d342087b1ed4ab2de0267417577452a3187c9b9592e4cc89188.json";

        let mut file = File::open(path).unwrap();

        let mut bytes_vec = Vec::new();
        file.read_to_end(&mut bytes_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(
            &hex::decode("a3cf9e0284d77d342087b1ed4ab2de0267417577452a3187c9b9592e4cc89188")
                .unwrap(),
        );

        let result =
            verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);

        assert_eq!(result, true);
    }
}
