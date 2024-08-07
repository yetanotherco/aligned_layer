use risc0_zkvm::{InnerReceipt, Receipt};
use log::error;

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_ffi(
    inner_receipt_bytes: *const u8,
    inner_receipt_len: u32,
    image_id: *const u8,
    image_id_len: u32,
    public_input: *const u8,
    public_input_len: u32,
) -> bool {
    if receipt_bytes.is_null() || image_id.is_null() {
        error!("Input buffer length null");
        return false;
    }

    if receipt_len == 0 || image_id_len == 0 || public_input_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let inner_receipt_bytes =
        unsafe { std::slice::from_raw_parts(inner_receipt_bytes, inner_receipt_len as usize) };

    let image_id = unsafe { std::slice::from_raw_parts(image_id, image_id_len as usize) };

    let public_input =
        unsafe { std::slice::from_raw_parts(public_input, public_input_len as usize) };

    let mut image_id_array = [0u8; 32];
    image_id_array.copy_from_slice(image_id);

    if let Ok(inner_receipt) = bincode::deserialize::<InnerReceipt>(inner_receipt_bytes) {
        let receipt = Receipt::new(inner_receipt, public_input.to_vec());

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
