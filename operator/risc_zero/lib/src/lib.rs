use risc0_zkvm::Receipt;

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_ffi(
    receipt_bytes: *const u8,
    receipt_len: u32,
    image_id: *const u8,
    image_id_len: u32,
    public_input: *const u8,
    public_input_len: u32,
) -> bool {
    if receipt_bytes.is_null() || image_id.is_null() {
        return false;
    }

    let receipt_bytes = unsafe { std::slice::from_raw_parts(receipt_bytes, receipt_len as usize) };

    let image_id = unsafe { std::slice::from_raw_parts(image_id, image_id_len as usize) };

    let public_input = unsafe { std::slice::from_raw_parts(public_input, public_input_len as usize) };

    let mut image_id_array = [0u8; 32];
    image_id_array.copy_from_slice(image_id);

    if let Ok(receipt) = bincode::deserialize::<Receipt>(receipt_bytes) {
        if public_input != receipt.journal.bytes {
            return false;
        }

        return receipt.verify(image_id_array).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECEIPT: &[u8] = include_bytes!("../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof");
    const IMAGE_ID: &[u8] = include_bytes!(
        "../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id.bin"
    );
    const PUBLIC_INPUT: &[u8] = include_bytes!(
        "../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub"
    );

    #[test]
    fn verify_risc_zero_receipt_with_image_id_works() {
        let receipt_bytes = RECEIPT.as_ptr();
        let image_id = IMAGE_ID.as_ptr();
        let public_input = PUBLIC_INPUT.as_ptr();

        let result = verify_risc_zero_receipt_ffi(
            receipt_bytes,
            RECEIPT.len() as u32,
            image_id,
            IMAGE_ID.len() as u32,
            public_input,
            PUBLIC_INPUT.len() as u32,
        );
        assert!(result)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        let receipt_bytes = RECEIPT.as_ptr();
        let image_id = IMAGE_ID.as_ptr();
        let public_input = PUBLIC_INPUT.as_ptr();

        let result = verify_risc_zero_receipt_ffi(
            receipt_bytes,
            (RECEIPT.len() - 1) as u32,
            image_id,
            IMAGE_ID.len() as u32,
            public_input,
            PUBLIC_INPUT.len() as u32,
        );
        assert!(!result)
    }
}
