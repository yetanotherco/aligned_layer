use super::sp1_aggregator::{self, SP1AggregationInput, SP1ProofWithPubValuesAndElf};

pub enum ProgramInput {
    SP1(SP1AggregationInput),
}

pub enum AggregatedProof {
    SP1(SP1ProofWithPubValuesAndElf),
}

pub struct ProgramOutput {
    pub proof: AggregatedProof,
}

impl ProgramOutput {
    pub fn new(proof: AggregatedProof) -> Self {
        Self { proof }
    }
}

#[derive(Debug)]
pub enum ProofAggregationError {
    SP1Verification(sp1_sdk::SP1VerificationError),
    SP1Proving,
    UnsupportedProof,
}

pub fn aggregate_proofs(input: ProgramInput) -> Result<ProgramOutput, ProofAggregationError> {
    match input {
        ProgramInput::SP1(input) => sp1_aggregator::aggregate_proofs(input),
    }
}
