use super::{
    risc0_aggregator::{self, Risc0AggregationInput, Risc0ProofReceiptAndImageId},
    sp1_aggregator::{self, SP1AggregationInput, SP1ProofWithPubValuesAndElf},
};

pub enum ProgramInput {
    SP1(SP1AggregationInput),
    Risc0(Risc0AggregationInput),
}

pub enum AggregatedProof {
    SP1(SP1ProofWithPubValuesAndElf),
    Risc0(Risc0ProofReceiptAndImageId),
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
    Risc0Proving,
    UnsupportedProof,
}

pub fn aggregate_proofs(input: ProgramInput) -> Result<ProgramOutput, ProofAggregationError> {
    match input {
        ProgramInput::SP1(input) => sp1_aggregator::aggregate_proofs(input),
        ProgramInput::Risc0(input) => risc0_aggregator::aggregate_proofs(input),
    }
}
