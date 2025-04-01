use alloy::primitives::Keccak256;
use sp1_aggregator::{ProofInput, SP1ProofInput};
use sp1_sdk::{
    HashableKey, Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};

use crate::zk::aggregator::{AggregatedProof, ProgramOutput, ProofAggregationError};

const PROGRAM_ELF: &[u8] = include_bytes!("../../../zkvm/sp1/elf/sp1_aggregator_program");

// TODO lock prover

pub struct SP1Proof {
    pub elf: Vec<u8>,
    pub proof: SP1ProofWithPublicValues,
}

impl SP1Proof {
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        for &word in &self.vk().hash_u32() {
            hasher.update(word.to_le_bytes());
        }
        hasher.update(self.proof.public_values.as_slice());
        hasher.finalize().into()
    }

    pub fn vk(&self) -> SP1VerifyingKey {
        vk_from_elf(&self.elf)
    }
}

pub struct SP1AggregationInput {
    pub proofs: Vec<SP1Proof>,
    pub merkle_root: [u8; 32],
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
                vk: proof.vk().hash_u32(),
            }));
    }
    stdin.write(&program_input);

    // write proofs
    for input_proof in input.proofs {
        let vk = input_proof.vk().vk;
        // we only support sp1 Compressed proofs for now
        let sp1_sdk::SP1Proof::Compressed(proof) = input_proof.proof.proof else {
            return Err(ProofAggregationError::UnsupportedProof);
        };
        stdin.write_proof(*proof, vk);
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

    let proof = SP1Proof {
        proof,
        elf: PROGRAM_ELF.to_vec(),
    };

    let output = ProgramOutput::new(AggregatedProof::SP1(proof));

    Ok(output)
}

#[derive(Debug)]
pub enum SP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
    UnsupportedProof,
}

pub(crate) fn verify(sp1_proof: &SP1Proof) -> Result<(), SP1VerificationError> {
    let client = ProverClient::from_env();

    let (_pk, vk) = client.setup(&sp1_proof.elf);

    // only sp1 compressed proofs are supported for aggregation now
    match sp1_proof.proof.proof {
        sp1_sdk::SP1Proof::Compressed(_) => client
            .verify(&sp1_proof.proof, &vk)
            .map_err(SP1VerificationError::Verification),
        _ => Err(SP1VerificationError::UnsupportedProof),
    }
}

pub fn vk_from_elf(elf: &[u8]) -> SP1VerifyingKey {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(elf);
    vk
}
