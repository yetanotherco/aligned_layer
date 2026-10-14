use agg_mode_sdk::blockchain::AggregationModeProvingSystem;
use alloy::primitives::Keccak256;
use sp1_aggregation_program::SP1VkAndPubInputs;
#[cfg(feature = "prove")]
use sp1_sdk::env::EnvProver;
use sp1_sdk::{
    Elf, HashableKey, LightProver, ProveRequest, Prover, ProverClient, ProvingKey,
    SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use tokio::sync::OnceCell;

const CHUNK_PROGRAM_ELF: Elf = Elf::Static(include_bytes!(
    "../../aggregation_programs/sp1/elf/sp1_chunk_aggregator_program"
));

const USER_PROOFS_PROGRAM_ELF: Elf = Elf::Static(include_bytes!(
    "../../aggregation_programs/sp1/elf/sp1_user_proofs_aggregator_program"
));

#[cfg(feature = "prove")]
static SP1_PROVER_CLIENT: OnceCell<EnvProver> = OnceCell::const_new();

/// Separate verify-only client that always runs on the CPU.
/// This is used for verification, which is performed in parallel and
/// cannot be done on the GPU.
static SP1_VERIFIER_CLIENT: OnceCell<LightProver> = OnceCell::const_new();

#[cfg(feature = "prove")]
async fn prover_client() -> &'static EnvProver {
    SP1_PROVER_CLIENT.get_or_init(ProverClient::from_env).await
}

pub async fn verifier_client() -> &'static LightProver {
    SP1_VERIFIER_CLIENT
        .get_or_init(|| ProverClient::builder().light().build())
        .await
}

pub struct SP1ProofWithPubValuesAndVk {
    pub proof_with_pub_values: SP1ProofWithPublicValues,
    pub vk: SP1VerifyingKey,
}

#[derive(Debug)]
pub enum AlignedSP1VerificationError {
    Verification(sp1_sdk::SP1VerificationError),
    UnsupportedProof,
}

impl SP1ProofWithPubValuesAndVk {
    /// Constructs a new instance of the struct by verifying a given SP1 proof with its public values.
    pub fn new(
        proof_with_pub_values: SP1ProofWithPublicValues,
        vk: SP1VerifyingKey,
        client: &LightProver,
    ) -> Result<Self, AlignedSP1VerificationError> {
        // only sp1 compressed proofs are supported for aggregation now
        match proof_with_pub_values.proof {
            sp1_sdk::SP1Proof::Compressed(_) => client
                .verify(&proof_with_pub_values, &vk, None)
                .map_err(AlignedSP1VerificationError::Verification),
            _ => Err(AlignedSP1VerificationError::UnsupportedProof),
        }?;

        Ok(Self {
            proof_with_pub_values,
            vk,
        })
    }

    pub fn hash_vk_and_pub_inputs(&self) -> [u8; 32] {
        let mut hasher = Keccak256::new();
        hasher.update(AggregationModeProvingSystem::SP1.id_bytes());
        let vk_bytes = &self.vk.hash_bytes();
        hasher.update(vk_bytes);
        hasher.update(self.proof_with_pub_values.public_values.as_slice());
        hasher.finalize().into()
    }
}

#[derive(Debug)]
pub enum SP1AggregationError {
    Verification(sp1_sdk::SP1VerificationError),
    Setup(String),
    Prove(String),
    UnsupportedProof,
}

pub(crate) async fn run_user_proofs_aggregator(
    proofs: &[SP1ProofWithPubValuesAndVk],
) -> Result<SP1ProofWithPubValuesAndVk, SP1AggregationError> {
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
    let client = prover_client().await;
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build().await;

    let pk = client
        .setup(USER_PROOFS_PROGRAM_ELF)
        .await
        .map_err(|e| SP1AggregationError::Setup(e.to_string()))?;
    let vk = pk.verifying_key().clone();

    let proof = client
        .prove(&pk, stdin)
        .compressed()
        .await
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk, None)
        .map_err(SP1AggregationError::Verification)?;

    let proof_and_vk = SP1ProofWithPubValuesAndVk {
        proof_with_pub_values: proof,
        vk,
    };

    Ok(proof_and_vk)
}

pub(crate) async fn run_chunk_aggregator(
    proofs: &[(SP1ProofWithPubValuesAndVk, Vec<[u8; 32]>)],
) -> Result<SP1ProofWithPubValuesAndVk, SP1AggregationError> {
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
    let client = prover_client().await;
    // If not in prove mode, create a mock proof via mock client
    #[cfg(not(feature = "prove"))]
    let client = ProverClient::builder().mock().build().await;

    let pk = client
        .setup(CHUNK_PROGRAM_ELF)
        .await
        .map_err(|e| SP1AggregationError::Setup(e.to_string()))?;
    let vk = pk.verifying_key().clone();

    #[cfg(feature = "prove")]
    let proof = client
        .prove(&pk, stdin)
        .groth16()
        .await
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;
    #[cfg(not(feature = "prove"))]
    let proof = client
        .prove(&pk, stdin)
        // In mock mode, deferred proof verification must be disabled to avoid recursive proof verification.
        // This is because chunk proofs are mocked, and enabling verification would cause a panic.
        // See: https://docs.succinct.xyz/docs/sp1/writing-programs/proof-aggregation#proof-aggregation-in-mock-mode
        .deferred_proof_verification(false)
        .groth16()
        .await
        .map_err(|e| SP1AggregationError::Prove(e.to_string()))?;

    // a sanity check, vm already performs it
    client
        .verify(&proof, &vk, None)
        .map_err(SP1AggregationError::Verification)?;

    let proof_and_vk = SP1ProofWithPubValuesAndVk {
        proof_with_pub_values: proof,
        vk,
    };

    Ok(proof_and_vk)
}

pub async fn vk_from_elf(elf: Elf) -> Result<SP1VerifyingKey, SP1AggregationError> {
    let pk = verifier_client()
        .await
        .setup(elf)
        .await
        .map_err(|e| SP1AggregationError::Setup(e.to_string()))?;
    Ok(pk.verifying_key().clone())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts/test_files/sp1");

    fn read_fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{FIXTURES_DIR}/{name}")).expect("fixture to exist")
    }

    pub(crate) async fn load_fibonacci_proof() -> SP1ProofWithPubValuesAndVk {
        let proof = bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0.proof")).unwrap();
        let vk = bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0_vk.bin")).unwrap();
        SP1ProofWithPubValuesAndVk::new(proof, vk, verifier_client().await)
            .expect("fixture proof to verify")
    }

    #[tokio::test]
    async fn leaf_commitment_matches_aggregation_program_commitment() {
        let proof = load_fibonacci_proof().await;

        // This is the commitment the aggregation program computes for each leaf
        let program_leaf = SP1VkAndPubInputs {
            vk: proof.vk.hash_u32(),
            public_inputs: proof.proof_with_pub_values.public_values.to_vec(),
        }
        .commitment();

        assert_eq!(proof.hash_vk_and_pub_inputs(), program_leaf);
    }

    #[tokio::test]
    async fn rejects_proof_with_tampered_public_values() {
        let mut proof: SP1ProofWithPublicValues =
            bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0.proof")).unwrap();
        let vk = bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0_vk.bin")).unwrap();
        proof.public_values = sp1_sdk::SP1PublicValues::from(&[1, 2, 3]);

        let result = SP1ProofWithPubValuesAndVk::new(proof, vk, verifier_client().await);

        assert!(matches!(
            result,
            Err(AlignedSP1VerificationError::Verification(_))
        ));
    }
}
