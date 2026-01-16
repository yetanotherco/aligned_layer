pub mod config;
mod db;
mod eth;
pub mod fetcher;
mod merkle_tree;
mod retry;
mod types;

use crate::{
    aggregators::{AlignedProof, ProofAggregationError, ZKVMEngine},
    backend::{
        db::{Db, DbError},
        retry::{
            retry_function, RetryError, ETHEREUM_CALL_BACKOFF_FACTOR, ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY, ETHEREUM_CALL_MIN_RETRY_DELAY,
        },
        types::{AlignedProofAggregationService, AlignedProofAggregationServiceContract},
    },
};

use alloy::{
    consensus::{BlobTransactionSidecar, EnvKzgSettings, EthereumTxEnvelope, TxEip4844WithSidecar},
    eips::{
        eip4844::BYTES_PER_BLOB, eip7594::BlobTransactionSidecarEip7594, BlockNumberOrTag,
        Encodable2718,
    },
    hex,
    network::{EthereumWallet, TransactionBuilder},
    primitives::{utils::parse_ether, Address, TxHash, U256},
    providers::{PendingTransactionError, Provider, ProviderBuilder},
    rpc::types::{TransactionReceipt, TransactionRequest},
    signers::local::LocalSigner,
};
use config::Config;
use fetcher::{ProofsFetcher, ProofsFetcherError};
use merkle_tree::compute_proofs_merkle_root;
use risc0_ethereum_contracts::encode_seal;
use sqlx::types::Uuid;
use std::{str::FromStr, time::Duration};
use tokio::time::sleep;
use tracing::info;
use tracing::{error, warn};

#[derive(Debug)]
pub enum AggregatedProofSubmissionError {
    BuildingBlobCommitment,
    BuildingBlobProof,
    BuildingBlobVersionedHash,
    Risc0EncodingSeal(String),
    SendVerifyAggregatedProofTransaction(String),
    ReceiptError(PendingTransactionError),
    FetchingProofs(ProofsFetcherError),
    ZKVMAggregation(ProofAggregationError),
    BuildingMerkleRoot,
    MerkleRootMisMatch,
    StoringMerklePaths(DbError),
    GasPriceError(String),
    LatestBlockNotFound,
    BaseFeePerGasMissing,
}

enum SubmitOutcome {
    // NOTE: Boxed because enums are sized to their largest variant; without boxing,
    // every `SubmitOutcome` would reserve space for a full `TransactionReceipt`,
    // even in the `Pending` case (see clippy::large_enum_variant).
    Confirmed(Box<TransactionReceipt>),
    Pending(TxHash),
}

pub struct ProofAggregator {
    engine: ZKVMEngine,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    fetcher: ProofsFetcher,
    config: Config,
    sp1_chunk_aggregator_vk_hash_bytes: [u8; 32],
    risc0_chunk_aggregator_image_id_bytes: [u8; 32],
    db: Db,
    signer_address: Address,
}

