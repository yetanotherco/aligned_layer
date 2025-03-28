use super::sp1::{self, SP1AggregatedProof};
use serde::{Deserialize, Serialize};
use zkvm_sp1_interface::SP1CompressedProof;

#[derive(Serialize, Deserialize)]
pub enum InputProofs {
    SP1Compressed(Vec<SP1CompressedProof>),
}

#[derive(Serialize, Deserialize)]
pub struct ProgramInput {
    proofs: InputProofs,
}

impl ProgramInput {
    pub fn new(input_proofs: InputProofs) -> Self {
        ProgramInput {
            proofs: input_proofs,
        }
    }
}

pub enum AggregatedProof {
    SP1(SP1AggregatedProof),
}

pub struct ProgramOutput {
    pub proof: AggregatedProof,
    pub leaves: Vec<Vec<u8>>,
}

impl ProgramOutput {
    pub fn new(proof: AggregatedProof, leaves: Vec<Vec<u8>>) -> Self {
        Self { proof, leaves }
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

pub fn verify_proofs(input: ProgramInput) -> Result<ProgramOutput, AggregatedVerificationError> {
    match input.proofs {
        InputProofs::SP1Compressed(proofs) => sp1::verify_proof_aggregation(proofs),
    }
}
