mod verifiers;

use verifiers::interface::{
    verify_proofs, AggregatedVerificationError, InputProofs, ProgramInput, ProgramOutput,
};

pub fn verify_aggregated_proofs() -> Result<ProgramOutput, AggregatedVerificationError> {
    let sp1_compressed_proofs = vec![];
    let input = ProgramInput::new(InputProofs::SP1Compressed(sp1_compressed_proofs));
    verify_proofs(input)
}
