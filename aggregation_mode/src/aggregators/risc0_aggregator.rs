include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, Receipt};
use sha3::{Digest, Keccak256};

pub struct Risc0ProofReceiptAndImageId {
    pub image_id: [u8; 32],
    pub receipt: Receipt,
}

impl Risc0ProofReceiptAndImageId {
    pub fn public_inputs(&self) -> &Vec<u8> {
        &self.receipt.journal.bytes
    }
}

#[derive(Debug)]
pub enum Risc0AggregationError {
    WriteInput(String),
    BuildExecutor(String),
    Prove(String),
    Verification(String),
}

#[derive(Debug)]
pub enum AlignedRisc0VerificationError {
    Verification(String),
    UnsupportedProof,
}

/// Byte representation of the user proofs aggregator image_id, converted from `[u32; 8]` to `[u8; 32]`.
pub const RISC0_USER_PROOFS_AGGREGATOR_PROGRAM_ID_BYTES: [u8; 32] = {
    let mut res = [0u8; 32];
    let mut i = 0;
    while i < 8 {
        let bytes = RISC0_USER_PROOFS_AGGREGATOR_PROGRAM_ID[i].to_le_bytes();
        res[i * 4] = bytes[0];
        res[i * 4 + 1] = bytes[1];
        res[i * 4 + 2] = bytes[2];
        res[i * 4 + 3] = bytes[3];
        i += 1;
    }
    res
};

/// Byte representation of the chunk aggregator image_id, converted from `[u32; 8]` to `[u8; 32]`.
pub const RISC0_CHUNK_AGGREGATOR_PROGRAM_ID_BYTES: [u8; 32] = {
    let mut res = [0u8; 32];
    let mut i = 0;
    while i < 8 {
        let bytes = RISC0_CHUNK_AGGREGATOR_PROGRAM_ID[i].to_le_bytes();
        res[i * 4] = bytes[0];
        res[i * 4 + 1] = bytes[1];
        res[i * 4 + 2] = bytes[2];
        res[i * 4 + 3] = bytes[3];
        i += 1;
    }
    res
};

impl Risc0ProofReceiptAndImageId {
    pub fn hash_image_id_and_public_inputs(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(self.image_id);
        hasher.update(self.public_inputs());
        hasher.finalize().into()
    }
}

pub(crate) fn run_user_proofs_aggregator(
    proofs: &[Risc0ProofReceiptAndImageId],
) -> Result<Risc0ProofReceiptAndImageId, Risc0AggregationError> {
    let mut env_builder = ExecutorEnv::builder();

    // write assumptions and proof image id + pub inputs
    let mut proofs_image_id_and_pub_inputs = vec![];
    for proof in proofs {
        proofs_image_id_and_pub_inputs.push(risc0_aggregation_program::Risc0ImageIdAndPubInputs {
            image_id: proof.image_id,
            public_inputs: proof.receipt.journal.bytes.clone(),
        });
        env_builder.add_assumption(proof.receipt.clone());
    }

    // write input data
    let input = risc0_aggregation_program::UserProofsAggregatorInput {
        proofs_image_id_and_pub_inputs,
    };
    env_builder
        .write(&input)
        .map_err(|e| Risc0AggregationError::WriteInput(e.to_string()))?;

    let env = env_builder
        .build()
        .map_err(|e| Risc0AggregationError::BuildExecutor(e.to_string()))?;

    let prover = default_prover();

    let receipt = prover
        .prove_with_opts(
            env,
            RISC0_USER_PROOFS_AGGREGATOR_PROGRAM_ELF,
            &ProverOpts::composite(),
        )
        .map_err(|e| Risc0AggregationError::Prove(e.to_string()))?
        .receipt;

    receipt
        .verify(RISC0_USER_PROOFS_AGGREGATOR_PROGRAM_ID)
        .map_err(|e| Risc0AggregationError::Verification(e.to_string()))?;

    let proof = Risc0ProofReceiptAndImageId {
        image_id: RISC0_USER_PROOFS_AGGREGATOR_PROGRAM_ID_BYTES,
        receipt,
    };

    Ok(proof)
}

pub(crate) fn run_chunk_aggregator(
    proofs: &[(Risc0ProofReceiptAndImageId, Vec<[u8; 32]>)],
) -> Result<Risc0ProofReceiptAndImageId, Risc0AggregationError> {
    let mut env_builder = ExecutorEnv::builder();

    // write assumptions and proof image id + pub inputs
    let mut proofs_and_leaves_commitment = vec![];
    for (proof, leaves_commitment) in proofs {
        proofs_and_leaves_commitment.push((
            risc0_aggregation_program::Risc0ImageIdAndPubInputs {
                image_id: proof.image_id,
                public_inputs: proof.receipt.journal.bytes.clone(),
            },
            leaves_commitment.clone(),
        ));
        env_builder.add_assumption(proof.receipt.clone());
    }

    // write input data
    let input = risc0_aggregation_program::ChunkAggregatorInput {
        proofs_and_leaves_commitment,
    };
    env_builder
        .write(&input)
        .map_err(|e| Risc0AggregationError::WriteInput(e.to_string()))?;

    let env = env_builder
        .build()
        .map_err(|e| Risc0AggregationError::BuildExecutor(e.to_string()))?;

    let prover = default_prover();

    let receipt = prover
        .prove_with_opts(
            env,
            RISC0_CHUNK_AGGREGATOR_PROGRAM_ELF,
            &ProverOpts::groth16(),
        )
        .map_err(|e| Risc0AggregationError::Prove(e.to_string()))?
        .receipt;

    receipt
        .verify(RISC0_CHUNK_AGGREGATOR_PROGRAM_ID)
        .map_err(|e| Risc0AggregationError::Verification(e.to_string()))?;

    let proof = Risc0ProofReceiptAndImageId {
        image_id: RISC0_CHUNK_AGGREGATOR_PROGRAM_ID_BYTES,
        receipt,
    };

    Ok(proof)
}

pub(crate) fn verify(
    proof: &Risc0ProofReceiptAndImageId,
) -> Result<(), AlignedRisc0VerificationError> {
    // only stark proofs are supported for recursion
    if proof.receipt.inner.composite().is_ok() || proof.receipt.inner.succinct().is_ok() {
        proof
            .receipt
            .verify(proof.image_id)
            .map_err(|e| AlignedRisc0VerificationError::Verification(e.to_string()))
    } else {
        Err(AlignedRisc0VerificationError::UnsupportedProof)
    }
}
