use std::sync::LazyLock;

use alloy::primitives::Keccak256;
use sp1_aggregation_program::SP1VkAndPubInputs;
use sp1_sdk::{
    EnvProver, HashableKey, Prover, ProverClient, SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin, SP1VerifyingKey
};

const ROOT_PROGRAM_ELF: &[u8] =
    include_bytes!("../../aggregation_programs/sp1/elf/sp1_root_aggregator_program");

const CHUNK_PROGRAM_ELF: &[u8] =
    include_bytes!("../../aggregation_programs/sp1/elf/sp1_chunk_aggregator_program");

static SP1_PROVER_CLIENT: LazyLock<EnvProver> = LazyLock::new(ProverClient::from_env);

pub struct SP1ProofWithPubValuesAndElf {
    pub proof_with_pub_values: SP1ProofWithPublicValues,
    pub elf: Vec<u8>,
}

impl SP1ProofWithPubValuesAndElf {
    pub fn hash_vk_and_pub_inputs(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        let vk_bytes = &self.vk().hash_bytes();
        hasher.update(vk_bytes);
        hasher.update(self.proof_with_pub_values.public_values.as_slice());
        hasher.finalize().into()
    }

    pub fn vk(&self) -> SP1VerifyingKey {
        vk_from_elf(&self.elf)
    }
}

#[derive(Debug)]
pub enum SP1AggregationError {
    Verification(sp1_sdk::SP1VerificationError),
    Prove(String),
    UnsupportedProof,
}

pub(crate) fn aggregate_proofs(
    proofs: &[SP1ProofWithPubValuesAndElf],
    is_aggregated_chunk: bool,
    should_wrap_to_groth16: bool,
) -> Result<SP1ProofWithPubValuesAndElf, SP1AggregationError> {
    let mut stdin = SP1Stdin::new();

    println!("Len proofs: {}", proofs.len());

    let mut program_input = sp1_aggregation_program::Input {
        proofs_vk_and_pub_inputs: vec![]
    };

    
    // write vk + public inputs
    for (i, proof) in proofs.iter().enumerate() {
        println!("Proof #{} public values: {:?}", i, proof.proof_with_pub_values.public_values);
        
        program_input
            .proofs_vk_and_pub_inputs
            .push(SP1VkAndPubInputs {
                public_inputs: proof.proof_with_pub_values.public_values.to_vec(),
                vk: proof.vk().hash_u32(),
            });
    }

    stdin.write(&program_input);

    // write proofs
    for (i, input_proof) in proofs.iter().enumerate() {
        let vk = input_proof.vk().vk;
        // we only support sp1 Compressed proofs for now
        let sp1_sdk::SP1Proof::Compressed(proof) = input_proof.proof_with_pub_values.proof.clone()
        else {
            return Err(SP1AggregationError::UnsupportedProof);
        };

        println!("Proof # VK: {:?}", vk);
        println!("Proof #: {:?}", proof);

        // Print first 32 bytes of the proof for debugging
        stdin.write_proof(*proof, vk);
    }

    #[cfg(feature = "prove")]
    let client = &*SP1_PROVER_CLIENT;
    
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build();

    let (pk, vk): (SP1ProvingKey, SP1VerifyingKey);
    let program_elf: Vec<u8>;

    if is_aggregated_chunk {
        program_elf = ROOT_PROGRAM_ELF.to_vec();
        (pk, vk) = client.setup(ROOT_PROGRAM_ELF);
    } else {
        program_elf = CHUNK_PROGRAM_ELF.to_vec();
        (pk, vk) = client.setup(CHUNK_PROGRAM_ELF);
    }

    let proof_builder = client.prove(&pk, &stdin);
    let proof_builder = if should_wrap_to_groth16 {
        proof_builder.groth16()
    } else {
        proof_builder.compressed()
    };

    let proof = proof_builder
        .run()
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk)
        .map_err(SP1AggregationError::Verification)?;

    let proof_and_elf = SP1ProofWithPubValuesAndElf {
        proof_with_pub_values: proof,
        elf: program_elf,
    };

    Ok(proof_and_elf)
}

#[derive(Debug)]
pub enum AlignedSP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
    UnsupportedProof,
}

pub(crate) fn verify(
    sp1_proof_with_pub_values_and_elf: &SP1ProofWithPubValuesAndElf,
) -> Result<(), AlignedSP1VerificationError> {
    let client = &*SP1_PROVER_CLIENT;

    let (_pk, vk) = client.setup(&sp1_proof_with_pub_values_and_elf.elf);

    // only sp1 compressed proofs are supported for aggregation now
    match sp1_proof_with_pub_values_and_elf
        .proof_with_pub_values
        .proof
    {
        sp1_sdk::SP1Proof::Compressed(_) => client
            .verify(
                &sp1_proof_with_pub_values_and_elf.proof_with_pub_values,
                &vk,
            )
            .map_err(AlignedSP1VerificationError::Verification),
        _ => Err(AlignedSP1VerificationError::UnsupportedProof),
    }
}

pub fn vk_from_elf(elf: &[u8]) -> SP1VerifyingKey {
    let prover = &*SP1_PROVER_CLIENT;
    let (_, vk) = prover.setup(elf);
    vk
}
