use alloy_primitives::Address;
use lambdaworks_crypto::merkle_tree::{proof::Proof, traits::IsMerkleTreeBackend};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    #[default]
    SP1,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub pub_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>,
    pub proof_generator_addr: Address,
}

#[derive(Debug, Default)]
pub struct VerificationDataCommitment {
    pub proof_commitment: [u8; 32],
    pub pub_input_commitment: [u8; 32],
    // This could be either the VM code (ELF, bytecode) or the verification key
    // depending on the proving system.
    pub proving_system_aux_data_commitment: [u8; 32],
    pub proof_generator_addr: [u8; 20],
}

impl From<&VerificationData> for VerificationDataCommitment {
    fn from(verification_data: &VerificationData) -> Self {
        let mut hasher = Keccak256::new();

        // compute proof commitment
        hasher.update(verification_data.proof.as_slice());
        let proof_commitment = hasher.finalize_reset().into();

        // compute public input commitment
        let mut pub_input_commitment = [0u8; 32];
        if let Some(pub_input) = &verification_data.pub_input {
            hasher.update(pub_input);
            pub_input_commitment = hasher.finalize_reset().into();
        }

        // compute proving system auxiliary data commitment
        let mut proving_system_aux_data_commitment = [0u8; 32];
        // FIXME(marian): This should probably be reworked, for the moment when the proving
        // system is SP1, `proving_system_aux_data` stands for the compiled ELF, while in the case
        // of Groth16 and PLONK, stands for the verification key.
        if let Some(vm_program_code) = &verification_data.vm_program_code {
            debug_assert_eq!(verification_data.proving_system, ProvingSystemId::SP1);
            hasher.update(vm_program_code);
            proving_system_aux_data_commitment = hasher.finalize_reset().into();
        } else if let Some(verification_key) = &verification_data.verification_key {
            hasher.update(verification_key);
            proving_system_aux_data_commitment = hasher.finalize_reset().into();
        }

        // serialize proof generator address to bytes
        let proof_generator_addr = verification_data.proof_generator_addr.into();

        VerificationDataCommitment {
            proof_commitment,
            pub_input_commitment,
            proving_system_aux_data_commitment,
            proof_generator_addr,
        }
    }
}

#[derive(Default)]
pub struct VerificationCommitmentBatch(pub Vec<VerificationDataCommitment>);
impl From<&Vec<VerificationData>> for VerificationCommitmentBatch {
    fn from(verification_data_batch: &Vec<VerificationData>) -> Self {
        VerificationCommitmentBatch(verification_data_batch.iter().map(|vd| vd.into()).collect())
    }
}

/// BatchInclusionData is the information that is retrieved to the clients once
/// the verification data sent by them has been processed.
pub struct BatchInclusionData {
    pub verification_data_commitment: VerificationDataCommitment,
    pub batch_merkle_root: [u8; 32],
    pub batch_inclusion_proof: Proof<[u8; 32]>,
}

impl IsMerkleTreeBackend for VerificationCommitmentBatch {
    type Node = [u8; 32];
    type Data = VerificationDataCommitment;

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(leaf.proof_commitment);
        hasher.update(leaf.pub_input_commitment);
        hasher.update(leaf.proving_system_aux_data_commitment);
        hasher.update(leaf.pub_input_commitment);
        hasher.update(leaf.proof_generator_addr);

        hasher.finalize().into()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_new_parent_is_correct() {
        let mut hasher = Keccak256::new();
        hasher.update(vec![1u8]);
        let child_1 = hasher.finalize_reset().into();
        hasher.update(vec![2u8]);
        let child_2 = hasher.finalize().into();

        let parent = VerificationCommitmentBatch::hash_new_parent(&child_1, &child_2);

        // This value is built using Openzeppelin's module for Merkle Trees, in particular using
        // the SimpleMerkleTree. For more details see the openzeppelin_merkle_tree/merkle_tree.js script.
        let expected_parent = "71d8979cbfae9b197a4fbcc7d387b1fae9560e2f284d30b4e90c80f6bc074f57";

        assert_eq!(hex::encode(parent), expected_parent)
    }
}
