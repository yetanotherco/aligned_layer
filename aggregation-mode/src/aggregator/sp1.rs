use sp1_aggregator::SP1CompressedProof;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

use super::interface::{AggregatedProof, AggregatedVerificationError, ProgramOutput};

const PROGRAM_ELF: &[u8] = include_bytes!("../../zkvm/sp1/elf/sp1_aggregator_program");

pub struct SP1AggregatedProof {
    proof: SP1ProofWithPublicValues,
    vk: SP1VerifyingKey,
}

pub(crate) fn aggregate_proofs(
    proofs: Vec<SP1CompressedProof>,
) -> Result<ProgramOutput, AggregatedVerificationError> {
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
