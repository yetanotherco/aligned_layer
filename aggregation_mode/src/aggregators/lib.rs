use super::{
    risc0_aggregator::{Risc0AggregationInput, Risc0ProofReceiptAndImageId},
    sp1_aggregator::{SP1AggregationInput, SP1ProofWithPubValuesAndElf},
};

pub enum ProgramInput {
    SP1(SP1AggregationInput),
    Risc0(Risc0AggregationInput),
}

pub enum AggregatedProof {
    SP1(Box<SP1ProofWithPubValuesAndElf>),
    Risc0(Box<Risc0ProofReceiptAndImageId>),
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
    Risc0Proving(String),
    UnsupportedProof,
}
