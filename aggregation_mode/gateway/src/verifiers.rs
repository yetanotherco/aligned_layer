use std::sync::LazyLock;

use sp1_sdk::{CpuProver, Prover, ProverClient, SP1ProofWithPublicValues, SP1VerifyingKey};

static SP1_PROVER_CLIENT_CPU: LazyLock<CpuProver> =
    LazyLock::new(|| ProverClient::builder().cpu().build());

pub enum VerificationError {
    InvalidProof,
    UnsupportedProof,
}

pub fn verify_sp1_proof(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Result<(), VerificationError> {
    let client = &*SP1_PROVER_CLIENT_CPU;

    match proof.proof {
        sp1_sdk::SP1Proof::Compressed(_) => client
            .verify(proof, vk)
            .map_err(|_| VerificationError::InvalidProof),
        _ => Err(VerificationError::UnsupportedProof),
    }?;

    Ok(())
}

/// TODO: implement Zisk proof verification
pub fn verify_zisk_proof(_proof: &[u8]) -> Result<(), VerificationError> {
    Ok(())
}
