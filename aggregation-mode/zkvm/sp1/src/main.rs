#![no_main]
sp1_zkvm::entrypoint!(main);

use sp1_verifier_program::SP1CompressedProof;

/// TODO: write proof aggregation program
///
/// For now we are only receiving the inputs and committing a silly output
/// Future iteration will include the aggregation of proofs and will return the
/// proof of the verification of proofs + merkle tree leaves
pub fn main() {
    let input = sp1_zkvm::io::read::<Vec<SP1CompressedProof>>();
    let result = input.len() + 1;
    sp1_zkvm::io::commit(&result);
}
