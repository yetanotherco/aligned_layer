use risc0_zkvm::Receipt;

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_ffi(
    receipt_bytes: *const u8,
    receipt_len: u32,
    image_id: &[u32; 8],
) -> bool {
    let receipt_bytes = unsafe {
        assert!(!receipt_bytes.is_null());
        std::slice::from_raw_parts(receipt_bytes, receipt_len as usize)
    };

    if let Ok(receipt) = bincode::deserialize::<Receipt>(receipt_bytes) {
        return receipt.verify(*image_id).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};

    const RECEIPT: &[u8] = include_bytes!("../../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof");

    #[test]
    fn verify_risc_zero_receipt_with_image_id_works() {
        let receipt_bytes = RECEIPT.as_ptr();

        let image_id = read_image_id_from_file("../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/fibonacci_id.txt").expect("Failed to read image ID from file");

        let result = verify_risc_zero_receipt_ffi(receipt_bytes, RECEIPT.len() as u32, &image_id);
        assert!(result)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        let receipt_bytes = RECEIPT.as_ptr();

        let image_id = read_image_id_from_file("../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/fibonacci_id.txt").expect("Failed to read image ID from file");

        let result =
            verify_risc_zero_receipt_ffi(receipt_bytes, (RECEIPT.len() - 1) as u32, &image_id);
        assert!(!result)
    }

    fn read_image_id_from_file(file_name: &str) -> Result<[u32; 8], Box<dyn std::error::Error>> {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let contents = contents.trim_matches(|c: char| c == '[' || c == ']' || c.is_whitespace());
        let parts: Vec<&str> = contents.split(',').collect();
        if parts.len() != 8 {
            return Err("The file does not contain 8 numbers".into());
        }
        let mut image_id = [0u32; 8];

        for (i, part) in parts.iter().enumerate() {
            image_id[i] = part
                .trim()
                .parse()
                .map_err(|e| format!("Error parsing number at index {}: {}", i, e))?;
        }

        Ok(image_id)
    }
}
