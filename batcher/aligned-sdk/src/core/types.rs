use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::signers::Signer;
use ethers::signers::Wallet;
use ethers::types::transaction::eip712::EIP712Domain;
use ethers::types::transaction::eip712::Eip712;
use ethers::types::transaction::eip712::Eip712Error;
use ethers::types::Address;
use ethers::types::Signature;
use ethers::types::U256;
use lambdaworks_crypto::merkle_tree::{
    merkle::MerkleTree, proof::Proof, traits::IsMerkleTreeBackend,
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

use super::errors::VerifySignatureError;

// VerificationData is a bytes32 instead of a VerificationData struct because in the BatcherPaymentService contract
// we don't have the fields of VerificationData, we only have the hash of the VerificationData.
// chain_id is not included in the type because it is now part of the domain.
const NONCED_VERIFICATION_DATA_TYPE: &[u8] =
    b"NoncedVerificationData(bytes32 verification_data_hash,uint256 nonce,uint256 max_fee)";

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq, Copy)]
#[repr(u8)]
pub enum ProvingSystemId {
    GnarkPlonkBls12_381,
    GnarkPlonkBn254,
    Groth16Bn254,
    #[default]
    SP1,
    Risc0,
}

impl Display for ProvingSystemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProvingSystemId::GnarkPlonkBls12_381 => write!(f, "GnarkPlonkBls12_381"),
            ProvingSystemId::GnarkPlonkBn254 => write!(f, "GnarkPlonkBn254"),
            ProvingSystemId::Groth16Bn254 => write!(f, "Groth16Bn254"),
            ProvingSystemId::SP1 => write!(f, "SP1"),
            ProvingSystemId::Risc0 => write!(f, "Risc0"),
        }
    }
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
    pub nonce: U256,
    pub max_fee: U256,
    pub chain_id: U256,
    pub payment_service_addr: Address,
}

impl NoncedVerificationData {
    pub fn new(
        verification_data: VerificationData,
        nonce: U256,
        max_fee: U256,
        chain_id: U256,
        payment_service_addr: Address,
    ) -> Self {
        Self {
            verification_data,
            nonce,
            max_fee,
            chain_id,
            payment_service_addr,
        }
    }
}