impl ProofAggregator {
    pub async fn new(config: Config) -> Self {
        let rpc_url: reqwest::Url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let signer = LocalSigner::decrypt_keystore(
            config.ecdsa.private_key_store_path.clone(),
            config.ecdsa.private_key_store_password.clone(),
        )
        .expect("Keystore signer should be `cast wallet` compliant");
        let wallet = EthereumWallet::from(signer.clone());

        let signer_address = signer.address();

        // Check if the monthly budget is non-negative to avoid runtime errors later
        let _monthly_budget_in_wei = parse_ether(&config.monthly_budget_eth.to_string())
            .expect("Monthly budget must be a non-negative value");

        info!("Monthly budget set to {} eth", config.monthly_budget_eth);

        let signed_rpc_provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

        let proof_aggregation_service = AlignedProofAggregationService::new(
            Address::from_str(&config.proof_aggregation_service_address)
                .expect("AlignedProofAggregationService address should be valid"),
            signed_rpc_provider.clone(),
        );

        let engine =
            ZKVMEngine::from_env().expect("AGGREGATOR env variable to be set to one of sp1|risc0");

        let db = Db::try_new(config.db_connection_urls.as_slice())
            .await
            .expect("To connect to db");

        let fetcher = ProofsFetcher::new(db.clone());

        let sp1_chunk_aggregator_vk_hash_bytes: [u8; 32] =
            hex::decode(&config.sp1_chunk_aggregator_vk_hash)
                .expect("Failed to decode SP1 chunk aggregator VK hash")
                .try_into()
                .expect("SP1 chunk aggregator VK hash must be 32 bytes");

        let risc0_chunk_aggregator_image_id_bytes: [u8; 32] =
            hex::decode(&config.risc0_chunk_aggregator_image_id)
                .expect("Failed to decode Risc0 chunk aggregator image id")
                .try_into()
                .expect("Risc0 chunk aggregator image id must be 32 bytes");

        Self {
            engine,
            proof_aggregation_service,
            fetcher,
            config,
            sp1_chunk_aggregator_vk_hash_bytes,
            risc0_chunk_aggregator_image_id_bytes,
            db,
            signer_address,
        }
    }

    pub async fn start(&mut self) {
        info!("Starting proof aggregator service");

        info!("About to aggregate and submit proof to be verified on chain");

        let (proofs, tasks_id) = match self
            .fetcher
            .fetch_pending_proofs(self.engine.clone(), self.config.total_proofs_limit as i64)
            .await
            .map_err(AggregatedProofSubmissionError::FetchingProofs)
        {
            Ok(res) => res,
            Err(e) => {
                error!("Error while aggregating and submitting proofs: {:?}", e);
                return;
            }
        };

        let res = self
            .aggregate_and_submit_proofs_on_chain((proofs, &tasks_id))
            .await;

        match res {
            Ok(()) => {
                info!("Process finished successfully");
            }
            Err(err) => {
                error!("Error while aggregating and submitting proofs: {:?}", err);
                warn!("Marking tasks back to pending after failure");
                if let Err(e) = self.db.mark_tasks_as_pending(&tasks_id).await {
                    error!("Error while marking proofs to pending again: {:?}", e);
                };
            }
        }
    }

    async fn aggregate_and_submit_proofs_on_chain(
        &mut self,
        (proofs, tasks_id): (Vec<AlignedProof>, &[Uuid]),
    ) -> Result<(), AggregatedProofSubmissionError> {
        if proofs.is_empty() {
            warn!("No proofs collected, skipping aggregation...");
            return Ok(());
        }

        info!("Proofs fetched, constructing merkle root...");
        let (merkle_tree, leaves) = compute_proofs_merkle_root(&proofs)
            .ok_or(AggregatedProofSubmissionError::BuildingMerkleRoot)?;
        let merkle_root = merkle_tree.root;
        info!("Merkle root constructed: 0x{}", hex::encode(merkle_root));

        info!("Starting proof aggregation program...");
        let (aggregated_proof, zkvm_merkle_root) = self
            .engine
            .aggregate_proofs(proofs, self.config.proofs_per_chunk)
            .map_err(AggregatedProofSubmissionError::ZKVMAggregation)?;
        info!("Proof aggregation program finished");

        info!("Starting Merkle root verification: comparing ZKVM output with off-VM computation");
        if zkvm_merkle_root != merkle_root {
            error!(
                "Merkle root mismatch detected: ZKVM = {zkvm_merkle_root:?}, off-VM = {merkle_root:?}"
            );
            return Err(AggregatedProofSubmissionError::MerkleRootMisMatch);
        }
        info!("Merkle root verification successful: roots match");

        info!("Constructing blob...");
        let (blob, blob_versioned_hash) = self.construct_blob(leaves).await?;
        info!(
            "Blob constructed, versioned hash: {}",
            hex::encode(blob_versioned_hash)
        );

        let receipt = self
            .wait_and_send_proof_on_chain_retryable(blob, blob_versioned_hash, aggregated_proof)
            .await?;
        info!(
            "Proof sent and verified, tx hash {:?}",
            receipt.transaction_hash
        );

        info!("Storing merkle paths for each task...",);
        let mut merkle_paths_for_tasks: Vec<(Uuid, Vec<u8>)> = vec![];
        for (idx, task_id) in tasks_id.iter().enumerate() {
            let Some(proof) = merkle_tree.get_proof_by_pos(idx) else {
                warn!("Proof not found for task id {task_id}");
                continue;
            };
            let proof_bytes = proof
                .merkle_path
                .iter()
                .flat_map(|e| e.to_vec())
                .collect::<Vec<_>>();

            merkle_paths_for_tasks.push((*task_id, proof_bytes))
        }
        self.db
            .insert_tasks_merkle_path_and_mark_them_as_verified(merkle_paths_for_tasks)
            .await
            .map_err(AggregatedProofSubmissionError::StoringMerklePaths)?;
        info!("Merkle path inserted sucessfully",);

        Ok(())
    }

