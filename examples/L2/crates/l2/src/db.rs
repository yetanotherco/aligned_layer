use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter},
    str::FromStr,
};

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use primitive_types::{H160, U256};
use rand::Rng;
use tracing::warn;
use types::{Transfer, UserState};

pub struct DB {
    pub user_states: BTreeMap<H160, UserState>,
    pub file_path: String,
}

#[derive(Debug)]
pub enum DBError {
    #[allow(dead_code)]
    IO(String),
}

impl DB {
    pub fn new(file_path: String) -> Self {
        match Self::new_from_file(file_path.clone()) {
            Ok(db) => db,
            Err(e) => {
                warn!("Error when loading db from file {:?}, will start a new db with a default initial state", e);
                // if db does not exists, create one with initial state
                let initial_state = Self::initial_state();
                let mut user_states: BTreeMap<H160, UserState> = BTreeMap::new();
                for state in initial_state {
                    user_states.insert(state.address, state);
                }

                DB {
                    user_states,
                    file_path,
                }
            }
        }
    }

    fn new_from_file(file_path: String) -> Result<Self, DBError> {
        let file = File::open(&file_path).map_err(|e| DBError::IO(e.to_string()))?;
        let reader = BufReader::new(file);
        let user_states: Vec<UserState> =
            serde_json::from_reader(reader).map_err(|e| DBError::IO(e.to_string()))?;

        let mut user_states_map: BTreeMap<H160, UserState> = BTreeMap::new();
        for state in user_states {
            user_states_map.insert(state.address, state);
        }

        let db = Self {
            user_states: user_states_map,
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
        MerkleTree::<UserState>::build(&values).unwrap().root
    }

    /// Db genesis state used if a file is not provided
    /// Its commitment is: 0x3c1d1c01f8e0a4533085bc9d8a3829c5f6872e6d6cf62e04ae71acbc803747ce
    fn initial_state() -> Vec<UserState> {
        vec![
            UserState {
                address: H160::from_str("0x742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap(),
                balance: U256::from_dec_str("100000000000000000000").unwrap(),
                nonce: U256::from(0),
            },
            UserState {
                address: H160::from_str("0x53d284357ec70cE289D6D64134DfAc8E511c8a3D").unwrap(),
                balance: U256::from_dec_str("50000000000000000000").unwrap(),
                nonce: U256::from(0),
            },
            UserState {
                address: H160::from_str("0xfe9e8709d3215310075d67e3ed32a380ccf451c8").unwrap(),
                balance: U256::from_dec_str("250000000000000000000").unwrap(),
                nonce: U256::from(0),
            },
            UserState {
                address: H160::from_str("0xab5801a7d398351b8be11c439e05c5b3259aec9b").unwrap(),
                balance: U256::from_dec_str("75000000000000000000").unwrap(),
                nonce: U256::from(0),
            },
        ]
    }
}

pub fn generate_random_transfers(db: &DB, num_to_generate: usize) -> Vec<Transfer> {
    let mut transfers = vec![];
    let mut rng = rand::thread_rng();

    let mut accounts: Vec<UserState> = db.user_states.clone().into_values().collect();

    for _ in 0..num_to_generate {
        let (from, amount) = {
            let user = accounts
                .get_mut(rng.gen_range(0..db.user_states.len()))
                .unwrap();
            let new_balance = user.balance / 2;
            user.balance = new_balance;

            (user.address, new_balance)
        };

        let to = accounts
            .get(rng.gen_range(0..db.user_states.len()))
            .unwrap()
            .address;

        let transfer = Transfer { amount, from, to };

        transfers.push(transfer);
    }

    transfers
}
