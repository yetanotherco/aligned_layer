use aligned::{send_proof_to_be_verified_on_aligned, wait_until_proof_is_aggregated};
use db::{generate_random_transfers, DB};
use eth::send_state_transition_to_chain;
use primitive_types::U256;
use prover::{prove_state_transition, PROGRAM_ELF};
use sp1_state_transition_program::ProgramOutput;
use tracing::info;

mod aligned;
mod db;
mod eth;
mod prover;

pub struct Config {
    pub network: aligned_sdk::common::types::Network,
    pub eth_rpc_url: String,
    pub ws_eth_rpc_url: String,
    pub beacon_client_url: String,
    pub private_key_store_path: String,
    pub private_key_store_password: String,
    pub state_transition_contract_address: String,
}

pub async fn start_l2(config: Config) {
    // 0. Load merkle tree file, if not created, create initial state
    let mut db = DB::new("./db".to_string());

    loop {
        // 1. Create random transfers
        let transfers = generate_random_transfers(&db, 10);

        // 2. Call zkvm and transfer to perform and verify
        info!("Starting prover...");
        let (mut proof, vk) = prove_state_transition(&db, transfers.clone());
        let ProgramOutput {
            initial_state_merkle_root,
            post_state_merkle_root,
        } = proof.public_values.read::<ProgramOutput>();
        info!("Prover finish");

        // 3. If the proving went alright, update the db and verify that the merkle root matches
        assert!(db.commitment() == initial_state_merkle_root);
        // Note: we don't have to verify that the user has enough balance, as the prover already validates it
        for transfer in transfers {
            let mut user_from = db
                .user_states
                .get(&transfer.from)
                .expect("User must exist in state")
                .clone();

            let mut user_to = db
                .user_states
                .get(&transfer.to)
                .expect("User must exist in state")
                .clone();

            user_from.balance -= transfer.amount;
            user_from.nonce += U256::one();
            user_to.balance += transfer.amount;

            db.user_states.insert(transfer.from, user_from);
            db.user_states.insert(transfer.to, user_to);
        }
        assert!(db.commitment() == post_state_merkle_root);

        // Fow now, in order for a proof to be aggregated, we first need to submit it via the fast mode or verification layer
        // Let's suppose that our L2 would run the prover once every 24hs and submit it on aligned
        // Once aligned aggregates the proof we will be notified and we'll send the new state commitment on chain

        // 4. Send the proof to aligned and wait for verification
        info!("Sending proof to aligned batcher...");
        let _ = send_proof_to_be_verified_on_aligned(&config, &proof, PROGRAM_ELF.to_vec()).await;
        info!("Proof submitted");

        // 5. Wait until proof is aggregated
        info!("Waiting until proof is aggregated...");
        let merkle_path = wait_until_proof_is_aggregated(&config, &proof, &vk).await;
        info!("Proof has been aggregated on aligned, about to send update to chain...");

        // 6. Send updateState transaction to Ethereum
        let receipt =
            send_state_transition_to_chain(&config, proof.public_values.to_vec(), merkle_path)
                .await;

        info!(
            "State update in contracts tx hash: {:?}",
            receipt.transaction_hash
        );

        // 7. Finally save the db to a file to be retrieved later
        db.save().unwrap();
    }
}
