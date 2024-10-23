use aligned_sdk::communication::serialization::{cbor_deserialize, cbor_serialize};
use config::NonPayingConfig;
use connection::{send_message, WsMessageSink};
use dotenvy::dotenv;
use eth::service_manager::ServiceManager;
use ethers::contract::ContractError;
use ethers::signers::Signer;
use types::batch_state::BatchState;
use types::user_state::UserState;

use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use aligned_sdk::core::constants::{
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF, AGGREGATOR_GAS_COST, CONSTANT_GAS_COST,
    DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER, DEFAULT_MAX_FEE_PER_PROOF,
    GAS_PRICE_PERCENTAGE_MULTIPLIER, MIN_FEE_PER_PROOF, PERCENTAGE_DIVIDER,
    RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER,
};
use aligned_sdk::core::types::{
    ClientMessage, NoncedVerificationData, ProofInvalidReason, ProvingSystemId, ResponseMessage,
    ValidityResponseMessage, VerificationCommitmentBatch, VerificationData,
    VerificationDataCommitment,
};

use aws_sdk_s3::client::Client as S3Client;
use eth::payment_service::{
    try_create_new_task, BatcherPaymentService, CreateNewTaskFeeParams, SignerMiddlewareT,
};
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use ethers::types::{Address, Signature, TransactionReceipt, U256};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, MutexGuard, RwLock};
use tokio_tungstenite::tungstenite::{Error, Message};
use types::batch_queue::{self, BatchQueueEntry, BatchQueueEntryPriority};
use types::errors::{BatcherError, BatcherSendError};

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};

mod config;
mod connection;
mod eth;
pub mod gnark;
pub mod metrics;
pub mod risc_zero;
pub mod s3;
pub mod sp1;
pub mod types;
mod zk_utils;

pub struct Batcher {
    s3_client: S3Client,
    s3_bucket_name: String,
    download_endpoint: String,
    eth_ws_provider: Provider<Ws>,
    eth_ws_provider_fallback: Provider<Ws>,
    chain_id: U256,
    payment_service: BatcherPaymentService,
    payment_service_fallback: BatcherPaymentService,
    service_manager: ServiceManager,
    service_manager_fallback: ServiceManager,
    batch_state: Mutex<BatchState>,
    max_block_interval: u64,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
    pre_verification_is_enabled: bool,
    non_paying_config: Option<NonPayingConfig>,
    posting_batch: Mutex<bool>,
    disabled_verifiers: Mutex<U256>,
    pub metrics: metrics::BatcherMetrics,
}

