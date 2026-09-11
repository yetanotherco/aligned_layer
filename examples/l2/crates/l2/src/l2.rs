use crate::config::Config;
use crate::db::{generate_random_transfers, DB};
use crate::eth::send_state_transition_to_chain;
use crate::prover::{self, prove_state_transition, PROGRAM_ELF};
use agg_mode_sdk::blockchain::provider::ProofAggregationServiceProvider;
use agg_mode_sdk::blockchain::{AggregationModeVerificationData, ProofStatus};
use agg_mode_sdk::gateway::provider::AggregationModeGatewayProvider;
use alloy::hex;
use alloy::signers::k256::ecdsa::SigningKey;
use alloy::signers::local::LocalSigner;
use primitive_types::U256;
use sp1_sdk::{HashableKey, SP1ProofWithPublicValues};
use sp1_state_transition_program::ProgramOutput;
use tracing::info;

pub struct L2 {
    aligned_agg_mode_gateway_provider: AggregationModeGatewayProvider<LocalSigner<SigningKey>>,
    aligned_proof_agg_service: ProofAggregationServiceProvider,
    config: Config,
    db: DB,
}

impl L2 {
    pub fn new(config: Config) -> Self {
        let db_path = config.db_path.clone().unwrap_or("./db".to_string());
        let signer = LocalSigner::decrypt_keystore(
            config.private_key_store_path.clone(),
            config.private_key_store_password.clone(),
        )
        .expect("failed to parse private key");

        let gatewat_provider =
            AggregationModeGatewayProvider::new_with_signer(config.network.clone(), signer)
                .expect("to build gateway provider");

        let proof_agg_service = ProofAggregationServiceProvider::new(
            config.network.clone(),
            config.eth_rpc_url.clone(),
            config.beacon_client_url.clone(),
        );

        Self {
            config,
            aligned_agg_mode_gateway_provider: gatewat_provider,
            aligned_proof_agg_service: proof_agg_service,
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
        let (mut proof, vk) = prove_state_transition(&self.db, transfers.clone());
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
        info!("Sending proof to aligned gateway...");
        let res = self
            .aligned_agg_mode_gateway_provider
            .submit_sp1_proof(&proof, &vk)
            .await
            .expect("Failed to send proof to aggregation mode gateway");
        info!("Response from gateway: {:?}", res);

        self.db.save().unwrap();

        proof
    }

    pub async fn update_state_on_chain(&mut self, proof: SP1ProofWithPublicValues) {
        let vk = prover::vk_from_elf(PROGRAM_ELF);
        // 5. Check if proof has been aggregated
        info!("Checking if proof has been aggregated in the last 24 hours...");
        let proof_status = self
            .aligned_proof_agg_service
            .check_proof_verification(
                None,
                AggregationModeVerificationData::SP1 {
                    vk: vk.hash_bytes(),
                    public_inputs: proof.public_values.to_vec(),
                },
            )
            .await
            .expect("To be able to check proof status");

        let merkle_path = match proof_status {
            ProofStatus::Verified {
                merkle_root,
                merkle_path,
            } => {
                info!(
                    "Proof aggregated in aggregation with merkle root {:?}",
                    hex::encode(merkle_root)
                );
                merkle_path
            }
            ProofStatus::Invalid => {
                panic!("Proof did pass merkle root verification");
            }
            ProofStatus::NotFound => {
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
