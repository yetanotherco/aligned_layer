extern crate honggfuzz;

use aligned_batcher::sp1::verify_sp1_proof;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct InputData {
    proof: Vec<u8>,
    elf: Vec<u8>,
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            if let Ok(input) = InputData::arbitrary(&mut arbitrary::Unstructured::new(data)) {
                let _ = verify_sp1_proof(&input.proof, &input.elf);
            }
        });
    }
}
