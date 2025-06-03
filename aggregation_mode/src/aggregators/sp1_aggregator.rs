use std::sync::LazyLock;

use alloy::primitives::Keccak256;
use sp1_aggregation_program::SP1VkAndPubInputs;
#[cfg(feature = "prove")]
use sp1_sdk::EnvProver;
use sp1_sdk::{
    CpuProver, HashableKey, Prover, ProverClient, SP1ProofWithPublicValues, SP1Stdin,
    SP1VerifyingKey,
};

const CHUNK_PROGRAM_ELF: &[u8] =
    include_bytes!("../../aggregation_programs/sp1/elf/sp1_chunk_aggregator_program");

const USER_PROOFS_PROGRAM_ELF: &[u8] =
    include_bytes!("../../aggregation_programs/sp1/elf/sp1_user_proofs_aggregator_program");

#[cfg(feature = "prove")]
static SP1_PROVER_CLIENT: LazyLock<EnvProver> = LazyLock::new(ProverClient::from_env);

/// Separate prover instance configured to always use the CPU.
/// This is used for verification, which is performed in parallel and
/// cannot be done on the GPU.
static SP1_PROVER_CLIENT_CPU: LazyLock<CpuProver> =
    LazyLock::new(|| ProverClient::builder().cpu().build());

pub struct SP1ProofWithPubValuesAndElf {
    pub proof_with_pub_values: SP1ProofWithPublicValues,
    pub elf: Vec<u8>,
    pub vk: SP1VerifyingKey,
}

#[derive(Debug)]
pub enum AlignedSP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
    UnsupportedProof,
}

impl SP1ProofWithPubValuesAndElf {
    /// Constructs a new instance of the struct by verifying a given SP1 proof with its public values.
    pub fn new(
        proof_with_pub_values: SP1ProofWithPublicValues,
        elf: Vec<u8>,
    ) -> Result<Self, AlignedSP1VerificationError> {
        let client = &*SP1_PROVER_CLIENT_CPU;

        let (_pk, vk) = client.setup(&elf);

        // only sp1 compressed proofs are supported for aggregation now
        match proof_with_pub_values.proof {
            sp1_sdk::SP1Proof::Compressed(_) => client
                .verify(&proof_with_pub_values, &vk)
                .map_err(AlignedSP1VerificationError::Verification),
            _ => Err(AlignedSP1VerificationError::UnsupportedProof),
        }?;

        Ok(Self {
            proof_with_pub_values,
            elf,
            vk,
        })
    }

    pub fn hash_vk_and_pub_inputs(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        let vk_bytes = &self.vk.hash_bytes();
        hasher.update(vk_bytes);
        hasher.update(self.proof_with_pub_values.public_values.as_slice());
        hasher.finalize().into()
    }
}

#[derive(Debug)]
pub enum SP1AggregationError {
    Verification(sp1_sdk::SP1VerificationError),
    Prove(String),
    UnsupportedProof,
}

pub(crate) fn run_user_proofs_aggregator(
    proofs: &[SP1ProofWithPubValuesAndElf],
) -> Result<SP1ProofWithPubValuesAndElf, SP1AggregationError> {
    let mut stdin = SP1Stdin::new();

    let mut program_input = sp1_aggregation_program::UserProofsAggregatorInput {
        proofs_vk_and_pub_inputs: vec![],
    };

    // write vk + public inputs
    for proof in proofs.iter() {
        program_input
            .proofs_vk_and_pub_inputs
            .push(SP1VkAndPubInputs {
                public_inputs: proof.proof_with_pub_values.public_values.to_vec(),
                vk: proof.vk.hash_u32(),
            });
    }

    stdin.write(&program_input);

    // write proofs
    for input_proof in proofs.iter() {
        let vk = input_proof.vk.vk.clone();
        // we only support sp1 Compressed proofs for now
        let sp1_sdk::SP1Proof::Compressed(proof) = input_proof.proof_with_pub_values.proof.clone()
        else {
            return Err(SP1AggregationError::UnsupportedProof);
        };

        stdin.write_proof(*proof, vk);
    }

    #[cfg(feature = "prove")]
    let client = &*SP1_PROVER_CLIENT;
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build();

    let (pk, vk) = client.setup(USER_PROOFS_PROGRAM_ELF);

    let proof = client
        .prove(&pk, &stdin)
        .compressed()
        .run()
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk)
        .map_err(SP1AggregationError::Verification)?;

    let proof_and_elf = SP1ProofWithPubValuesAndElf {
        proof_with_pub_values: proof,
        elf: USER_PROOFS_PROGRAM_ELF.to_vec(),
        vk,
    };

    Ok(proof_and_elf)
}

pub(crate) fn run_chunk_aggregator(
    proofs: &[(SP1ProofWithPubValuesAndElf, Vec<[u8; 32]>)],
) -> Result<SP1ProofWithPubValuesAndElf, SP1AggregationError> {
    let mut stdin = SP1Stdin::new();

    let mut program_input = sp1_aggregation_program::ChunkAggregatorInput {
        proofs_and_leaves_commitment: vec![],
    };

    // write vk + public inputs
    for (proof, leaves_commitment) in proofs.iter() {
        program_input.proofs_and_leaves_commitment.push((
            SP1VkAndPubInputs {
                public_inputs: proof.proof_with_pub_values.public_values.to_vec(),
                vk: proof.vk.hash_u32(),
            },
            leaves_commitment.clone(),
        ));
    }

    stdin.write(&program_input);

    // write proofs
    for (input_proof, _) in proofs.iter() {
        let vk = input_proof.vk.vk.clone();
        // we only support sp1 Compressed proofs for now
        let sp1_sdk::SP1Proof::Compressed(proof) = input_proof.proof_with_pub_values.proof.clone()
        else {
            return Err(SP1AggregationError::UnsupportedProof);
        };

        stdin.write_proof(*proof, vk);
    }

    #[cfg(feature = "prove")]
    let client = &*SP1_PROVER_CLIENT;
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build();

    let (pk, vk) = client.setup(CHUNK_PROGRAM_ELF);

    #[cfg(feature = "prove")]
    let proof = client
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;
    #[cfg(not(feature = "prove"))]
    let proof = client
        .prove(&pk, &stdin)
        // In mock mode, deferred proof verification must be disabled to avoid recursive proof verification.
        // This is because chunk proofs are mocked, and enabling verification would cause a panic.
        // See: https://docs.succinct.xyz/docs/sp1/writing-programs/proof-aggregation#proof-aggregation-in-mock-mode
        .deferred_proof_verification(false)
        .groth16()
        .run()
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk)
        .map_err(SP1AggregationError::Verification)?;

    let proof_and_elf = SP1ProofWithPubValuesAndElf {
        proof_with_pub_values: proof,
        elf: CHUNK_PROGRAM_ELF.to_vec(),
        vk,
    };

    Ok(proof_and_elf)
}

pub fn vk_from_elf(elf: &[u8]) -> SP1VerifyingKey {
    let prover = &*SP1_PROVER_CLIENT_CPU;
    let (_, vk) = prover.setup(elf);
    vk
}
