use risc0_zkvm::Receipt;

pub const MAX_RECEIPT_SIZE: usize = 215523;
pub const MAX_IMAGE_ID_SIZE: usize = 8;

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_ffi(
    receipt_bytes: &[u8; MAX_RECEIPT_SIZE],
    receipt_len: usize,
    image_id: &[u32; MAX_IMAGE_ID_SIZE],
    ) -> bool {
    if let Ok(receipt) = bincode::deserialize::<Receipt>(&receipt_bytes[..receipt_len]) {
        return receipt.verify(*image_id).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECEIPT: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof");
    const IMAGE_ID: &[u32; 8] = &[3090655438, 2953112184, 965953788, 2757110989, 1044116726, 4262054234, 2330742163, 3902204400];

    #[test]
    fn verify_risc_zero_receipt_with_image_id_works() {
       const RECEIPT_SIZE: usize = RECEIPT.len();
       let mut receipt_buffer = [0u8; MAX_RECEIPT_SIZE];
       receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

       let result = verify_risc_zero_receipt_ffi(&receipt_buffer, RECEIPT_SIZE, IMAGE_ID);
       assert!(result)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        const RECEIPT_SIZE: usize = RECEIPT.len();
        let mut receipt_buffer = [42u8; super::MAX_RECEIPT_SIZE];
        receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

        let result = verify_risc_zero_receipt_ffi(&receipt_buffer, RECEIPT_SIZE - 1, IMAGE_ID);
        assert!(!result)
    }
}
