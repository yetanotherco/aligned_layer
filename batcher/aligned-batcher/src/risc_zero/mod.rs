use risc0_zkvm::Receipt;

pub fn verify_risc_zero_proof(
    receipt_bytes: &[u8],
    image_id: &[u8; 32],
    public_input: &[u8],
) -> bool {
    if let Ok(receipt) = bincode::deserialize::<Receipt>(receipt_bytes) {
        if public_input != receipt.journal.bytes {
            return false;
        }

        return receipt.verify(*image_id).is_ok();
    }
    false
}
