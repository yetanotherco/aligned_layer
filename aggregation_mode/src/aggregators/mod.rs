pub mod lib;
pub mod sp1_aggregator;

use sp1_aggregator::{AlignedSP1VerificationError, SP1ProofWithPubValuesAndElf};
pub enum ZKVMEngine {
    SP1,
}

pub enum AlignedProof {
    SP1(SP1ProofWithPubValuesAndElf),
}

impl AlignedProof {
    pub fn hash(&self) -> [u8; 32] {
        match self {
            AlignedProof::SP1(proof) => proof.hash_vk_and_pub_inputs(),
        }
    }
}

#[derive(Debug)]
pub enum AlignedVerificationError {
    Sp1(AlignedSP1VerificationError),
}

impl AlignedProof {
    pub fn verify(&self) -> Result<(), AlignedVerificationError> {
        match self {
            AlignedProof::SP1(proof) => sp1_aggregator::verify(proof).map_err(
                |arg0: sp1_aggregator::AlignedSP1VerificationError| {
                    AlignedVerificationError::Sp1(arg0)
                },
            ),
        }
    }
}
