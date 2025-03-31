use crate::zk::backends::sp1::{self, SP1AggregatedProof};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ProgramInput {
    SP1(sp1_aggregator::Input),
}

pub enum AggregatedProof {
    SP1(SP1AggregatedProof),
}

pub struct ProgramOutput {
    pub proof: AggregatedProof,
}

impl ProgramOutput {
    pub fn new(proof: AggregatedProof) -> Self {
        Self { proof }
    }

    /// TODO: return the contract calldata to verify proof
    pub fn calldata(&self) -> Vec<u8> {
        vec![]
    }
}

#[derive(Debug)]
pub enum ProofAggregationError {
    SP1Verification(sp1_sdk::SP1VerificationError),
    SP1Proving,
}

pub fn aggregate_proofs(input: ProgramInput) -> Result<ProgramOutput, ProofAggregationError> {
    match input {
        ProgramInput::SP1(input) => sp1::aggregate_proofs(input),
    }
}
