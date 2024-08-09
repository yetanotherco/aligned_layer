use risc0_zkvm::{InnerReceipt, Receipt};

pub fn verify_risc_zero_proof(
    inner_receipt_bytes: &[u8],
    image_id: &[u8; 32],
    public_input: &[u8],
) -> bool {
    if let Ok(inner_receipt) = bincode::deserialize::<InnerReceipt>(inner_receipt_bytes) {
        let receipt = Receipt::new(inner_receipt, public_input.to_vec());

        return receipt.verify(*image_id).is_ok();
    }
    false
}
