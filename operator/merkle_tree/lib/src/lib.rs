use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use aligned_sdk::types::{VerificationData, VerificationDataCommitment, VerificationCommitmentBatch};


#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_ptr: *const u8,
    batch_len: usize,
    merkle_root: &[u8; 32]
) -> bool {
    if batch_ptr.is_null() || batch_len == 0 {
        println!("batch_ptr == null || batch_len == 0 {}", batch_len);
        return false;
    }
    
    let batch_bytes = unsafe { std::slice::from_raw_parts(batch_ptr, batch_len) };

    let batch = match serde_json::from_slice::<Vec<VerificationData>>(batch_bytes) {
        Ok(batch) => batch,
        Err(e) => {
            println!("Error: {:?}", e);
            return false;
        }
    };

    let batch_data_comm: Vec<VerificationDataCommitment> = batch.into_iter()
                                                            .map(VerificationDataCommitment::from)
                                                            .collect();

    let computed_batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> = MerkleTree::build(&batch_data_comm);
    let computed_batch_merkle_root = hex::encode(computed_batch_merkle_tree.root);
    let received_merkle_root = hex::encode(merkle_root);
    println!("returning after merkle_check: {}", computed_batch_merkle_root == received_merkle_root);

    return computed_batch_merkle_root == received_merkle_root;
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn test_verify_merkle_tree_batch_ffi() {
        let path = "./test_files/5ba2f046e3c1072b96f55728a67d73b4e246a6c27960b0c52d7fafb77981bcb0.json";

        let mut file = File::open(path).unwrap();

        let mut bytes_vec = Vec::new();
        file.read_to_end(&mut bytes_vec).unwrap();

        let mut merkle_root = [0; 32];
        merkle_root.copy_from_slice(&hex::decode("5ba2f046e3c1072b96f55728a67d73b4e246a6c27960b0c52d7fafb77981bcb0").unwrap());

        let result = verify_merkle_tree_batch_ffi(bytes_vec.as_ptr(), bytes_vec.len(), &merkle_root);

        assert_eq!(result, true);
    }
}
