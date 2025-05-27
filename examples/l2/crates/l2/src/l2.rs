use crate::aligned::{check_proof_proof_aggregation_status, send_proof_to_be_verified_on_aligned};
use crate::config::Config;
use crate::db::{generate_random_transfers, DB};
use crate::eth::send_state_transition_to_chain;
use crate::prover::{self, prove_state_transition, PROGRAM_ELF};
use alloy::hex;
use primitive_types::U256;
use sp1_sdk::SP1ProofWithPublicValues;
use sp1_state_transition_program::ProgramOutput;
use tracing::info;

pub struct L2 {
    config: Config,
    db: DB,
}

impl L2 {
    pub fn new(config: Config) -> Self {
        let db_path = config.db_path.clone().unwrap_or("./db".to_string());

        Self {
            config,
            db: DB::new(db_path),
        }
    }

    pub async fn prove_state_transition_and_send_proof_to_aligned(
        &mut self,
    ) -> SP1ProofWithPublicValues {
        // 1. Create random transfers
        let transfers = generate_random_transfers(&self.db, 10);

        // 2. Call zkvm and transfer to perform and verify
        info!("Starting prover...");
        let (mut proof, _vk) = prove_state_transition(&self.db, transfers.clone());
        let ProgramOutput {
            initial_state_merkle_root,
            post_state_merkle_root,
        } = proof.public_values.read::<ProgramOutput>();
        info!("Prover finish");

        // 3. If the proving went alright, update the db and verify that the merkle root matches
        assert_eq!(self.db.commitment(), initial_state_merkle_root);
        // Note: we don't have to verify that the user has enough balance, as the prover already validates it
        for transfer in transfers {
            let mut user_from = self
                .db
                .user_states
                .get(&transfer.from)
                .expect("User must exist in state")
                .clone();

            let mut user_to = self
                .db
                .user_states
                .get(&transfer.to)
                .expect("User must exist in state")
                .clone();

            user_from.balance -= transfer.amount;
            user_from.nonce += U256::one();
            user_to.balance += transfer.amount;

            self.db.user_states.insert(transfer.from, user_from);
            self.db.user_states.insert(transfer.to, user_to);
        }
        assert_eq!(self.db.commitment(), post_state_merkle_root);

        // Fow now, in order for a proof to be aggregated, we first need to submit it via the fast mode or verification layer
        // Let's suppose that our L2 would run the prover once every 24hs and submit it on aligned
        // Once aligned aggregates the proof we will be notified and we'll send the new state commitment on chain

        // 4. Send the proof to aligned and wait for verification
        info!("Sending proof to aligned batcher...");
        let _ =
            send_proof_to_be_verified_on_aligned(&self.config, &proof, PROGRAM_ELF.to_vec()).await;
        info!("Proof submitted");

        self.db.save().unwrap();

        proof
    }

    pub async fn update_state_on_chain(&mut self, proof: SP1ProofWithPublicValues) {
        let vk = prover::vk_from_elf(PROGRAM_ELF);
        // 5. Check if proof has been aggregated
        info!("Checking if proof has been aggregated in the last 24 hours...");
        let proof_status = check_proof_proof_aggregation_status(&self.config, &proof, &vk).await;
        let merkle_path = match proof_status {
            aligned_sdk::aggregation_layer::ProofStatus::Verified {
                merkle_root,
                merkle_path,
            } => {
                info!(
                    "Proof aggregated in aggregation with merkle root {:?}",
                    hex::encode(merkle_root)
                );
                merkle_path
            }
            aligned_sdk::aggregation_layer::ProofStatus::Invalid => {
                panic!("Proof did pass merkle root verification");
            }
            aligned_sdk::aggregation_layer::ProofStatus::NotFound => {
                panic!("Proof not found in the last 24 hours logs");
            }
        };
        info!("Proof has been aggregated on aligned, about to send update to chain...");

        // 6. Send updateState transaction to Ethereum
        let receipt =
            send_state_transition_to_chain(&self.config, proof.public_values.to_vec(), merkle_path)
                .await;

        info!(
            "State update in contracts tx hash: {:?}",
            receipt.transaction_hash
        );
    }
}
