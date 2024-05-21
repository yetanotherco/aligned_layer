use serde::{Deserialize, Serialize};

const MAX_BATCH_SIZE: usize = 2 * 1024 * 1024 * 10;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>
}

#[no_mangle]
pub extern "C" fn verify_merkle_tree_batch_ffi(
    batch_bytes: &[u8; MAX_BATCH_SIZE],
    batch_len: usize,
) -> bool {

    if let Ok(batch) = bincode::deserialize::<Vec<VerificationData>>(&batch_bytes[..batch_len]) {
        println!("Batch: {:?}", batch);
        return true;
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
        let path = "/Users/nicolasrampoldi/Downloads/b4b654a31b43c7b5711206eea7d44f884ece1fe7164b478fa16215be77dc84cb.json";

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

        // Call the FFI function
        let result = verify_merkle_tree_batch_ffi(&batch_bytes, bytes.len());

        // Assert that the result is true
        assert_eq!(result, true);
    }
}
