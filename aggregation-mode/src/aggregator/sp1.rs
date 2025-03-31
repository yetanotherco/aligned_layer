use sp1_sdk::{Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

use super::interface::{AggregatedProof, AggregatedVerificationError, ProgramOutput};

const PROGRAM_ELF: &[u8] = include_bytes!("../../zkvm/sp1/elf/sp1_aggregator_program");

// TODO lock prover

pub struct SP1Proof {
    pub elf: Vec<u8>,
    pub proof: Vec<u8>,
}

pub struct SP1AggregatedProof {
    pub proof: SP1ProofWithPublicValues,
    pub vk: SP1VerifyingKey,
}

pub(crate) fn aggregate_proofs(
    input: sp1_aggregator::Input,
) -> Result<ProgramOutput, AggregatedVerificationError> {
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    #[cfg(feature = "prove")]
    let client = ProverClient::from_env();
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build();

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

    let proof = SP1AggregatedProof { proof, vk };

    let output = ProgramOutput::new(AggregatedProof::SP1(proof));

    Ok(output)
}

pub enum SP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
    DecodeProofBinary,
}

pub(crate) fn verify(proof: &SP1Proof) -> Result<(), SP1VerificationError> {
    let client = ProverClient::from_env();

    let (_pk, vk) = client.setup(&proof.elf);

    if let Ok(proof) = bincode::deserialize(&proof.proof) {
        client
            .verify(&proof, &vk)
            .map_err(SP1VerificationError::Verification)
    } else {
        Err(SP1VerificationError::DecodeProofBinary)
    }
}
