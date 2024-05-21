use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VerificationBatch(Vec<VerificationData>);

impl IsMerkleTreeBackend for VerificationBatch {
    type Node = Vec<u8>;
    type Data = VerificationData;

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        let leaf_bytes = bincode::serialize(leaf).expect("Failed to serialize leaf");
        let mut hasher = Keccak256::new();
        hasher.update(&leaf_bytes);
        hasher.finalize().to_vec()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().to_vec()
    }
}
