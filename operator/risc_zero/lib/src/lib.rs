use risc0_zkvm::Receipt;

pub const MAX_RECEIPT_SIZE: usize = 215523;

#[no_mangle]
pub extern "C" fn verify_risc_zero_receipt_ffi(
    receipt_bytes: &[u8; MAX_RECEIPT_SIZE],
    receipt_len: u32,
    image_id: &[u32; 8],
) -> bool {
    if let Ok(receipt) = bincode::deserialize::<Receipt>(&receipt_bytes[..receipt_len as usize]) {
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
        const RECEIPT_SIZE: u32 = RECEIPT.len() as u32;
        let mut receipt_buffer = [0u8; super::MAX_RECEIPT_SIZE];
        receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

        let image_id = read_image_id_from_file("../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/fibonacci_id.txt").expect("Failed to read image ID from file");

        let result = verify_risc_zero_receipt_ffi(&receipt_buffer, RECEIPT_SIZE, &image_id);
        assert!(result)
    }

    #[test]
    fn verify_risc_zero_aborts_with_bad_proof() {
        const RECEIPT_SIZE: u32 = RECEIPT.len() as u32;
        let mut receipt_buffer = [42u8; super::MAX_RECEIPT_SIZE];
        receipt_buffer[..RECEIPT_SIZE].clone_from_slice(RECEIPT);

        let image_id = read_image_id_from_file("../../../task_sender/test_examples/risc_zero/fibonacci_proof_generator/fibonacci_id.txt").expect("Failed to read image ID from file");

        let result = verify_risc_zero_receipt_ffi(&receipt_buffer, RECEIPT_SIZE - 1, &image_id);
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
            image_id[i] = part.trim().parse().map_err(|e| format!("Error parsing number at index {}: {}", i, e))?;
        }

        Ok(image_id)
    }
}
