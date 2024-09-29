extern crate honggfuzz;

use aligned_batcher::risc_zero::verify_risc_zero_proof;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct InputData {
    inner_receipt_bytes: Vec<u8>,
    image_id: Vec<u8>,
    public_input: Vec<u8>,
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            if let Ok(input) = InputData::arbitrary(&mut arbitrary::Unstructured::new(data)) {
                let _ = verify_risc_zero_proof(
                    &input.inner_receipt_bytes,
                    &input.image_id,
                    &input.public_input,
                );
            }
        });
    }
}
