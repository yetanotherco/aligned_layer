use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Serialize, Deserialize)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    SP1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub public_input: Vec<u8>,
    pub verification_key: Vec<u8>,
    pub quorum_numbers: Vec<u8>,
    pub quorum_threshold_percentages: Vec<u8>,
    pub fee: u64,
}

impl Task {
    pub(crate) fn hash(self) -> Vec<u8> {
        let mut hasher = Sha3_256::new();
        hasher.update(self.proof);
        hasher.update(self.public_input);
        hasher.update(self.verification_key);
        hasher.update(self.quorum_numbers);
        hasher.update(self.quorum_threshold_percentages);
        hasher.finalize().to_vec()
    }
}

// Used for hashing
impl AsRef<[u8]> for Task {
    fn as_ref(&self) -> &[u8] {
        self.proof.as_ref()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationResult {
    Success { hash: Vec<u8> },
    Failure,
}
