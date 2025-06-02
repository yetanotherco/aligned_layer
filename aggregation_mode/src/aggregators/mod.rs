pub mod risc0_aggregator;
pub mod sp1_aggregator;

use std::fmt::Display;

use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use risc0_aggregator::{
    AlignedRisc0VerificationError, Risc0AggregationError, Risc0ProofReceiptAndImageId,
};
use sha3::{Digest, Keccak256};
use sp1_aggregator::{
    AlignedSP1VerificationError, SP1AggregationError, SP1ProofWithPubValuesAndElf,
};
use tracing::info;

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
    /// This function performs multi-level proof aggregation. It splits the input proofs into chunks of
    /// `proofs_per_chunk`` and uses the `user_proofs_aggregator` to aggregate the proofs.
    /// Then, the `chunk_aggregator` takes the resulting proofs and their corresponding leaves commitments
    /// to produce the final aggregated proof.
    pub fn aggregate_proofs(
        &self,
        proofs: Vec<AlignedProof>,
        proofs_per_chunk: u16,
    ) -> Result<(AlignedProof, [u8; 32]), ProofAggregationError> {
        let res = match self {
            ZKVMEngine::SP1 => {
                let proofs: Vec<SP1ProofWithPubValuesAndElf> = proofs
                    .into_iter()
                    // Fetcher already filtered for SP1
                    // We do this for type casting, as to avoid using generics
                    // or macros in this function
                    .filter_map(|proof| match proof {
                        AlignedProof::SP1(proof) => Some(*proof),
                        _ => None,
                    })
                    .collect();

                let chunks = proofs.chunks(proofs_per_chunk as usize);
                info!(
                    "Total proofs to aggregate {}. They aggregation will be performed in {} chunks (i.e {} proofs per chunk)",
                    proofs.len(),
                    chunks.len(),
                    proofs_per_chunk,
                );

                let mut agg_proofs: Vec<(SP1ProofWithPubValuesAndElf, Vec<[u8; 32]>)> = vec![];
                for (i, chunk) in chunks.enumerate() {
                    let leaves_commitment =
                        chunk.iter().map(|e| e.hash_vk_and_pub_inputs()).collect();
                    let agg_proof = sp1_aggregator::run_user_proofs_aggregator(chunk)
                        .map_err(ProofAggregationError::SP1Aggregation)?;
                    agg_proofs.push((agg_proof, leaves_commitment));

                    info!("Chunk number {} has been aggregated", i);
                }

                info!("All chunks have been aggregated, performing last aggregation...");
                let mut agg_proof = sp1_aggregator::run_chunk_aggregator(&agg_proofs)
                    .map_err(ProofAggregationError::SP1Aggregation)?;

                let merkle_root: [u8; 32] = agg_proof
                    .proof_with_pub_values
                    .public_values
                    .read::<[u8; 32]>();

                (AlignedProof::SP1(agg_proof.into()), merkle_root)
            }
            ZKVMEngine::RISC0 => {
                let proofs: Vec<Risc0ProofReceiptAndImageId> = proofs
                    .into_iter()
                    // Fetcher already filtered for Risc0
                    // We do this for type casting, as to avoid using generics
                    // or macros in this function
                    .filter_map(|proof| match proof {
                        AlignedProof::Risc0(proof) => Some(*proof),
                        _ => None,
                    })
                    .collect();

                let chunks = proofs.chunks(proofs_per_chunk as usize);
                info!(
                    "Total proofs to aggregate {}. They aggregation will be performed in {} chunks (i.e {} proofs per chunk)",
                    proofs.len(),
                    chunks.len(),
                    proofs_per_chunk,
                );

                let mut agg_proofs: Vec<(Risc0ProofReceiptAndImageId, Vec<[u8; 32]>)> = vec![];
                for (i, chunk) in chunks.enumerate() {
                    let leaves_commitment = chunk
                        .iter()
                        .map(|e| e.hash_image_id_and_public_inputs())
                        .collect();
                    let agg_proof = risc0_aggregator::run_user_proofs_aggregator(chunk)
                        .map_err(ProofAggregationError::Risc0Aggregation)?;
                    agg_proofs.push((agg_proof, leaves_commitment));

                    info!("Chunk number {} has been aggregated", i);
                }

                info!("All chunks have been aggregated, performing last aggregation...");
                let agg_proof = risc0_aggregator::run_chunk_aggregator(&agg_proofs)
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

/// Merkle tree commitment for aligned proofs.
///
/// Each leaf node (representing a proof) is committed by hashing:
/// — The program id: the verification key hash in SP1 or the image ID in RISC Zero
/// — Public inputs.
///
/// Intermediate nodes in the tree are formed by computing the keccak pairs of child nodes.
// Note: this MerkleTreeBackend is defined in three locations
// - aggregation_mode/src/aggregators/mod.rs
// - aggregation_mode/src/aggregators/risc0_aggregator.rs
// - aggregation_mode/src/aggregators/sp1_aggregator.rs
// All 3 implementations should match
// The definition on aggregator/mod.rs supports taking proofs from both Risc0 and SP1,
// Additionally, a version that takes the leaves as already hashed data is defined on:
// - batcher/aligned-sdk/src/sdk/aggregation.rs
// This one is used in the SDK since,
// the user may not have access to the proofs that he didn't submit
impl IsMerkleTreeBackend for AlignedProof {
    type Data = AlignedProof;
    type Node = [u8; 32];

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        leaf.commitment()
    }

    /// Computes a commutative Keccak256 hash, ensuring H(a, b) == H(b, a).
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#Hashes
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/Hashes.sol#L17-L19
    ///
    /// Compliant with OpenZeppelin's `processProofCalldata` function from MerkleProof.sol.
    ///
    /// See: https://docs.openzeppelin.com/contracts/5.x/api/utils#MerkleProof
    ///
    /// Source: https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/cryptography/MerkleProof.sol#L114-L128
    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        if child_1 < child_2 {
            hasher.update(child_1);
            hasher.update(child_2);
        } else {
            hasher.update(child_2);
            hasher.update(child_1);
        }
        hasher.finalize().into()
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
