use alloy::primitives::Keccak256;
use sp1_aggregator::{ProofInput, SP1ProofInput};
use sp1_sdk::{
    HashableKey, Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};

use crate::zk::aggregator::{AggregatedProof, ProgramOutput, ProofAggregationError};

const PROGRAM_ELF: &[u8] = include_bytes!("../../../zkvm/sp1/elf/sp1_aggregator_program");

// TODO lock prover

pub struct SP1Proof {
    pub vk: SP1VerifyingKey,
    pub proof: SP1ProofWithPublicValues,
}

impl SP1Proof {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk.hash_u32() {
            hasher.update(word.to_le_bytes());
        }
        hasher.update(self.proof.public_values.as_slice());
        hasher.finalize().into()
    }
}

pub struct SP1AggregationInput {
    proofs: Vec<SP1Proof>,
    merkle_root: [u8; 32],
}

pub(crate) fn aggregate_proofs(
    input: SP1AggregationInput,
) -> Result<ProgramOutput, ProofAggregationError> {
    let mut stdin = SP1Stdin::new();

    let mut program_input = sp1_aggregator::Input {
        proofs: vec![],
        merkle_root: input.merkle_root,
    };

    // write vk + public inputs
    for proof in input.proofs.iter() {
        program_input
            .proofs
            .push(ProofInput::SP1Compressed(SP1ProofInput {
                public_inputs: proof.proof.public_values.to_vec(),
                vk: proof.vk.hash_u32(),
            }));
    }
    stdin.write(&program_input);

    // write proofs
    for SP1Proof { proof, vk } in input.proofs {
        // we only support sp1 Compressed proofs for now
        let sp1_sdk::SP1Proof::Compressed(proof) = proof.proof else {
            return Err(ProofAggregationError::UnsupportedProof);
        };
        stdin.write_proof(*proof, vk.vk);
    }

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
        .map_err(|_| ProofAggregationError::SP1Proving)?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk)
        .map_err(ProofAggregationError::SP1Verification)?;

    let proof = SP1Proof { proof, vk };

    let output = ProgramOutput::new(AggregatedProof::SP1(proof));

    Ok(output)
}

pub enum SP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
}

pub(crate) fn verify(proof: &SP1Proof, elf: &[u8]) -> Result<(), SP1VerificationError> {
    let client = ProverClient::from_env();

    let (_pk, vk) = client.setup(elf);
    client
        .verify(&proof.proof, &vk)
        .map_err(SP1VerificationError::Verification)
}
