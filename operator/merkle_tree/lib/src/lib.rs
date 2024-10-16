use aligned_sdk::core::types::{
    VerificationCommitmentBatch, VerificationData, VerificationDataCommitment,
};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::error;

fn inner_verify_merkle_tree_batch_ffi(
    batch_ptr: *const u8,
    batch_len: usize,
    merkle_root: &[u8; 32],
) -> bool {
    if batch_ptr.is_null() {
        error!("Batch buffer null");
        return false;
    }

    if batch_len == 0 {
        error!("Batch buffer length 0");
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

    if batch.is_empty() {
        return false;
    }

    let batch_data_comm: Vec<VerificationDataCommitment> =
        batch.into_iter().map(|v| v.into()).collect();

    let Some(computed_batch_merkle_tree) =
        MerkleTree::<VerificationCommitmentBatch>::build(&batch_data_comm)
    else {
        error!("Failed to build merkle tree, batch data commitment is empty");
        return false;
    };

    computed_batch_merkle_tree.root == *merkle_root
}

#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_ptr: *const u8,
    batch_len: usize,
    merkle_root: &[u8; 32],
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        inner_verify_merkle_tree_batch_ffi(batch_ptr, batch_len, merkle_root)
    });

    match result {
        Ok(v) => v as i32,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn verify_merkle_tree_batch_returns_true() {
        let mut merkle_batch_file = File::open("./test_files/merkle_tree_batch.bin").unwrap();
        let mut bytes_vec = Vec::new();
        merkle_batch_file.read_to_end(&mut bytes_vec).unwrap();

        let mut merkle_root_file = File::open("./test_files/merkle_root.bin").unwrap();
        let mut root_vec = Vec::new();
        merkle_root_file.read_to_end(&mut root_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode(&root_vec).unwrap());

        let result =
            verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);

        assert_eq!(result, 1);
    }

    #[test]
    fn merkle_batch_len_1_does_not_panic() {
        let bytes_vec = vec![1u8];

        let mut merkle_root_file = File::open("./test_files/merkle_root.bin").unwrap();
        let mut root_vec = Vec::new();
        merkle_root_file.read_to_end(&mut root_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode(&root_vec).unwrap());

        let result =
            verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);

        assert_eq!(result, 0);
    }

    #[test]
    fn merkle_batch_len_0_does_not_panic() {
        let bytes_vec = Vec::new();

        let mut merkle_root_file = File::open("./test_files/merkle_root.bin").unwrap();
        let mut root_vec = Vec::new();
        merkle_root_file.read_to_end(&mut root_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode(&root_vec).unwrap());

        let result =
            verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);

        assert_eq!(result, 0);
    }
}