// Defines an estimate price preference for the user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PriceEstimate {
    Min,
    Default,
    Instant,
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

        // Compute proof commitment

        hasher.update(verification_data.proof.as_slice());
        let proof_commitment = hasher.finalize_reset().into();

        // Compute public input commitment

        let mut pub_input_commitment = [0u8; 32];
        if let Some(pub_input) = &verification_data.pub_input {
            hasher.update(pub_input);
            pub_input_commitment = hasher.finalize_reset().into();
        }

        // Compute proving system auxiliary data commitment

        // FIXME(marian): This should probably be reworked, for the moment when the proving
        // system is SP1 or Risc0, `proving_system_aux_data` stands for information related to the
        // compiled ELF, while in the rest of the proving systems, stands for the verification key.
        let proving_system_byte = verification_data.proving_system as u8;
        let proving_system_aux_data_commitment =
            if let Some(vm_program_code) = &verification_data.vm_program_code {
                hasher.update(vm_program_code);
                hasher.update([proving_system_byte]);
                hasher.finalize_reset().into()
            } else if let Some(verification_key) = &verification_data.verification_key {
                hasher.update(verification_key);
                hasher.update([proving_system_byte]);
                hasher.finalize_reset().into()
            } else {
                [0u8; 32]
            };

        // Serialize proof generator address to bytes

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
        Ok(EIP712Domain {
            name: Some("Aligned".into()),
            version: Some("1".into()),
            chain_id: Some(self.chain_id),
            verifying_contract: Some(self.payment_service_addr),
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        let mut hasher = Keccak256::new();
        hasher.update(NONCED_VERIFICATION_DATA_TYPE);
        Ok(hasher.finalize().into())
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        //EIP requires big endian for u256
        let mut nonce_bytes = [0u8; 32];
        self.nonce.to_big_endian(&mut nonce_bytes);

        let mut max_fee_bytes = [0u8; 32];
        self.max_fee.to_big_endian(&mut max_fee_bytes);

        // This hashes the data of the task the user wants solved
        // This is the data that is the leaf on the batch merkle tree
        let verification_data_hash =
            VerificationCommitmentBatch::hash_data(&self.verification_data.clone().into());

        let mut hasher = Keccak256::new();

        // hashStruct(s : 𝕊) = keccak256(typeHash ‖ encodeData(s))

        // We first generate the type hash
        hasher.update(NONCED_VERIFICATION_DATA_TYPE);
        let type_hash = hasher.finalize_reset();

        // Then we hash it with the rest of the data in the struct
        hasher.update(type_hash);
        hasher.update(verification_data_hash);
        hasher.update(nonce_bytes);
        hasher.update(max_fee_bytes);
        let hash_struct = hasher.finalize_reset();

        Ok(hash_struct.into())
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
    pub fn verify_signature(&self) -> Result<Address, VerifySignatureError> {
        // Recovers the address from the signed data
        let recovered = self.signature.recover_typed_data(&self.verification_data)?;

        let hashed_data = self.verification_data.encode_eip712()?;

        self.signature.verify(hashed_data, recovered)?;
        Ok(recovered)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    InvalidProof(ProofInvalidReason),
    InvalidMaxFee,
    InvalidReplacementMessage,
    AddToBatchError,
    ProofTooLarge,
    InsufficientBalance(Address),
    EthRpcError,
    InvalidPaymentServiceAddress(Address, Address),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProofInvalidReason {
    RejectedProof,
    VerifierNotSupported,
    DisabledVerifier(ProvingSystemId),
}

impl Display for ValidityResponseMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ValidityResponseMessage::Valid => write!(f, "Valid"),
            ValidityResponseMessage::InvalidNonce => write!(f, "Invalid nonce"),
            ValidityResponseMessage::InvalidSignature => write!(f, "Invalid signature"),
            ValidityResponseMessage::InvalidChainId => write!(f, "Invalid chain id"),
            ValidityResponseMessage::InvalidProof(reason) => {
                write!(f, "Invalid proof: {}", reason)
            }
            ValidityResponseMessage::InvalidMaxFee => write!(f, "Invalid max fee"),
            ValidityResponseMessage::InvalidReplacementMessage => {
                write!(f, "Invalid replacement message")
            }
            ValidityResponseMessage::AddToBatchError => write!(f, "Add to batch error"),
            ValidityResponseMessage::ProofTooLarge => write!(f, "Proof too large"),
            ValidityResponseMessage::InsufficientBalance(addr) => {
                write!(f, "Insufficient balance for address {}", addr)
            }
            ValidityResponseMessage::EthRpcError => write!(f, "Eth RPC error"),
            ValidityResponseMessage::InvalidPaymentServiceAddress(addr, expected) => {
                write!(
                    f,
                    "Invalid payment service address: {}, expected: {}",
                    addr, expected
                )
            }
        }
    }
}

impl Display for ProofInvalidReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ProofInvalidReason::VerifierNotSupported => write!(f, "Verifier not supported"),
            ProofInvalidReason::DisabledVerifier(proving_system_id) => {
                write!(f, "Disabled verifier: {}", proving_system_id)
            }
            ProofInvalidReason::RejectedProof => write!(f, "Proof did not verify"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseMessage {
    BatchInclusionData(BatchInclusionData),
    ProtocolVersion(u16),
    CreateNewTaskError(String),
    InvalidProof(ProofInvalidReason),
    BatchReset,
    Error(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Network {
    Devnet,
    Holesky,
    HoleskyStage,
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "holesky" => Ok(Network::Holesky),
            "holesky-stage" => Ok(Network::HoleskyStage),
            "devnet" => Ok(Network::Devnet),
            _ => Err(
                "Invalid network, possible values are: \"holesky\", \"holesky-stage\", \"devnet\""
                    .to_string(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use ethers::signers::LocalWallet;
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn eip_712_recovers_same_address_as_signed() {
        const ANVIL_PRIVATE_KEY: &str =
            "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9
        let wallet = LocalWallet::from_str(ANVIL_PRIVATE_KEY).expect("Failed to create wallet");

        let proof = [42, 42, 42, 42].to_vec();
        let pub_input = Some([32, 32, 32, 32].to_vec());
        let verification_key = Some([8, 8, 8, 8].to_vec());
        let proving_system = ProvingSystemId::Groth16Bn254;

        let verification_data = VerificationData {
            proving_system,
            proof,
            pub_input,
            verification_key,
            vm_program_code: None,
            proof_generator_addr: wallet.address(),
        };

        let nonced_verification_data = NoncedVerificationData::new(
            verification_data,
            1.into(),
            2.into(),
            3.into(),
            wallet.address(),
        );

        let signed_data = wallet
            .sign_typed_data(&nonced_verification_data)
            .await
            .unwrap();

        let recovered_address = signed_data
            .recover_typed_data(&nonced_verification_data)
            .unwrap();

        assert_eq!(recovered_address, wallet.address())
    }
}