    async fn wait_and_send_proof_on_chain_retryable(
        &self,
        blob: BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: AlignedProof,
    ) -> Result<TransactionReceipt, AggregatedProofSubmissionError> {
        retry_function(
            || {
                self.wait_and_send_proof_to_verify_on_chain(
                    blob.clone(),
                    blob_versioned_hash,
                    &aggregated_proof,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        .map_err(|e| {
            error!("Couldn't get nonce: {:?}", e);
            e.inner()
        })
    }

    /// ### Blob capacity
    ///
    /// As dictated in [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844), each blob can hold:
    ///
    /// - `FIELD_ELEMENTS_PER_BLOB = 4096`
    /// - `BYTES_PER_FIELD_ELEMENT = 32`
    ///
    /// This gives a total theoretical capacity of:
    ///
    /// `FIELD_ELEMENTS_PER_BLOB * BYTES_PER_FIELD_ELEMENT = 4096 * 32 = 131072 bytes`
    ///
    /// However, this full capacity isn't usable due to the encoding of KZG commitments to elliptic curve points.
    /// Specifically:
    ///
    /// - Ethereum uses the BLS12-381 curve, whose scalar field modulus is slightly less than `2^256`
    ///   (closer to `2^255`).
    /// - Therefore, 32-byte field elements can't represent all 256-bit values.
    /// - To ensure values are within the field modulus, we **pad with a leading `0x00` byte**,
    ///   effectively capping values below the modulus.
    /// - This reduces the usable payload to **31 bytes per field element**.
    ///
    /// So, the _actual usable capacity_ per blob is:
    ///
    /// `4096 * 31 = 126976 bytes`
    ///
    /// Meaning that we can send as much as 126976 / 32 = 3968 proofs per blob
    async fn construct_blob(
        &self,
        leaves: Vec<[u8; 32]>,
    ) -> Result<(BlobTransactionSidecar, [u8; 32]), AggregatedProofSubmissionError> {
        let data: Vec<u8> = leaves.iter().flat_map(|arr| arr.iter().copied()).collect();
        let mut blob_data: [u8; BYTES_PER_BLOB] = [0u8; BYTES_PER_BLOB];

        // We pad the data with 0x0 byte every 31 bytes so that the field elements
        // constructed from the bytes are less than BLS_MODULUS.
        //
        // See https://github.com/ethereum/consensus-specs/blob/86fb82b221474cc89387fa6436806507b3849d88/specs/deneb/polynomial-commitments.md#bytes_to_bls_field
        let mut offset = 0;
        for chunk in data.chunks(31) {
            blob_data[offset] = 0x00;
            let start = offset + 1;
            let end = start + chunk.len();
            blob_data[start..end].copy_from_slice(chunk);
            offset += 32;
        }

        // calculate kzg commitments for blob

        // This parameter is the optimal balance between performance and memory usage to load the trusted setup
        // Source: https://github.com/ethereum/c-kzg-4844?tab=readme-ov-file#precompute
        let settings = c_kzg::ethereum_kzg_settings(8);
        let blob = c_kzg::Blob::new(blob_data);
        let commitment = settings
            .blob_to_kzg_commitment(&blob)
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobCommitment)?;
        let proof = settings
            .compute_blob_kzg_proof(&blob, &commitment.to_bytes())
            .map_err(|_| AggregatedProofSubmissionError::BuildingBlobProof)?;

        let blob = BlobTransactionSidecar::from_kzg(
            vec![blob],
            vec![commitment.to_bytes()],
            vec![proof.to_bytes()],
        );
        let blob_versioned_hash = blob
            .versioned_hash_for_blob(0)
            .ok_or(AggregatedProofSubmissionError::BuildingBlobVersionedHash)?
            .0;

        Ok((blob, blob_versioned_hash))
    }

    pub async fn wait_and_send_proof_to_verify_on_chain(
        &self,
        blob: BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: &AlignedProof,
    ) -> Result<TransactionReceipt, RetryError<AggregatedProofSubmissionError>> {
        self.wait_until_can_submit_aggregated_proof().await?;

        info!("Sending proof to ProofAggregationService contract...");

        let max_retries = self.config.max_bump_retries;

        let mut last_error: Option<AggregatedProofSubmissionError> = None;

        let mut pending_hashes: Vec<TxHash> = Vec::with_capacity(max_retries as usize);

        // Get the nonce once at the beginning and reuse it for all retries
        let nonce = self
            .proof_aggregation_service
            .provider()
            .get_transaction_count(self.signer_address)
            .await
            .map_err(|e| {
                RetryError::Transient(
                    AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                        "Failed to get nonce: {e}"
                    )),
                )
            })?;

        info!("Using nonce {}", nonce);

        for attempt in 0..max_retries {
            info!("Transaction attempt {} of {}", attempt + 1, max_retries);

            // Wrap the entire transaction submission in a result to catch all errors, passing
            // the same nonce to all attempts
            let attempt_result = self
                .try_submit_transaction(
                    &blob,
                    blob_versioned_hash,
                    aggregated_proof,
                    nonce,
                    attempt,
                )
                .await;

            match attempt_result {
                Ok(SubmitOutcome::Confirmed(receipt)) => {
                    info!(
                        "Transaction confirmed successfully on attempt {}",
                        attempt + 1
                    );
                    return Ok(*receipt);
                }
                Ok(SubmitOutcome::Pending(tx_hash)) => {
                    warn!(
                        "Attempt {} timed out waiting for receipt; storing pending tx",
                        attempt + 1
                    );
                    pending_hashes.push(tx_hash);
                    last_error = Some(
                        AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(
                            "Timed out waiting for receipt".to_string(),
                        ),
                    );
                }
                Err(err) => {
                    warn!("Attempt {} failed: {:?}", attempt + 1, err);
                    last_error = Some(err);
                }
            }

            // Check if any pending tx was confirmed before retrying
            if let Some(receipt) = self.check_pending_txs_confirmed(&pending_hashes).await {
                return Ok(receipt);
            }

            info!("Retrying with bumped gas fees and same nonce {}...", nonce);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        warn!("Max retries ({}) exceeded", max_retries);
        Err(RetryError::Transient(last_error.unwrap_or_else(|| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(
                "Max retries exceeded with no error details".to_string(),
            )
        })))
    }