impl Batcher {
    pub async fn new(config_file: String) -> Self {
        dotenv().ok();

        // https://docs.aws.amazon.com/sdk-for-rust/latest/dg/localstack.html
        let upload_endpoint = env::var("UPLOAD_ENDPOINT").ok();

        let s3_bucket_name =
            env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME not found in environment");

        let download_endpoint =
            env::var("DOWNLOAD_ENDPOINT").expect("DOWNLOAD_ENDPOINT not found in environment");

        let s3_client = s3::create_client(upload_endpoint).await;

        let config = ConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        let eth_ws_provider =
            Provider::connect_with_reconnects(&config.eth_ws_url, config.batcher.eth_ws_reconnects)
                .await
                .expect("Failed to get ethereum websocket provider");

        log::info!(
            "Starting metrics server on port {}",
            config.batcher.metrics_port
        );
        let metrics = metrics::BatcherMetrics::start(config.batcher.metrics_port)
            .expect("Failed to start metrics server");

        let eth_ws_provider_fallback = Provider::connect_with_reconnects(
            &config.eth_ws_url_fallback,
            config.batcher.eth_ws_reconnects,
        )
        .await
        .expect("Failed to get fallback ethereum websocket provider");

        let eth_rpc_provider =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        let eth_rpc_provider_fallback = eth::get_provider(config.eth_rpc_url_fallback.clone())
            .expect("Failed to get fallback provider");

        let eth_rpc_provider_service_manager =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        let eth_rpc_provider_service_manager_fallback =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        // FIXME(marian): We are getting just the last block number right now, but we should really
        // have the last submitted batch block registered and query it when the batcher is initialized.
        let last_uploaded_batch_block = match eth_rpc_provider.get_block_number().await {
            Ok(block_num) => block_num,
            Err(e) => {
                warn!(
                    "Failed to get block number with main rpc, trying with fallback rpc. Err: {:?}",
                    e
                );
                eth_rpc_provider_fallback
                    .get_block_number()
                    .await
                    .expect("Failed to get block number with fallback rpc")
            }
        };

        let last_uploaded_batch_block = last_uploaded_batch_block.as_u64();

        let chain_id = match eth_rpc_provider.get_chainid().await {
            Ok(chain_id) => chain_id,
            Err(e) => {
                warn!("Failed to get chain id with main rpc: {}", e);
                eth_rpc_provider_fallback
                    .get_chainid()
                    .await
                    .expect("Failed to get chain id with fallback rpc")
            }
        };

        let payment_service = eth::payment_service::get_batcher_payment_service(
            eth_rpc_provider,
            config.ecdsa.clone(),
            deployment_output.addresses.batcher_payment_service.clone(),
        )
        .await
        .expect("Failed to get Batcher Payment Service contract");

        let payment_service_fallback = eth::payment_service::get_batcher_payment_service(
            eth_rpc_provider_fallback,
            config.ecdsa.clone(),
            deployment_output.addresses.batcher_payment_service,
        )
        .await
        .expect("Failed to get fallback Batcher Payment Service contract");

        let service_manager = eth::service_manager::get_service_manager(
            eth_rpc_provider_service_manager,
            config.ecdsa.clone(),
            deployment_output.addresses.service_manager.clone(),
        )
        .await
        .expect("Failed to get Service Manager contract");

        let service_manager_fallback = eth::service_manager::get_service_manager(
            eth_rpc_provider_service_manager_fallback,
            config.ecdsa,
            deployment_output.addresses.service_manager,
        )
        .await
        .expect("Failed to get fallback Service Manager contract");

        let mut user_states = HashMap::new();
        let mut batch_state = BatchState::new();
        let non_paying_config = if let Some(non_paying_config) = config.batcher.non_paying {
            warn!("Non-paying address configuration detected. Will replace non-paying address {} with configured address.",
                non_paying_config.address);

            let non_paying_config = NonPayingConfig::from_yaml_config(non_paying_config).await;
            let nonpaying_nonce = payment_service
                .user_nonces(non_paying_config.replacement.address())
                .call()
                .await
                .expect("Could not get non-paying nonce from Ethereum");

            let non_paying_user_state = UserState::new(nonpaying_nonce);
            user_states.insert(
                non_paying_config.replacement.address(),
                non_paying_user_state,
            );

            batch_state = BatchState::new_with_user_states(user_states);
            Some(non_paying_config)
        } else {
            None
        };

        let disabled_verifiers = match service_manager.disabled_verifiers().call().await {
            Ok(disabled_verifiers) => Ok(disabled_verifiers),
            Err(_) => service_manager_fallback.disabled_verifiers().call().await,
        }
        .expect("Failed to get disabled verifiers");

        Self {
            s3_client,
            s3_bucket_name,
            download_endpoint,
            eth_ws_provider,
            eth_ws_provider_fallback,
            chain_id,
            payment_service,
            payment_service_fallback,
            service_manager,
            service_manager_fallback,
            max_block_interval: config.batcher.block_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
            non_paying_config,
            posting_batch: Mutex::new(false),
            batch_state: Mutex::new(batch_state),
            disabled_verifiers: Mutex::new(disabled_verifiers),
            metrics,
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) -> Result<(), BatcherError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| BatcherError::TcpListenerError(e.to_string()))?;
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            self.metrics.open_connections.inc();
            let batcher = self.clone();
            tokio::spawn(batcher.handle_connection(stream, addr));
        }
        Ok(())
    }

    pub async fn listen_new_blocks(self: Arc<Self>) -> Result<(), BatcherError> {
        let mut stream = self
            .eth_ws_provider
            .subscribe_blocks()
            .await
            .map_err(|e| BatcherError::EthereumSubscriptionError(e.to_string()))?;

        let mut stream_fallback = self
            .eth_ws_provider_fallback
            .subscribe_blocks()
            .await
            .map_err(|e| BatcherError::EthereumSubscriptionError(e.to_string()))?;

        let last_seen_block = Mutex::<u64>::new(0);

        while let Some(block) = tokio::select! {
            block = stream.next() => block,
            block = stream_fallback.next() => block,
        } {
            let batcher = self.clone();
            let block_number = block.number.unwrap_or_default();
            let block_number = u64::try_from(block_number).unwrap_or_default();

            {
                let mut last_seen_block = last_seen_block.lock().await;
                if block_number <= *last_seen_block {
                    continue;
                }
                *last_seen_block = block_number;
            }

            info!("Received new block: {}", block_number);
            tokio::spawn(async move {
                if let Err(e) = batcher.handle_new_block(block_number).await {
                    error!("Error when handling new block: {:?}", e);
                };
            });
        }

        Ok(())
    }

    async fn handle_connection(
        self: Arc<Self>,
        raw_stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), BatcherError> {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;

        debug!("WebSocket connection established: {}", addr);
        let (outgoing, incoming) = ws_stream.split();
        let outgoing = Arc::new(RwLock::new(outgoing));

        let protocol_version_msg = ResponseMessage::ProtocolVersion(
            aligned_sdk::communication::protocol::EXPECTED_PROTOCOL_VERSION,
        );

        let serialized_protocol_version_msg = cbor_serialize(&protocol_version_msg)
            .map_err(|e| BatcherError::SerializationError(e.to_string()))?;

        outgoing
            .write()
            .await
            .send(Message::binary(serialized_protocol_version_msg))
            .await?;

        match incoming
            .try_filter(|msg| future::ready(msg.is_binary()))
            .try_for_each(|msg| self.clone().handle_message(msg, outgoing.clone()))
            .await
        {
            Err(e) => {
                self.metrics.broken_ws_connections.inc();
                error!("Unexpected error: {}", e)
            }
            Ok(_) => info!("{} disconnected", &addr),
        }

        self.metrics.open_connections.dec();
        Ok(())
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: WsMessageSink,
    ) -> Result<(), Error> {
        // Deserialize verification data from message
        let client_msg: ClientMessage = match cbor_deserialize(message.into_data().as_slice()) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to deserialize message: {}", e);
                return Ok(());
            }
        };
        let msg_nonce = client_msg.verification_data.nonce;
        debug!("Received message with nonce: {msg_nonce:?}",);
        self.metrics.received_proofs.inc();

        // * ---------------------------------------------------*
        // *        Perform validations over the message        *
        // * ---------------------------------------------------*

        // This check does not save against "Holesky" and "HoleskyStage", since both are chain_id 17000
        let msg_chain_id = client_msg.verification_data.chain_id;
        if msg_chain_id != self.chain_id {
            warn!("Received message with incorrect chain id: {msg_chain_id}");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidChainId,
            )
            .await;

            return Ok(());
        }

        // This checks saves against "Holesky" and "HoleskyStage", since each one has a different payment service address
        let msg_payment_service_addr = client_msg.verification_data.payment_service_addr;
        if msg_payment_service_addr != self.payment_service.address() {
            warn!("Received message with incorrect payment service address: {msg_payment_service_addr}");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidPaymentServiceAddress(
                    msg_payment_service_addr,
                    self.payment_service.address(),
                ),
            )
            .await;

            return Ok(());
        }

        info!("Verifying message signature...");
        let Ok(addr) = client_msg.verify_signature() else {
            error!("Signature verification error");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidSignature,
            )
            .await;
            return Ok(());
        };
        info!("Message signature verified");

        let proof_size = client_msg.verification_data.verification_data.proof.len();
        if proof_size > self.max_proof_size {
            error!("Proof size exceeds the maximum allowed size.");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::ProofTooLarge).await;
            return Ok(());
        }

        let nonced_verification_data = client_msg.verification_data.clone();

        // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
        if self.pre_verification_is_enabled {
            let verification_data = &nonced_verification_data.verification_data;
            if self
                .is_verifier_disabled(verification_data.proving_system)
                .await
            {
                warn!(
                    "Verifier for proving system {} is disabled, skipping verification",
                    verification_data.proving_system
                );
                send_message(
                    ws_conn_sink.clone(),
                    ValidityResponseMessage::InvalidProof(ProofInvalidReason::DisabledVerifier(
                        verification_data.proving_system,
                    )),
                )
                .await;
                return Ok(());
            }

            if !zk_utils::verify(verification_data).await {
                error!("Invalid proof detected. Verification failed");
                send_message(
                    ws_conn_sink.clone(),
                    ValidityResponseMessage::InvalidProof(ProofInvalidReason::RejectedProof),
                )
                .await;
                return Ok(());
            }
        }

        if self.is_nonpaying(&addr) {
            return self
                .handle_nonpaying_msg(ws_conn_sink.clone(), &client_msg)
                .await;
        }

        info!("Handling paying message");

        // We don't need a batch state lock here, since if the user locks its funds
        // after the check, some blocks should pass until he can withdraw.
        // It is safe to do just do this here.
        if self.user_balance_is_unlocked(&addr).await {
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InsufficientBalance(addr),
            )
            .await;
            return Ok(());
        }

        // We aquire the lock first only to query if the user is already present and the lock is dropped.
        // If it was not present, then the user nonce is queried to the Aligned contract.
        // Lastly, we get a lock of the batch state again and insert the user state if it was still missing.

        let is_user_in_state: bool;
        {
            let batch_state_lock = self.batch_state.lock().await;
            is_user_in_state = batch_state_lock.user_states.contains_key(&addr);
        }

        if !is_user_in_state {
            let ethereum_user_nonce = match self.get_user_nonce_from_ethereum(addr).await {
                Ok(ethereum_user_nonce) => ethereum_user_nonce,
                Err(e) => {
                    error!(
                        "Failed to get user nonce from Ethereum for address {addr:?}. Error: {e:?}"
                    );
                    send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
                    return Ok(());
                }
            };
            let user_state = UserState::new(ethereum_user_nonce);
            let mut batch_state_lock = self.batch_state.lock().await;
            batch_state_lock
                .user_states
                .entry(addr)
                .or_insert(user_state);
        }

        // * ---------------------------------------------------*
        // *        Perform validations over user state         *
        // * ---------------------------------------------------*

        let Some(user_balance) = self.get_user_balance(&addr).await else {
            error!("Could not get balance for address {addr:?}");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::EthRpcError).await;
            return Ok(());
        };

        // For now on until the message is fully processed, the batch state is locked
        // This is needed because we need to query the user state to make validations and
        // finally add the proof to the batch queue.

        let batch_state_lock = self.batch_state.lock().await;
        let Some(proofs_in_batch) = batch_state_lock.get_user_proof_count(&addr).await else {
            error!("Failed to get user proof count: User not found in user states, but it should have been already inserted");
            std::mem::drop(batch_state_lock);
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return Ok(());
        };

        if !self.check_min_balance(proofs_in_batch + 1, user_balance) {
            std::mem::drop(batch_state_lock);
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InsufficientBalance(addr),
            )
            .await;
            return Ok(());
        }

        let cached_user_nonce = batch_state_lock.get_user_nonce(&addr).await;
        let Some(expected_nonce) = cached_user_nonce else {
            error!("Failed to get cached user nonce: User not found in user states, but it should have been already inserted");
            std::mem::drop(batch_state_lock);
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return Ok(());
        };

        if expected_nonce < msg_nonce {
            std::mem::drop(batch_state_lock);
            warn!("Invalid nonce for address {addr}, had nonce {expected_nonce:?} < {msg_nonce:?}");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return Ok(());
        }

        // In this case, the message might be a replacement one. If it is valid,
        // we replace the old entry with the new from the replacement message.
        if expected_nonce > msg_nonce {
            info!("Possible replacement message received: Expected nonce {expected_nonce:?} - message nonce: {msg_nonce:?}");
            self.handle_replacement_message(
                batch_state_lock,
                nonced_verification_data,
                ws_conn_sink.clone(),
                client_msg.signature,
                addr,
            )
            .await;

            return Ok(());
        }

        let msg_max_fee = nonced_verification_data.max_fee;
        let Some(user_min_fee) = batch_state_lock.get_user_min_fee(&addr).await else {
            std::mem::drop(batch_state_lock);
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return Ok(());
        };

        if msg_max_fee > user_min_fee {
            std::mem::drop(batch_state_lock);
            warn!("Invalid max fee for address {addr}, had fee {user_min_fee:?} < {msg_max_fee:?}");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidMaxFee).await;
            return Ok(());
        }

        // * ---------------------------------------------------------------------*
        // *        Add message data into the queue and update user state         *
        // * ---------------------------------------------------------------------*

        if let Err(e) = self
            .add_to_batch(
                batch_state_lock,
                nonced_verification_data,
                ws_conn_sink.clone(),
                client_msg.signature,
                addr,
            )
            .await
        {
            error!("Error while adding entry to batch: {e:?}");
            send_message(ws_conn_sink, ValidityResponseMessage::AddToBatchError).await;
            return Ok(());
        };

        info!("Verification data message handled");
        send_message(ws_conn_sink, ValidityResponseMessage::Valid).await;
        Ok(())
    }

    async fn is_verifier_disabled(&self, verifier: ProvingSystemId) -> bool {
        let disabled_verifiers = self.disabled_verifiers.lock().await;
        zk_utils::is_verifier_disabled(*disabled_verifiers, verifier)
    }

    // Checks user has sufficient balance for paying all its the proofs in the current batch.
    fn check_min_balance(&self, user_proofs_in_batch: usize, user_balance: U256) -> bool {
        let min_balance = U256::from(user_proofs_in_batch) * U256::from(MIN_FEE_PER_PROOF);
        user_balance >= min_balance
    }

    /// Handles a replacement message
    /// First checks if the message is already in the batch
    /// If the message is in the batch, checks if the max fee is higher
    /// If the max fee is higher, replaces the message in the batch
    /// If the max fee is lower, sends an error message to the client
    /// If the message is not in the batch, sends an error message to the client
    /// Returns true if the message was replaced in the batch, false otherwise
    async fn handle_replacement_message(
        &self,
        mut batch_state_lock: MutexGuard<'_, BatchState>,
        nonced_verification_data: NoncedVerificationData,
        ws_conn_sink: WsMessageSink,
        signature: Signature,
        addr: Address,
    ) {
        let replacement_max_fee = nonced_verification_data.max_fee;
        let nonce = nonced_verification_data.nonce;
        let Some(entry) = batch_state_lock.get_entry(addr, nonce) else {
            std::mem::drop(batch_state_lock);
            warn!("Invalid nonce for address {addr}. Queue entry with nonce {nonce} not found");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return;
        };

        let original_max_fee = entry.nonced_verification_data.max_fee;
        if original_max_fee > replacement_max_fee {
            std::mem::drop(batch_state_lock);
            warn!("Invalid replacement message for address {addr}, had fee {original_max_fee:?} < {replacement_max_fee:?}");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidReplacementMessage,
            )
            .await;

            return;
        }

        info!("Replacing message for address {addr} with nonce {nonce} and max fee {replacement_max_fee}");

        // The replacement entry is built from the old entry and validated for then to be replaced
        let mut replacement_entry = entry.clone();
        replacement_entry.signature = signature;
        replacement_entry.verification_data_commitment =
            nonced_verification_data.verification_data.clone().into();
        replacement_entry.nonced_verification_data = nonced_verification_data;

        // Close old sink in old entry and replace it with the new one
        {
            if let Some(messaging_sink) = replacement_entry.messaging_sink {
                let mut old_sink = messaging_sink.write().await;
                if let Err(e) = old_sink.close().await {
                    // we dont want to exit here, just log the error
                    warn!("Error closing sink: {e:?}");
                }
            } else {
                warn!(
                    "Old websocket sink was empty. This should only happen in testing environments"
                )
            };
        }

        replacement_entry.messaging_sink = Some(ws_conn_sink.clone());
        if !batch_state_lock.replacement_entry_is_valid(&replacement_entry) {
            std::mem::drop(batch_state_lock);
            warn!("Invalid replacement message");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidReplacementMessage,
            )
            .await;
            return;
        }

        info!(
            "Replacement entry is valid, incrementing fee for sender: {:?}, nonce: {:?}, max_fee: {:?}",
            replacement_entry.sender, replacement_entry.nonced_verification_data.nonce, replacement_max_fee
        );

        // remove the old entry and insert the new one
        // note that the entries are considered equal for the priority queue
        // if they have the same nonce and sender, so we can remove the old entry
        // by calling remove with the new entry
        batch_state_lock.batch_queue.remove(&replacement_entry);
        batch_state_lock.batch_queue.push(
            replacement_entry.clone(),
            BatchQueueEntryPriority::new(replacement_max_fee, nonce),
        );

        let updated_min_fee_in_batch = batch_state_lock.get_user_min_fee_in_batch(&addr);
        if batch_state_lock
            .update_user_min_fee(&addr, updated_min_fee_in_batch)
            .is_none()
        {
            std::mem::drop(batch_state_lock);
            warn!("User state for address {addr:?} was not present in batcher user states, but it should be");
        };
    }

    async fn disabled_verifiers(&self) -> Result<U256, ContractError<SignerMiddlewareT>> {
        match self.service_manager.disabled_verifiers().call().await {
            Ok(disabled_verifiers) => Ok(disabled_verifiers),
            Err(_) => {
                self.service_manager_fallback
                    .disabled_verifiers()
                    .call()
                    .await
            }
        }
    }

    async fn get_user_nonce_from_ethereum(
        &self,
        addr: Address,
    ) -> Result<U256, ContractError<SignerMiddlewareT>> {
        match self.payment_service.user_nonces(addr).call().await {
            Ok(nonce) => Ok(nonce),
            Err(_) => self.payment_service_fallback.user_nonces(addr).call().await,
        }
    }

    /// Adds verification data to the current batch queue.
    async fn add_to_batch(
        &self,
        mut batch_state_lock: MutexGuard<'_, BatchState>,
        verification_data: NoncedVerificationData,
        ws_conn_sink: WsMessageSink,
        proof_submitter_sig: Signature,
        proof_submitter_addr: Address,
    ) -> Result<(), BatcherError> {
        info!("Calculating verification data commitments...");
        let verification_data_comm = verification_data.clone().into();
        info!("Adding verification data to batch...");

        let max_fee = verification_data.max_fee;
        let nonce = verification_data.nonce;
        batch_state_lock.batch_queue.push(
            BatchQueueEntry::new(
                verification_data,
                verification_data_comm,
                ws_conn_sink,
                proof_submitter_sig,
                proof_submitter_addr,
            ),
            BatchQueueEntryPriority::new(max_fee, nonce),
        );

        info!(
            "Current batch queue length: {}",
            batch_state_lock.batch_queue.len()
        );

        let mut proof_submitter_addr = proof_submitter_addr;

        // If the proof submitter is the nonpaying one, we should update the state
        // of the replacement address.
        proof_submitter_addr = if self.is_nonpaying(&proof_submitter_addr) {
            self.get_nonpaying_replacement_addr()
                .unwrap_or(proof_submitter_addr)
        } else {
            proof_submitter_addr
        };

        let Some(user_proof_count) = batch_state_lock
            .get_user_proof_count(&proof_submitter_addr)
            .await
        else {
            error!("User state of address {proof_submitter_addr} was not found when trying to update user state. This user state should have been present");
            std::mem::drop(batch_state_lock);
            return Err(BatcherError::AddressNotFoundInUserStates(
                proof_submitter_addr,
            ));
        };

        // User state is updated
        if batch_state_lock
            .update_user_state(
                &proof_submitter_addr,
                nonce + U256::one(),
                max_fee,
                user_proof_count + 1,
            )
            .is_none()
        {
            error!("User state of address {proof_submitter_addr} was not found when trying to update user state. This user state should have been present");
            std::mem::drop(batch_state_lock);
            return Err(BatcherError::AddressNotFoundInUserStates(
                proof_submitter_addr,
            ));
        };

        Ok(())
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///   * Has the current batch reached the minimum size to be posted?
    ///   * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    ///
    /// Then the batch will be made as big as possible given this two conditions:
    ///   * The serialized batch size needs to be smaller than the maximum batch size
    ///   * The batch submission fee is less than the lowest `max fee` included the batch,
    ///   * And the batch submission fee is more than the highest `max fee` not included the batch.
    ///
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    /// Once the batch meets the conditions for submission, the finalized batch is then passed to the
    /// `finalize_batch` function.
    async fn is_batch_ready(
        &self,
        block_number: u64,
        gas_price: U256,
    ) -> Option<Vec<BatchQueueEntry>> {
        let mut batch_state_lock = self.batch_state.lock().await;
        let current_batch_len = batch_state_lock.batch_queue.len();
        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        if current_batch_len < 2 {
            info!(
                "Current batch has {} proof. Waiting for more proofs...",
                current_batch_len
            );
            return None;
        }

        if block_number < *last_uploaded_batch_block_lock + self.max_block_interval {
            info!(
                "Current batch not ready to be posted. Minimium amount of {} blocks have not passed. Block passed: {}", self.max_block_interval,
                block_number - *last_uploaded_batch_block_lock,
            );
            return None;
        }

        // Check if a batch is currently being posted
        let mut batch_posting = self.posting_batch.lock().await;
        if *batch_posting {
            info!(
                "Batch is currently being posted. Waiting for the current batch to be finalized..."
            );
            return None;
        }

        // Set the batch posting flag to true
        *batch_posting = true;
        let batch_queue_copy = batch_state_lock.batch_queue.clone();
        let (resulting_batch_queue, finalized_batch) =
            batch_queue::try_build_batch(batch_queue_copy, gas_price, self.max_batch_size)
                .inspect_err(|e| {
                    *batch_posting = false;
                    match e {
                        // We can't post a batch since users are not willing to pay the needed fee, wait for more proofs
                        BatcherError::BatchCostTooHigh => {
                            info!("No working batch found. Waiting for more proofs")
                        }
                        // FIXME: We should refactor this code and instead of returning None, return an error.
                        // See issue https://github.com/yetanotherco/aligned_layer/issues/1046.
                        e => error!("Unexpected error: {:?}", e),
                    }
                })
                .ok()?;

        batch_state_lock.batch_queue = resulting_batch_queue;
        let updated_user_proof_count_and_min_fee =
            batch_state_lock.get_user_proofs_in_batch_and_min_fee();

        let user_addresses: Vec<Address> = batch_state_lock.user_states.keys().cloned().collect();
        for addr in user_addresses.iter() {
            let (proof_count, min_fee) = updated_user_proof_count_and_min_fee
                .get(addr)
                .unwrap_or(&(0, U256::MAX));

            // FIXME: The case where a the update functions return `None` can only happen when the user was not found
            // in the `user_states` map should not really happen here, but doing this check so that we don't unwrap.
            // Once https://github.com/yetanotherco/aligned_layer/issues/1046 is done we could return a more
            // informative error.

            // Now we update the user states related to the batch (proof count in batch and min fee in batch)
            batch_state_lock.update_user_proof_count(addr, *proof_count)?;
            batch_state_lock.update_user_min_fee(addr, *min_fee)?;
        }

        Some(finalized_batch)
    }

    /// Takes the finalized batch as input and builds the merkle tree, posts verification data batch
    /// to s3, creates new task in Aligned contract and sends responses to all clients that added proofs
    /// to the batch. The last uploaded batch block is updated once the task is created in Aligned.
    async fn finalize_batch(
        &self,
        block_number: u64,
        finalized_batch: Vec<BatchQueueEntry>,
        gas_price: U256,
    ) -> Result<(), BatcherError> {
        let nonced_batch_verifcation_data: Vec<NoncedVerificationData> = finalized_batch
            .clone()
            .into_iter()
            .map(|entry| entry.nonced_verification_data)
            .collect();

        let batch_verification_data: Vec<VerificationData> = nonced_batch_verifcation_data
            .iter()
            .map(|vd| vd.verification_data.clone())
            .collect();

        let batch_bytes = cbor_serialize(&batch_verification_data)
            .map_err(|e| BatcherError::TaskCreationError(e.to_string()))?;

        info!("Finalizing batch. Length: {}", finalized_batch.len());
        let batch_data_comm: Vec<VerificationDataCommitment> = finalized_batch
            .clone()
            .into_iter()
            .map(|entry| entry.verification_data_commitment)
            .collect();

        let batch_merkle_tree: MerkleTree<VerificationCommitmentBatch> =
            MerkleTree::build(&batch_data_comm).ok_or_else(|| {
                BatcherError::TaskCreationError(
                    "Failed to Build Merkle Tree: Empty Batch".to_string(),
                )
            })?;

        {
            let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
            // update last uploaded batch block
            *last_uploaded_batch_block = block_number;
            info!(
                "Batch Finalizer: Last uploaded batch block updated to: {}. Lock unlocked",
                block_number
            );
        }

        let leaves: Vec<[u8; 32]> = batch_data_comm
            .iter()
            .map(VerificationCommitmentBatch::hash_data)
            .collect();

        if let Err(e) = self
            .submit_batch(
                &batch_bytes,
                &batch_merkle_tree.root,
                leaves,
                &finalized_batch,
                gas_price,
            )
            .await
        {
            for entry in finalized_batch.into_iter() {
                if let Some(ws_sink) = entry.messaging_sink {
                    let merkle_root = hex::encode(batch_merkle_tree.root);
                    send_message(
                        ws_sink.clone(),
                        ResponseMessage::CreateNewTaskError(merkle_root),
                    )
                    .await
                } else {
                    warn!("Websocket sink was found empty. This should only happen in tests");
                }
            }

            self.flush_queue_and_clear_nonce_cache().await;

            return Err(e);
        };

        connection::send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await
    }

    async fn flush_queue_and_clear_nonce_cache(&self) {
        warn!("Resetting state... Flushing queue and nonces");
        let mut batch_state_lock = self.batch_state.lock().await;
        for (entry, _) in batch_state_lock.batch_queue.iter() {
            if let Some(ws_sink) = entry.messaging_sink.as_ref() {
                send_message(ws_sink.clone(), ResponseMessage::BatchReset).await;
            } else {
                warn!("Websocket sink was found empty. This should only happen in tests");
            }
        }

        let Some(nonpaying_replacement_addr) = self.get_nonpaying_replacement_addr() else {
            batch_state_lock.batch_queue.clear();
            batch_state_lock.user_states.clear();
            return;
        };

        // If there is a nonpaying address configured, then fetch the correct nonce from Ethereum
        // so that it is already loaded

        let Ok(nonpaying_replacement_addr_nonce) = self
            .get_user_nonce_from_ethereum(nonpaying_replacement_addr)
            .await
        else {
            batch_state_lock.batch_queue.clear();
            batch_state_lock.user_states.clear();
            return;
        };
        batch_state_lock.batch_queue.clear();
        batch_state_lock.user_states.clear();
        let nonpaying_user_state = UserState::new(nonpaying_replacement_addr_nonce);
        batch_state_lock
            .user_states
            .insert(nonpaying_replacement_addr, nonpaying_user_state);
    }

    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) -> Result<(), BatcherError> {
        let gas_price_future = self.get_gas_price();
        let disabled_verifiers_future = self.disabled_verifiers();

        let (gas_price, disable_verifiers) =
            tokio::join!(gas_price_future, disabled_verifiers_future);
        let gas_price = gas_price.ok_or(BatcherError::GasPriceError)?;

        {
            let new_disable_verifiers = disable_verifiers
                .map_err(|e| BatcherError::DisabledVerifiersError(e.to_string()))?;
            let mut disabled_verifiers_lock = self.disabled_verifiers.lock().await;
            if new_disable_verifiers != *disabled_verifiers_lock {
                *disabled_verifiers_lock = new_disable_verifiers;
                self.flush_queue_and_clear_nonce_cache().await;
            }
        }

        let modified_gas_price = gas_price * U256::from(GAS_PRICE_PERCENTAGE_MULTIPLIER)
            / U256::from(PERCENTAGE_DIVIDER);

        if let Some(finalized_batch) = self.is_batch_ready(block_number, modified_gas_price).await {
            let batch_finalization_result = self
                .finalize_batch(block_number, finalized_batch, modified_gas_price)
                .await;

            // Resetting this here to avoid doing it on every return path of `finalize_batch` function
            let mut batch_posting = self.posting_batch.lock().await;
            *batch_posting = false;

            batch_finalization_result?;
        }

        Ok(())
    }

    /// Post batch to s3 and submit new task to Ethereum
    async fn submit_batch(
        &self,
        batch_bytes: &[u8],
        batch_merkle_root: &[u8; 32],
        leaves: Vec<[u8; 32]>,
        finalized_batch: &[BatchQueueEntry],
        gas_price: U256,
    ) -> Result<(), BatcherError> {
        let s3_client = self.s3_client.clone();
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: 0x{}", batch_merkle_root_hex);
        let file_name = batch_merkle_root_hex.clone() + ".json";

        info!("Uploading batch to S3...");
        s3::upload_object(
            &s3_client,
            &self.s3_bucket_name,
            batch_bytes.to_vec(),
            &file_name,
        )
        .await
        .map_err(|e| BatcherError::BatchUploadError(e.to_string()))?;

        info!("Batch sent to S3 with name: {}", file_name);

        info!("Uploading batch to contract");
        let batch_data_pointer: String = "".to_owned() + &self.download_endpoint + "/" + &file_name;

        let num_proofs_in_batch = leaves.len();

        let gas_per_proof = (CONSTANT_GAS_COST
            + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs_in_batch as u128)
            / num_proofs_in_batch as u128;

        let fee_per_proof = U256::from(gas_per_proof) * gas_price;
        let fee_for_aggregator = (U256::from(AGGREGATOR_GAS_COST)
            * gas_price
            * U256::from(DEFAULT_AGGREGATOR_FEE_PERCENTAGE_MULTIPLIER))
            / U256::from(PERCENTAGE_DIVIDER);
        let respond_to_task_fee_limit = (fee_for_aggregator
            * U256::from(RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER))
            / U256::from(PERCENTAGE_DIVIDER);
        let fee_params = CreateNewTaskFeeParams::new(
            fee_for_aggregator,
            fee_per_proof,
            gas_price,
            respond_to_task_fee_limit,
        );

        let proof_submitters = finalized_batch.iter().map(|entry| entry.sender).collect();

        self.metrics
            .gas_price_used_on_latest_batch
            .set(gas_price.as_u64() as i64);

        match self
            .create_new_task(
                *batch_merkle_root,
                batch_data_pointer,
                proof_submitters,
                fee_params,
            )
            .await
        {
            Ok(_) => {
                info!("Batch verification task created on Aligned contract");
                self.metrics.sent_batches.inc();
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to send batch to contract, batch will be lost: {:?}",
                    e
                );

                self.metrics.reverted_batches.inc();
                Err(e)
            }
        }
    }

    async fn create_new_task(
        &self,
        batch_merkle_root: [u8; 32],
        batch_data_pointer: String,
        proof_submitters: Vec<Address>,
        fee_params: CreateNewTaskFeeParams,
    ) -> Result<TransactionReceipt, BatcherError> {
        info!("Creating task for: 0x{}", hex::encode(batch_merkle_root));

        match try_create_new_task(
            batch_merkle_root,
            batch_data_pointer.clone(),
            proof_submitters.clone(),
            fee_params.clone(),
            &self.payment_service,
        )
        .await
        {
            Ok(receipt) => Ok(receipt),
            Err(BatcherSendError::TransactionReverted(err)) => {
                // Since transaction was reverted, we don't want to retry with fallback.
                warn!("Transaction reverted {:?}", err);

                Err(BatcherError::TransactionSendError)
            }
            Err(_) => {
                let receipt = try_create_new_task(
                    batch_merkle_root,
                    batch_data_pointer,
                    proof_submitters,
                    fee_params,
                    &self.payment_service_fallback,
                )
                .await?;

                Ok(receipt)
            }
        }
    }

    /// Only relevant for testing and for users to easily use Aligned
    fn is_nonpaying(&self, addr: &Address) -> bool {
        self.non_paying_config
            .as_ref()
            .is_some_and(|non_paying_config| non_paying_config.address == *addr)
    }

    fn get_nonpaying_replacement_addr(&self) -> Option<Address> {
        let non_paying_conf = self.non_paying_config.as_ref()?;
        Some(non_paying_conf.replacement.address())
    }

    /// Only relevant for testing and for users to easily use Aligned in testnet.
    async fn handle_nonpaying_msg(
        &self,
        ws_sink: WsMessageSink,
        client_msg: &ClientMessage,
    ) -> Result<(), Error> {
        info!("Handling nonpaying message");
        let Some(non_paying_config) = self.non_paying_config.as_ref() else {
            warn!("There isn't a non-paying configuration loaded. This message will be ignored");
            send_message(ws_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
            return Ok(());
        };

        let replacement_addr = non_paying_config.replacement.address();
        let Some(replacement_user_balance) = self.get_user_balance(&replacement_addr).await else {
            error!("Could not get balance for non-paying address {replacement_addr:?}");
            send_message(
                ws_sink.clone(),
                ValidityResponseMessage::InsufficientBalance(replacement_addr),
            )
            .await;
            return Ok(());
        };

        if replacement_user_balance == U256::from(0) {
            error!("Insufficient funds for non-paying address {replacement_addr:?}");
            send_message(
                ws_sink.clone(),
                ValidityResponseMessage::InsufficientBalance(replacement_addr),
            )
            .await;
            return Ok(());
        }

        let batch_state_lock = self.batch_state.lock().await;
        let Some(non_paying_nonce) = batch_state_lock.get_user_nonce(&replacement_addr).await
        else {
            std::mem::drop(batch_state_lock);
            error!("Nonce for non-paying address {replacement_addr:?} not found in cache.");
            send_message(ws_sink.clone(), ValidityResponseMessage::EthRpcError).await;
            return Ok(());
        };

        debug!("Non-paying nonce: {:?}", non_paying_nonce);

        let nonced_verification_data = NoncedVerificationData::new(
            client_msg.verification_data.verification_data.clone(),
            non_paying_nonce,
            DEFAULT_MAX_FEE_PER_PROOF.into(), // 13_000 gas per proof * 100 gwei gas price (upper bound)
            self.chain_id,
            self.payment_service.address(),
        );

        let client_msg = ClientMessage::new(
            nonced_verification_data.clone(),
            non_paying_config.replacement.clone(),
        )
        .await;

        let signature = client_msg.signature;
        if let Err(e) = self
            .add_to_batch(
                batch_state_lock,
                nonced_verification_data,
                ws_sink.clone(),
                signature,
                replacement_addr,
            )
            .await
        {
            info!("Error while adding nonpaying address entry to batch: {e:?}");
            send_message(ws_sink, ValidityResponseMessage::AddToBatchError).await;
            return Ok(());
        };

        info!("Non-paying verification data message handled");
        send_message(ws_sink, ValidityResponseMessage::Valid).await;
        Ok(())
    }

    /// Gets the balance of user with address `addr` from Ethereum.
    /// Returns `None` if the balance couldn't be returned
    /// FIXME: This should return a `Result` instead.
    async fn get_user_balance(&self, addr: &Address) -> Option<U256> {
        if let Ok(balance) = self.payment_service.user_balances(*addr).call().await {
            return Some(balance);
        };

        self.payment_service_fallback
            .user_balances(*addr)
            .call()
            .await
            .inspect_err(|_| warn!("Failed to get balance for address {:?}", addr))
            .ok()
    }

    async fn user_balance_is_unlocked(&self, addr: &Address) -> bool {
        if let Ok(unlock_block) = self.payment_service.user_unlock_block(*addr).call().await {
            return unlock_block != U256::zero();
        }
        if let Ok(unlock_block) = self
            .payment_service_fallback
            .user_unlock_block(*addr)
            .call()
            .await
        {
            return unlock_block != U256::zero();
        }
        warn!("Could not get user locking state");
        false
    }

    /// Gets the current gas price from Ethereum.
    /// Returns `None` if the gas price couldn't be returned
    /// FIXME: This should return a `Result` instead.
    async fn get_gas_price(&self) -> Option<U256> {
        if let Ok(gas_price) = self
            .eth_ws_provider
            .get_gas_price()
            .await
            .inspect_err(|e| warn!("Failed to get gas price. Trying with fallback: {e:?}"))
        {
            return Some(gas_price);
        }

        self.eth_ws_provider_fallback
            .get_gas_price()
            .await
            .inspect_err(|e| warn!("Failed to get gas price: {e:?}"))
            .ok()
    }
}
