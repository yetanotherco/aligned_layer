use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use primitive_types::H160;
use rand::Rng;
use types::{Transfer, UserState};

pub struct DB {
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

    pub fn upsert(&mut self, address: H160, new_state: UserState) {
        self.user_states.insert(address, new_state);
    }
}

pub fn generate_random_transfers(db: &DB, num_to_generate: usize) -> Vec<Transfer> {
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
