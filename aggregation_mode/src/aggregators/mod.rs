pub mod lib;
pub mod risc0_aggregator;
pub mod sp1_aggregator;

use std::fmt::Display;

use risc0_aggregator::{AlignedRisc0VerificationError, Risc0ProofReceiptAndImageId};
use sp1_aggregator::{AlignedSP1VerificationError, SP1ProofWithPubValuesAndElf};

#[derive(Clone, Debug)]
pub enum ZKVMEngine {
    SP1,
    RISC0,
}

impl Display for ZKVMEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SP1 => write!(f, "SP1"),
            Self::RISC0 => write!(f, "Risc0"),
        }
    }
}

impl ZKVMEngine {
    pub fn from_env() -> Option<Self> {
        let key = "AGGREGATOR";
        let value = std::env::var(key).ok()?;
        let engine = match value.as_str() {
            "sp1" => ZKVMEngine::SP1,
            "risc0" => ZKVMEngine::RISC0,
            _ => panic!("Invalid AGGREGATOR, possible options are: sp1|risc0"),
        };

        Some(engine)
    }
}

pub enum AlignedProof {
    SP1(Box<SP1ProofWithPubValuesAndElf>),
    Risc0(Box<Risc0ProofReceiptAndImageId>),
}

impl AlignedProof {
    pub fn hash(&self) -> [u8; 32] {
        match self {
            AlignedProof::SP1(proof) => proof.hash_vk_and_pub_inputs(),
            AlignedProof::Risc0(proof) => proof.hash_image_id_and_public_inputs(),
        }
    }
}

#[derive(Debug)]
pub enum AlignedVerificationError {
    Sp1(AlignedSP1VerificationError),
    Risc0(AlignedRisc0VerificationError),
}

impl AlignedProof {
    pub fn verify(&self) -> Result<(), AlignedVerificationError> {
        match self {
            AlignedProof::SP1(proof) => sp1_aggregator::verify(proof).map_err(
                |arg0: sp1_aggregator::AlignedSP1VerificationError| {
                    AlignedVerificationError::Sp1(arg0)
                },
            ),
            AlignedProof::Risc0(proof) => {
                risc0_aggregator::verify(proof).map_err(AlignedVerificationError::Risc0)
            }
        }
    }
}