    async fn try_submit_transaction(
        &self,
        blob: &BlobTransactionSidecar,
        blob_versioned_hash: [u8; 32],
        aggregated_proof: &AlignedProof,
        nonce: u64,
        attempt: u16,
    ) -> Result<SubmitOutcome, AggregatedProofSubmissionError> {
        let retry_interval = Duration::from_secs(self.config.bump_retry_interval_seconds);

        // Build the transaction request
        let mut tx_req = match aggregated_proof {
            AlignedProof::SP1(proof) => self
                .proof_aggregation_service
                .verifyAggregationSP1(
                    blob_versioned_hash.into(),
                    proof.proof_with_pub_values.public_values.to_vec().into(),
                    proof.proof_with_pub_values.bytes().into(),
                    self.sp1_chunk_aggregator_vk_hash_bytes.into(),
                )
                .sidecar(blob.clone())
                .into_transaction_request(),
            AlignedProof::Risc0(proof) => {
                let encoded_seal = encode_seal(&proof.receipt).map_err(|e| {
                    AggregatedProofSubmissionError::Risc0EncodingSeal(e.to_string())
                })?;
                self.proof_aggregation_service
                    .verifyAggregationRisc0(
                        blob_versioned_hash.into(),
                        encoded_seal.into(),
                        proof.receipt.journal.bytes.clone().into(),
                        self.risc0_chunk_aggregator_image_id_bytes.into(),
                    )
                    .sidecar(blob.clone())
                    .into_transaction_request()
            }
        };

        // Set the nonce explicitly
        tx_req = tx_req.with_nonce(nonce);

        // Apply gas fee bump for retries
        tx_req = self.apply_gas_fee_bump(tx_req, attempt).await?;

        let provider = self.proof_aggregation_service.provider();

        // Fill the transaction
        let envelope = provider
            .fill(tx_req)
            .await
            .map_err(|err| {
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Failed to fill transaction: {err}"
                ))
            })?
            .try_into_envelope()
            .map_err(|err| {
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Failed to convert to envelope: {err}"
                ))
            })?;

        // Convert to EIP-4844 transaction
        let tx: EthereumTxEnvelope<TxEip4844WithSidecar<BlobTransactionSidecarEip7594>> = envelope
            .try_into_pooled()
            .map_err(|err| {
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Failed to pool transaction: {err}"
                ))
            })?
            .try_map_eip4844(|tx| {
                tx.try_map_sidecar(|sidecar| sidecar.try_into_7594(EnvKzgSettings::Default.get()))
            })
            .map_err(|err| {
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Failed to convert to EIP-7594: {err}"
                ))
            })?;

        // Send the transaction
        let encoded_tx = tx.encoded_2718();
        let pending_tx = provider
            .send_raw_transaction(&encoded_tx)
            .await
            .map_err(|err| {
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Failed to send raw transaction: {err}"
                ))
            })?;

        let tx_hash = *pending_tx.tx_hash();

        let receipt_result = tokio::time::timeout(retry_interval, pending_tx.get_receipt()).await;

        match receipt_result {
            Ok(Ok(receipt)) => Ok(SubmitOutcome::Confirmed(Box::new(receipt))),
            Ok(Err(err)) => Err(
                AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(format!(
                    "Error getting receipt: {err}"
                )),
            ),
            Err(_) => Ok(SubmitOutcome::Pending(tx_hash)),
        }
    }

    // Checks if any of the pending transactions have been confirmed.
    // Returns the receipt if one is found, otherwise None.
    async fn check_pending_txs_confirmed(
        &self,
        pending_hashes: &[TxHash],
    ) -> Option<TransactionReceipt> {
        for tx_hash in pending_hashes {
            if let Ok(Some(receipt)) = self
                .proof_aggregation_service
                .provider()
                .get_transaction_receipt(*tx_hash)
                .await
            {
                info!("Pending tx {} confirmed before retry", tx_hash);
                return Some(receipt);
            }
        }
        None
    }

    // Updates the gas fees of a `TransactionRequest` using EIP-1559 fee parameters.
    // Intended for retrying an on-chain submission after a timeout.
    //
    // Strategy:
    // - Fetch the current base fee from the latest block.
    // - Fetch the suggested priority fee from the network (eth_maxPriorityFeePerGas).
    // - Compute priority fee as: suggested * (1 + (attempt + 1) * 0.1), capped at `max_priority_fee_upper_limit`.
    // - Compute `max_fee_per_gas` as: (1 + max_fee_bump_percentage/100) * base_fee + priority_fee.
    //
    // Fees are recomputed on each retry using the latest base fee.

    async fn apply_gas_fee_bump(
        &self,
        tx_req: TransactionRequest,
        attempt: u16,
    ) -> Result<TransactionRequest, AggregatedProofSubmissionError> {
        let provider = self.proof_aggregation_service.provider();

        let max_fee_bump_percentage = self.config.max_fee_bump_percentage;
        let max_priority_fee_upper_limit = self.config.max_priority_fee_upper_limit;

        let latest_block = provider
            .get_block_by_number(BlockNumberOrTag::Latest)
            .await
            .map_err(|e| AggregatedProofSubmissionError::GasPriceError(e.to_string()))?
            .ok_or(AggregatedProofSubmissionError::LatestBlockNotFound)?;

        let current_base_fee = latest_block
            .header
            .base_fee_per_gas
            .ok_or(AggregatedProofSubmissionError::BaseFeePerGasMissing)?
            as f64;

        // Fetch suggested priority fee from the network
        let suggested_priority_fee = provider
            .get_max_priority_fee_per_gas()
            .await
            .map_err(|e| AggregatedProofSubmissionError::GasPriceError(e.to_string()))?;

        // Calculate priority fee: suggested * (attempt + 1), capped at max
        let priority_fee_multiplier = (attempt + 1) as u128;
        let max_priority_fee_per_gas =
            (suggested_priority_fee * priority_fee_multiplier).min(max_priority_fee_upper_limit);

        // Calculate max fee with cumulative bump per attempt to ensure replacement tx is accepted
        let max_fee_multiplier = 1.0 + max_fee_bump_percentage as f64 / 100.0;
        let max_fee_per_gas =
            (max_fee_multiplier * current_base_fee) as u128 + max_priority_fee_per_gas;

        info!(
            "Base fee: {:.4} Gwei. Applying max_fee_per_gas: {:.4} Gwei and max_priority_fee_per_gas: {:.4} Gwei to tx",
            current_base_fee / 1e9,
            max_fee_per_gas as f64 / 1e9,
            max_priority_fee_per_gas as f64 / 1e9
        );

        Ok(tx_req
            .with_max_fee_per_gas(max_fee_per_gas)
            .with_max_priority_fee_per_gas(max_priority_fee_per_gas))
    }

    async fn wait_until_can_submit_aggregated_proof(
        &self,
    ) -> Result<(), RetryError<AggregatedProofSubmissionError>> {
        info!("Started waiting until we can submit the aggregated proof.");

        // We start on 24 hours because the proof aggregator runs once a day, so the time elapsed
        // should be considered over a 24h period.
        let mut time_elapsed = Duration::from_secs(24 * 3600);

        // Sleep for 3 minutes (15 blocks) before re-evaluating on each iteration
        let time_to_sleep = Duration::from_secs(180);

        // Iterate until we can send the proof on-chain
        loop {
            // Fetch gas price from network
            let gas_price = self
                .proof_aggregation_service
                .provider()
                .get_gas_price()
                .await
                .map_err(|e| {
                    RetryError::Transient(AggregatedProofSubmissionError::GasPriceError(
                        e.to_string(),
                    ))
                })?;

            info!("Fetched gas price from network: {gas_price}");

            if eth::should_send_proof_to_verify_on_chain(
                time_elapsed,
                self.config.monthly_budget_eth,
                U256::from(gas_price),
            ) {
                break;
            } else {
                info!("Skipping sending proof to ProofAggregationService contract due to budget/time constraints.");
            }

            time_elapsed += time_to_sleep;
            sleep(time_to_sleep).await;
        }

        Ok(())
    }
}
