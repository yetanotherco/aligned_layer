use aligned::{send_proof_to_be_verified_on_aligned, wait_until_proof_is_aggregated};
use alloy::hex::hex;
use db::{generate_random_transfers, DB};
use eth::send_state_transition_to_chain;
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use primitive_types::U256;
use sp1_state_transition_program::ProgramOutput;
use zk::{prove_state_transition, PROGRAM_ELF};

mod aligned;
mod db;
mod eth;
mod zk;

pub async fn start_l2(
    network: aligned_sdk::core::types::Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    wallet: aligned_sdk::core::types::Wallet<aligned_sdk::core::types::SigningKey>,
) {
    // 0. Load merkle tree file, if not created, create initial state
    let mut db = DB::new("./db".to_string());

    // 1. Create random transfers
    let transfers = generate_random_transfers(&db, 10);

    // 2. Call zkvm and pass (MerkleTree, Updates to perform)
    println!("Staring prover");
    let (mut proof, vk) = prove_state_transition(&db, transfers.clone());
    let ProgramOutput {
        initial_state_merkle_root,
        post_state_merkle_root,
    } = proof.public_values.read::<ProgramOutput>();
    println!("Prover finish");

    // 3. If the proving went alright, update the db and verify that the merkle root matches
    assert!(db.commitment() == initial_state_merkle_root);
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
    println!("Sending proof to aligned batcher");
    let _ =
        send_proof_to_be_verified_on_aligned(&proof, PROGRAM_ELF.to_vec(), network.clone(), wallet)
            .await;
    println!("Proof submitted");

    println!("Waiting until is proof is aggregated");
    // 5. Wait until proof is aggregated
    let merkle_path = wait_until_proof_is_aggregated(
        network.clone(),
        eth_rpc_url.clone(),
        beacon_client_url,
        &proof,
        &vk,
    )
    .await
    .expect("To get merkle path in last aggregated proofs");

    println!("Proof has been aggregated on aligned, about to send update to chain.");

    // 6. Send updateState transaction to Ethereum
    let receipt = send_state_transition_to_chain(
        proof.public_values.to_vec(),
        merkle_path,
        eth_rpc_url.clone(),
        "0xdC62Bf47Da2a2afD8Eb838C7004337613Af5C3b9".into(),
        "".into(),
    )
    .await;

    println!("State update in contracts tx receipt {:?}", receipt);

    // Finally save the db to a file to be retrieved later
    db.save().unwrap();
}
