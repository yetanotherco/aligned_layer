use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use aligned_sdk::core::{
    constants::INSTANT_MAX_FEE_BATCH_SIZE,
    types::{AlignedVerificationData, Signer, SigningKey, VerificationData},
};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use primitive_types::{H160, U256};
use rand::Rng;
use sp1_sdk::SP1Stdin;
use types::{Transfer, UserState};

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

    pub fn upsert(&mut self, address: H160, new_state: UserState) {
        self.user_states.insert(address, new_state);
    }
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

const PROGRAM_ELF: &[u8] = include_bytes!("../zkvm_programs/sp1/elf/sp1_state_transition_program");
fn prove_state_transition(
    db: &mut DB,
    transfers: Vec<Transfer>,
) -> (sp1_sdk::SP1ProofWithPublicValues, sp1_sdk::SP1VerifyingKey) {
    let mut stdin = SP1Stdin::new();
    let program_input = sp1_state_transition_program::ProgramInput {
        transfers,
        user_states: db.user_states.clone(),
    };
    stdin.write(&program_input);

    let prover = sp1_sdk::ProverClient::from_env();
    let (pk, vk) = prover.setup(PROGRAM_ELF);
    let proof = prover
        .prove(&pk, &stdin)
        .groth16()
        .run()
        .expect("Prover to run fine");

    (proof, vk)
}

async fn send_proof_to_be_verified_on_aligned(
    proof: sp1_sdk::SP1ProofWithPublicValues,
    network: aligned_sdk::core::types::Network,
    wallet: aligned_sdk::core::types::Wallet<SigningKey>,
) -> AlignedVerificationData {
    let proof = bincode::serialize(&proof).expect("Serialize sp1 proof to binary");
    let vm_program_code = PROGRAM_ELF.to_vec();

    let verification_data = VerificationData {
        proof_generator_addr: H160::default(),
        proving_system: aligned_sdk::core::types::ProvingSystemId::SP1,
        proof,
        vm_program_code: Some(vm_program_code),
        pub_input: None,
        verification_key: None,
    };

    let nonce = aligned_sdk::sdk::get_nonce_from_batcher(network.clone(), wallet.address())
        .await
        .expect("Retrieve nonce from aligned batcher");

    let aligned_verification_data = aligned_sdk::sdk::submit(
        network,
        &verification_data,
        U256::from(INSTANT_MAX_FEE_BATCH_SIZE as u64),
        wallet,
        nonce,
    )
    .await
    .expect("Proof to be sent");

    aligned_verification_data
}

async fn start_l2(
    network: aligned_sdk::core::types::Network,
    wallet: aligned_sdk::core::types::Wallet<SigningKey>,
) {
    // 0. Load merkle tree file, if not created, create initial state
    let mut db = DB::new("./db".to_string()).expect("create db");

    // 1. Create random transfers
    let account_updates = generate_random_transfers(&db, 10);

    // 2. Call zkvm and pass (MerkleTree, Updates to perform)
    let (proof, vk) = prove_state_transition(&mut db, account_updates);

    // 3. If the proving went alright, update the db and verify that the merkle root match

    // Fow now, in order for a proof to be aggregated, we first need to submit it via the fast mode or verification layer
    // Let's suppose that our L2 would run the prover once every 24hs and submit it on aligned
    // Once aligned aggregates the proof we will be notified and we'll send the new state commitment on chain

    // 4. Send the proof to aligned and wait for verification
    send_proof_to_be_verified_on_aligned(proof, network, wallet).await;

    // 5. Wait until proof is aggregated
    // ...
    // 6. Send updateState transaction to Ethereum
    // let receipt = update_state_on_chain();
}
