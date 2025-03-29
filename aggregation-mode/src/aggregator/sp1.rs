use sp1_aggregator::SP1CompressedProof;
use sp1_sdk::{ProverClient, SP1ProofMode, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

use super::interface::{AggregatedProof, AggregatedVerificationError, ProgramOutput};

const PROGRAM_ELF: &[u8] = include_bytes!("../../zkvm/sp1/elf/sp1_aggregator_program");

pub struct SP1AggregatedProof {
    pub proof: SP1ProofWithPublicValues,
    pub vk: SP1VerifyingKey,
}

pub(crate) fn aggregate_proofs(
    proofs: Vec<SP1CompressedProof>,
) -> Result<ProgramOutput, AggregatedVerificationError> {
    #[cfg(feature = "prove")]
    {
        prove(proofs)
    }
    // If not in prove mode, execute the program and create a mock proof
    #[cfg(not(feature = "prove"))]
    {
        mock_prove(proofs)
    }
}

#[cfg(feature = "prove")]
fn prove(proofs: Vec<SP1CompressedProof>) -> Result<ProgramOutput, AggregatedVerificationError> {
    let mut stdin = SP1Stdin::new();
    stdin.write(&proofs);

    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(PROGRAM_ELF);
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .map_err(|_| AggregatedVerificationError::SP1Proving)?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk)
        .map_err(AggregatedVerificationError::SP1Verification)?;

    let output = ProgramOutput::new(AggregatedProof::SP1(SP1AggregatedProof { proof, vk }));

    Ok(output)
}

fn mock_prove(
    proofs: Vec<SP1CompressedProof>,
) -> Result<ProgramOutput, AggregatedVerificationError> {
    let mut stdin = SP1Stdin::new();
    stdin.write(&proofs);

    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(PROGRAM_ELF);
    let (public_vales, _) = client
        .execute(PROGRAM_ELF, &stdin)
        .run()
        .map_err(|_| AggregatedVerificationError::SP1Proving)?;

    let output = ProgramOutput::new(AggregatedProof::SP1(SP1AggregatedProof {
        proof: SP1ProofWithPublicValues::create_mock_proof(
            &pk,
            public_vales,
            SP1ProofMode::Groth16,
            "",
        ),
        vk,
    }));

    Ok(output)
}
