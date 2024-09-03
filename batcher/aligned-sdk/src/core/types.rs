use std::str::FromStr;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::Signer;
use ethers::signers::Wallet;
use ethers::types::transaction::eip712::EIP712Domain;
use ethers::types::transaction::eip712::Eip712;
use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::Address;
use ethers::types::Signature;
use ethers::types::SignatureError;
use ethers::types::H160;
use ethers::types::U256;
use lambdaworks_crypto::merkle_tree::{
    merkle::MerkleTree, proof::Proof, traits::IsMerkleTreeBackend,
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

const ANVIL_CHAIN_ID: u64 = 31337;
const HOLESKY_CHAIN_ID: u64 = 17000;
const MAINNET_CHAIN_ID: u64 = 1;
// VerificationData is a bytes32 instead of a VerificationData struct because in the BatcherPaymentService contract
// we don't have the fields of VerificationData, we only have the hash of the VerificationData.
// chain_id is not included in the type because it is now part of the domain.
const NONCED_VERIFICATION_DATA_TYPE: &[u8] =
    b"NoncedVerificationData(bytes32 verification_data_hash,bytes32 nonce)";

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    #[default]
    SP1,
    Halo2KZG,
    Halo2IPA,
    Risc0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub pub_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>,
    pub proof_generator_addr: Address,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoncedVerificationData {
    pub verification_data: VerificationData,
    pub nonce: [u8; 32],
    pub chain_id: U256,
}

impl NoncedVerificationData {
    pub fn new(verification_data: VerificationData, nonce: [u8; 32], chain_id: U256) -> Self {
        Self {
            verification_data,
            nonce,
            chain_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VerificationDataCommitment {
    pub proof_commitment: [u8; 32],
    pub pub_input_commitment: [u8; 32],
    // This could be either the VM code (ELF, bytecode) or the verification key
    // depending on the proving system.
    pub proving_system_aux_data_commitment: [u8; 32],
    pub proof_generator_addr: [u8; 20],
}

impl From<&NoncedVerificationData> for VerificationDataCommitment {
    fn from(nonced_verification_data: &NoncedVerificationData) -> Self {
        nonced_verification_data.verification_data.clone().into()
    }
}

impl From<VerificationData> for VerificationDataCommitment {
    fn from(verification_data: VerificationData) -> Self {
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

impl From<NoncedVerificationData> for VerificationDataCommitment {
    fn from(nonced_verification_data: NoncedVerificationData) -> Self {
        VerificationDataCommitment::from(&nonced_verification_data)
    }
}

#[derive(Clone, Default)]
pub struct VerificationCommitmentBatch;

impl IsMerkleTreeBackend for VerificationCommitmentBatch {
    type Node = [u8; 32];
    type Data = VerificationDataCommitment;

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(leaf.proof_commitment);
        hasher.update(leaf.pub_input_commitment);
        hasher.update(leaf.proving_system_aux_data_commitment);
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

/// BatchInclusionData is the information that is retrieved to the clients once
/// the verification data sent by them has been processed by Aligned.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchInclusionData {
    pub batch_merkle_root: [u8; 32],
    pub batch_inclusion_proof: Proof<[u8; 32]>,
    pub index_in_batch: usize,
}

impl BatchInclusionData {
    pub fn new(
        verification_data_batch_index: usize,
        batch_merkle_tree: &MerkleTree<VerificationCommitmentBatch>,
    ) -> Self {
        let batch_inclusion_proof = batch_merkle_tree
            .get_proof_by_pos(verification_data_batch_index)
            .unwrap();

        BatchInclusionData {
            batch_merkle_root: batch_merkle_tree.root,
            batch_inclusion_proof,
            index_in_batch: verification_data_batch_index,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMessage {
    pub verification_data: NoncedVerificationData,
    pub signature: Signature,
}

impl Eip712 for NoncedVerificationData {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        let payment_service_addr = get_payment_service_addr(self.chain_id);

        Ok(EIP712Domain {
            name: Some("Aligned".into()),
            version: Some("1".into()),
            chain_id: Some(self.chain_id),
            verifying_contract: payment_service_addr,
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        let mut hasher = Keccak256::new();
        hasher.update(NONCED_VERIFICATION_DATA_TYPE);
        Ok(hasher.finalize().into())
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let verification_data_hash =
            VerificationCommitmentBatch::hash_data(&self.verification_data.clone().into());

        let mut hasher = Keccak256::new();

        hasher.update(NONCED_VERIFICATION_DATA_TYPE);
        let nonced_verification_data_type_hash = hasher.finalize_reset();

        hasher.update(self.nonce);
        let nonce_hash = hasher.finalize_reset();

        hasher.update(nonced_verification_data_type_hash.as_slice());
        hasher.update(verification_data_hash.as_slice());
        hasher.update(nonce_hash.as_slice());

        Ok(hasher.finalize().into())
    }
}

impl ClientMessage {
    /// Client message is a wrap around verification data and its signature.
    /// The signature is obtained by calculating the commitments and then hashing them.
    pub async fn new(
        verification_data: NoncedVerificationData,
        wallet: Wallet<SigningKey>,
    ) -> Self {
        let signature = wallet
            .sign_typed_data(&verification_data)
            .await
            .expect("Failed to sign the verification data");

        ClientMessage {
            verification_data,
            signature,
        }
    }

    /// The signature of the message is verified, and when it correct, the
    /// recovered address from the signature is returned.
    pub fn verify_signature(&self) -> Result<Address, SignatureError> {
        let recovered = self.signature.recover_typed_data(&self.verification_data)?;

        // We can expect here because encode_eip712 can only error if
        // struct_hash or domain_separator return an error, which is not possible
        let hashed_data = self
            .verification_data
            .encode_eip712()
            .expect("Failed to encode verification data for signature verification");

        self.signature.verify(hashed_data, recovered)?;
        Ok(recovered)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlignedVerificationData {
    pub verification_data_commitment: VerificationDataCommitment,
    pub batch_merkle_root: [u8; 32],
    pub batch_inclusion_proof: Proof<[u8; 32]>,
    pub index_in_batch: usize,
}

impl AlignedVerificationData {
    pub fn new(
        verification_data_commitment: &VerificationDataCommitment,
        inclusion_data: &BatchInclusionData,
    ) -> Self {
        let batch_merkle_root = inclusion_data.batch_merkle_root;
        let batch_inclusion_proof = &inclusion_data.batch_inclusion_proof;
        let index_in_batch = inclusion_data.index_in_batch;

        Self {
            verification_data_commitment: verification_data_commitment.clone(),
            batch_merkle_root,
            batch_inclusion_proof: batch_inclusion_proof.clone(),
            index_in_batch,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidityResponseMessage {
    Valid,
    InvalidNonce,
    InvalidSignature,
    InvalidChainId,
    InvalidProof,
    ProofTooLarge,
    InsufficientBalance(Address),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseMessage {
    BatchInclusionData(BatchInclusionData),
    ProtocolVersion(u16),
    CreateNewTaskError(String),
    BatchReset,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum Chain {
    Devnet,
    Holesky,
    HoleskyStage,
}

fn get_payment_service_addr(chain_id: U256) -> Option<H160> {
    match chain_id.as_u64() {
        ANVIL_CHAIN_ID => H160::from_str("0x7969c5eD335650692Bc04293B07F5BF2e7A673C0").ok(),
        HOLESKY_CHAIN_ID => H160::from_str("0x815aeCA64a974297942D2Bbf034ABEe22a38A003").ok(),
        MAINNET_CHAIN_ID => {
            //FIXME: Add the payment service address for mainnet
            None
        }
        _ => None,
    }
}
