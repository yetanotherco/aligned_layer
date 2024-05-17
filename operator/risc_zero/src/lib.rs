use risc0_zkvm::Receipt;

pub const MAX_PROOF_SIZE: usize = 2 * 1024 * 1024;
pub const MAX_IMAGE_ID_SIZE: usize = 8;

#[no_mangle]
pub extern "C" fn verify_risc_zero_proof_ffi(
    receipt_bytes: &[u8],
    receipt_len: usize,
    image_id: [u32; MAX_IMAGE_ID_SIZE],
    ) -> bool {
    if let Ok(receipt) = bincode::deserialize::<Receipt>(&receipt_bytes[..receipt_len]) {
        return receipt.verify(image_id).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECEIPT: &[u8] =
        include_bytes!("../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof");
    const IMAGE_ID: [u32; 8] = [2168696514, 4069298130, 1005557306, 3274294743, 1735077096, 3539040653, 808254153, 306297660];

    #[test]
    fn verify_risc_zero_proof_with_image_id_works() {
       const RECEIPT_SIZE: usize = RECEIPT.len();
       let mut receipt_buffer = [0u8; RECEIPT_SIZE];
       receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

       let result = verify_risc_zero_proof_ffi(&receipt_buffer, RECEIPT_SIZE, IMAGE_ID);
       assert!(result)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        const RECEIPT_SIZE: usize = RECEIPT.len();
        let mut receipt_buffer = [42u8; RECEIPT_SIZE];
        receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

        let result = verify_risc_zero_proof_ffi(&receipt_buffer, RECEIPT_SIZE - 1, IMAGE_ID);
        assert!(!result)
    }
}
