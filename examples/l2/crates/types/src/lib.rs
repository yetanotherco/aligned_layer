use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use primitive_types::{H160, U256};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserState {
    pub address: H160,
    pub balance: U256,
    pub nonce: U256,
}

impl IsMerkleTreeBackend for UserState {
    type Node = [u8; 32];
    type Data = UserState;

    fn hash_data(leaf: &Self::Data) -> Self::Node {
        let mut hasher = Keccak256::new();

        let mut balance_bytes: [u8; 32] = [0u8; 32];
        let mut nonce_bytes: [u8; 32] = [0u8; 32];
        leaf.balance.to_little_endian(&mut balance_bytes);
        leaf.nonce.to_little_endian(&mut nonce_bytes);

        hasher.update(leaf.address);
        hasher.update(balance_bytes);
        hasher.update(nonce_bytes);
        hasher.finalize().into()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transfer {
    pub from: H160,
    pub to: H160,
    pub amount: U256,
}
