use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use lambdaworks_crypto::merkle_tree::{merkle::MerkleTree, traits::IsMerkleTreeBackend};
use primitive_types::{H160, U256};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Clone, Default, Serialize, Deserialize)]
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
        hasher.update(&balance_bytes);
        hasher.update(&nonce_bytes);
        hasher.finalize().into()
    }

    fn hash_new_parent(child_1: &Self::Node, child_2: &Self::Node) -> Self::Node {
        let mut hasher = Keccak256::new();
        hasher.update(child_1);
        hasher.update(child_2);
        hasher.finalize().into()
    }
}

struct DB {
    pub user_states: HashMap<H160, UserState>,
    pub root: [u8; 32],
    pub file_path: String,
}

#[derive(Debug)]
pub enum DBError {
    IO(String),
}

impl DB {
    pub fn new(file_path: String) -> Result<Self, DBError> {
        let file = File::open(&file_path).map_err(|e| DBError::IO(e.to_string()))?;
        let reader = BufReader::new(file);
        let user_states: Vec<UserState> =
            serde_json::from_reader(reader).map_err(|e| DBError::IO(e.to_string()))?;
        let root = MerkleTree::<UserState>::build(&user_states).unwrap().root;

        let mut user_states_map: HashMap<H160, UserState> = HashMap::new();
        for state in user_states {
            user_states_map.insert(state.address, state);
        }

        let db = Self {
            user_states: user_states_map,
            root,
            file_path,
        };

        Ok(db)
    }

    pub fn save(&self) -> Result<(), DBError> {
        let file = File::create(&self.file_path).map_err(|e| DBError::IO(e.to_string()))?;
        let writer = BufWriter::new(file);
        let values: Vec<UserState> = self.user_states.clone().into_values().collect();
        serde_json::to_writer(writer, &values).map_err(|e| DBError::IO(e.to_string()))?;

        Ok(())
    }

    pub fn commitment(&self) -> [u8; 32] {
        let values: Vec<UserState> = self.user_states.clone().into_values().collect();
        let root = MerkleTree::<UserState>::build(&values).unwrap().root;
        root
    }

    fn initial_state() -> Vec<UserState> {
        vec![]
    }

    pub fn upsert(&mut self, address: H160, newState: UserState) {
        self.user_states.insert(address, newState);
    }
}

struct Transfer {
    pub from: H160,
    pub to: H160,
    pub amount: U256,
}

fn generate_random_transfers(db: &DB, num_to_generate: usize) -> Vec<Transfer> {
    let mut transfers = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..num_to_generate {
        let accounts: Vec<&UserState> = db.user_states.values().collect();

        let sender = accounts
            .get(rng.gen_range(0..db.user_states.len()))
            .cloned()
            .unwrap();

        let receiver = accounts
            .get(rng.gen_range(0..db.user_states.len()))
            .cloned()
            .unwrap();

        let transfer = Transfer {
            amount: sender.balance / 2,
            from: sender.address,
            to: receiver.address,
        };

        transfers.push(transfer);
    }

    transfers
}

fn prove_state_transition(db: &mut DB, transfers: Vec<Transfer>) {}

fn main() {
    // 0. Load merkle tree file, if not created, create initial state
    let mut db = DB::new("./db".to_string()).expect("create db");

    // 1. Create random transfers
    let account_updates = generate_random_transfers(&db, 10);

    // 2. Call zkvm and pass (MerkleTree, Updates to perform)
    let proof = prove_state_transition(&mut db, account_updates);

    // Fow now, in order for a proof to be aggregated, we first need to submit it via the fast mode or verification layer
    // Let's suppose that our L2 would run the prover once every 24hs and submit it on aligned
    // Once aligned aggregates the proof we will be notified and we'll send the new state commitment on chain

    // 4. Send the proof to aligned and wait for verification
    // let response = send_proof_to_be_verified_on_aligned(proof);
    // 5. Wait until proof is aggregated
    // ...
    // 6. Send updateState transaction to Ethereum
    // let receipt = update_state_on_chain();
}
