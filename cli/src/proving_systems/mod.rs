use std::error::Error;
use sp1_sdk::ProverClient;
use crate::types::VerificationData;

pub async fn verify_sp1_proof(verification_data: &VerificationData) -> Result<(), Box<dyn Error>> {
    let proof = &verification_data.proof;
    let elf = verification_data.vm_program_code.as_ref().expect("VM program code is missing");
    let sp1_prover_client: ProverClient = ProverClient::new();
    let (_pk, vk) = sp1_prover_client.setup(elf);
    let proof = bincode::deserialize(proof).map_err(|_| anyhow::anyhow!("Invalid proof"))?;

    sp1_prover_client
        .verify(&proof, &vk)
        .map_err(|_| anyhow::anyhow!("Failed to verify proof"))?;

    Ok(())
}
