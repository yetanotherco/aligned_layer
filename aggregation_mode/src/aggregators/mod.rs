pub mod risc0_aggregator;
pub mod sp1_aggregator;

use std::fmt::Display;

use risc0_aggregator::{
    AlignedRisc0VerificationError, Risc0AggregationError, Risc0ProofReceiptAndImageId,
};
use sp1_aggregator::{
    AlignedSP1VerificationError, SP1AggregationError, SP1ProofWithPubValuesAndElf,
};

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

#[derive(Debug)]
pub enum ProofAggregationError {
    SP1Aggregation(SP1AggregationError),
    Risc0Aggregation(Risc0AggregationError),
    PublicInputsDeserialization,
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

    /// Aggregates a list of [`AlignedProof`]s into a single [`AlignedProof`].
    ///
    /// Returns a tuple containing:
    /// - The aggregated [`AlignedProof`], representing the combined proof
    /// - The Merkle root computed within the ZKVM, exposed as a public input
    ///
    /// This function performs proof aggregation and ensures the resulting Merkle root
    /// can be independently verified by external systems.
    pub fn aggregate_proofs(
        &self,
        proofs: Vec<AlignedProof>,
    ) -> Result<(AlignedProof, [u8; 32]), ProofAggregationError> {
        let res = match self {
            ZKVMEngine::SP1 => {
                let proofs = proofs
                    .into_iter()
                    // Fetcher already filtered for SP1
                    // We do this for type casting, as to avoid using generics
                    // or macros in this function
                    .filter_map(|proof| match proof {
                        AlignedProof::SP1(proof) => Some(*proof),
                        _ => None,
                    })
                    .collect();

                let mut agg_proof = sp1_aggregator::aggregate_proofs(proofs)
                    .map_err(ProofAggregationError::SP1Aggregation)?;

                let merkle_root: [u8; 32] = agg_proof
                    .proof_with_pub_values
                    .public_values
                    .read::<[u8; 32]>();

                (AlignedProof::SP1(agg_proof.into()), merkle_root)
            }
            ZKVMEngine::RISC0 => {
                let proofs = proofs
                    .into_iter()
                    // Fetcher already filtered for Risc0
                    // We do this for type casting, as to avoid using generics
                    // or macros in this function
                    .filter_map(|proof| match proof {
                        AlignedProof::Risc0(proof) => Some(*proof),
                        _ => None,
                    })
                    .collect();

                let agg_proof = risc0_aggregator::aggregate_proofs(proofs)
                    .map_err(ProofAggregationError::Risc0Aggregation)?;

                // Note: journal.decode() won't work here as risc0 deserializer works under u32 words
                let public_input_bytes = agg_proof.receipt.journal.as_ref();
                let merkle_root: [u8; 32] = public_input_bytes
                    .try_into()
                    .map_err(|_| ProofAggregationError::PublicInputsDeserialization)?;

                (AlignedProof::Risc0(agg_proof.into()), merkle_root)
            }
        };

        Ok(res)
    }
}

pub enum AlignedProof {
    SP1(Box<SP1ProofWithPubValuesAndElf>),
    Risc0(Box<Risc0ProofReceiptAndImageId>),
}

impl AlignedProof {
    pub fn commitment(&self) -> [u8; 32] {
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
