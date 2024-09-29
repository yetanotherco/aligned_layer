extern crate honggfuzz;

use aligned_batcher::gnark::verify_gnark;
use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug)]
struct InputData {
    proof: Vec<u8>,
    public_input: Vec<u8>,
    verification_key: Vec<u8>,
}

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            if let Ok(input) = InputData::arbitrary(&mut arbitrary::Unstructured::new(data)) {
                let _ = verify_gnark(
                    ProvingSystemArg::Groth16Bn254,
                    &input.proof,
                    &input.public_input,
                    &input.verification_key,
                );
            }
        });
    }
}
