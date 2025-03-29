use super::sp1::{self, SP1AggregatedProof};
use serde::{Deserialize, Serialize};
use sp1_aggregator::SP1CompressedProof;

#[derive(Serialize, Deserialize)]
pub enum InputProofs {
    SP1Compressed(Vec<SP1CompressedProof>),
}

#[derive(Serialize, Deserialize)]
pub struct ProgramInput {
    proofs: InputProofs,
}

impl ProgramInput {
    pub fn new(proofs: InputProofs) -> Self {
        ProgramInput { proofs }
    }
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
pub enum AggregatedVerificationError {
    SP1Verification(sp1_sdk::SP1VerificationError),
    SP1Proving,
}

pub fn aggregate_proofs(input: ProgramInput) -> Result<ProgramOutput, AggregatedVerificationError> {
    match input.proofs {
        InputProofs::SP1Compressed(proofs) => sp1::aggregate_proofs(proofs),
    }
}
