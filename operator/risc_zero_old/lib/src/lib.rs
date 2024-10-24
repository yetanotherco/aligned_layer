use log::error;
use risc0_zkvm::{InnerReceipt, Receipt};

fn inner_verify_risc_zero_receipt_ffi(
    inner_receipt_bytes: *const u8,
    inner_receipt_len: u32,
    image_id: *const u8,
    image_id_len: u32,
    public_input: *const u8,
    public_input_len: u32,
) -> bool {
    if inner_receipt_bytes.is_null() || image_id.is_null() {
        error!("Input buffer null");
        return false;
    }

    if inner_receipt_len == 0 || image_id_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    //NOTE: We allow the public input for risc0 to be empty.
    let mut public_input_slice: &[u8] = &[];
    if !public_input.is_null() && public_input_len > 0 {
        public_input_slice =
            unsafe { std::slice::from_raw_parts(public_input, public_input_len as usize) };
    }

    let inner_receipt_bytes =
        unsafe { std::slice::from_raw_parts(inner_receipt_bytes, inner_receipt_len as usize) };

    let image_id = unsafe { std::slice::from_raw_parts(image_id, image_id_len as usize) };

    let mut image_id_array = [0u8; 32];
    image_id_array.copy_from_slice(image_id);

    if let Ok(inner_receipt) = bincode::deserialize::<InnerReceipt>(inner_receipt_bytes) {
        let receipt = Receipt::new(inner_receipt, public_input_slice.to_vec());

        return receipt.verify(image_id_array).is_ok();
    }
    false
}

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_old_ffi(
    inner_receipt_bytes: *const u8,
    inner_receipt_len: u32,
    image_id: *const u8,
    image_id_len: u32,
    public_input: *const u8,
    public_input_len: u32,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        inner_verify_risc_zero_receipt_ffi(
            inner_receipt_bytes,
            inner_receipt_len,
            image_id,
            image_id_len,
            public_input,
            public_input_len,
        )
    });

    match result {
        Ok(v) => v as i32,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECEIPT: &[u8] = include_bytes!("../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_old.proof");
    const IMAGE_ID: &[u8] = include_bytes!(
        "../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/fibonacci_id_old.bin"
    );
    const PUBLIC_INPUT: &[u8] = include_bytes!(
        "../../../../scripts/test_files/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_old.pub"
    );

    #[test]
    fn verify_risc_zero_receipt_with_image_id_works() {
        let receipt_bytes = RECEIPT.as_ptr();
        let image_id = IMAGE_ID.as_ptr();
        let public_input = PUBLIC_INPUT.as_ptr();

        let result = verify_risc_zero_receipt_old_ffi(
            receipt_bytes,
            RECEIPT.len() as u32,
            image_id,
            IMAGE_ID.len() as u32,
            public_input,
            PUBLIC_INPUT.len() as u32,
        );
        assert_eq!(result, 1)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        let receipt_bytes = RECEIPT.as_ptr();
        let image_id = IMAGE_ID.as_ptr();
        let public_input = PUBLIC_INPUT.as_ptr();

        let result = verify_risc_zero_receipt_old_ffi(
            receipt_bytes,
            (RECEIPT.len() - 1) as u32,
            image_id,
            IMAGE_ID.len() as u32,
            public_input,
            PUBLIC_INPUT.len() as u32,
        );
        assert_eq!(result, 0)
    }

    #[test]
    fn verify_risc_zero_input_valid() {
        let receipt_bytes = RECEIPT.as_ptr();
        let image_id = IMAGE_ID.as_ptr();
        let public_input = [].as_ptr();

        let result = verify_risc_zero_receipt_old_ffi(
            receipt_bytes,
            (RECEIPT.len() - 1) as u32,
            image_id,
            IMAGE_ID.len() as u32,
            public_input,
            0,
        );
        assert_eq!(result, 0)
    }
}
