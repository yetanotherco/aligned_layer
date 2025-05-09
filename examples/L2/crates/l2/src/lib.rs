use aligned::{send_proof_to_be_verified_on_aligned, wait_until_proof_is_aggregated};
use db::{generate_random_transfers, DB};
use sp1_state_transition_program::ProgramOutput;
use zk::{prove_state_transition, PROGRAM_ELF};

mod aligned;
mod db;
mod zk;

async fn send_state_transition_to_chain() {}

pub async fn start_l2(
    network: aligned_sdk::core::types::Network,
    wallet: aligned_sdk::core::types::Wallet<aligned_sdk::core::types::SigningKey>,
) {
    // 0. Load merkle tree file, if not created, create initial state
    let mut db = DB::new("./db".to_string()).expect("create db");

    // 1. Create random transfers
    let account_updates = generate_random_transfers(&db, 10);

    // 2. Call zkvm and pass (MerkleTree, Updates to perform)
    let (mut proof, _vk) = prove_state_transition(&mut db, account_updates.clone());
    let ProgramOutput {
        initial_state_merkle_root,
        post_state_merkle_root,
    } = proof.public_values.read::<ProgramOutput>();

    // 3. If the proving went alright, update the db and verify that the merkle root matches
    assert!(db.commitment() == initial_state_merkle_root);
    for update in account_updates {
        db.user_states
            .get_mut(&update.from)
            .expect("User should exist")
            .balance -= update.amount;

        db.user_states
            .get_mut(&update.to)
            .expect("User should exist")
            .balance += update.amount;
    }
    assert!(db.commitment() == post_state_merkle_root);

    // Fow now, in order for a proof to be aggregated, we first need to submit it via the fast mode or verification layer
    // Let's suppose that our L2 would run the prover once every 24hs and submit it on aligned
    // Once aligned aggregates the proof we will be notified and we'll send the new state commitment on chain

    // 4. Send the proof to aligned and wait for verification
    let _ =
        send_proof_to_be_verified_on_aligned(&proof, PROGRAM_ELF.to_vec(), network, wallet).await;

    // 5. Wait until proof is aggregated
    wait_until_proof_is_aggregated().await;

    // 6. Send updateState transaction to Ethereum
    // let receipt = update_state_on_chain();

    // Finally save the db to a file to be retrieved later
    db.save().unwrap();
}
