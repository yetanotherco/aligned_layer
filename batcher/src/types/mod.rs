use lambdaworks_crypto::merkle_tree::{proof::Proof, traits::IsMerkleTreeBackend};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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
    pub proof_generator_addr: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VerificationBatch(Vec<VerificationData>);

#[derive(Debug, Default)]
pub struct VerificationDataCommitment {
    pub proof_commitment: [u8; 32],
    pub pub_input_commitment: [u8; 32],
    // This could be either the VM code (ELF, bytecode) or the verification key
    // depending on the proving system.
    pub proving_system_aux_data_commitment: [u8; 32],
    pub proof_generator_addr: [u8; 20],
}

#[derive(Default)]
pub struct VerificationCommitmentBatch(Vec<VerificationDataCommitment>);

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
        let mut hasher1 = Keccak256::new();
        hasher1.update(vec![1u8]);
        let child_1 = hasher1.finalize().into();

        let mut hasher2 = Keccak256::new();
        hasher2.update(vec![2u8]);
        let child_2 = hasher2.finalize().into();

        let parent = VerificationCommitmentBatch::hash_new_parent(&child_1, &child_2);

        // This value is built using Openzeppelin's module for Merkle Trees, in particular using
        // the SimpleMerkleTree. For more details see the openzeppelin_merkle_tree/merkle_tree.js script.
        let expected_parent = "71d8979cbfae9b197a4fbcc7d387b1fae9560e2f284d30b4e90c80f6bc074f57";

        assert_eq!(hex::encode(parent), expected_parent)
    }
}
