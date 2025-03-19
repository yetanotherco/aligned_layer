use aligned_sdk::communication::serialization::{cbor_deserialize, cbor_serialize};
use config::NonPayingConfig;
use connection::{send_message, WsMessageSink};
use dotenvy::dotenv;
use eth::service_manager::ServiceManager;
use eth::utils::{calculate_bumped_gas_price, get_batcher_signer, get_gas_price};
use ethers::contract::ContractError;
use ethers::signers::Signer;
use retry::batcher_retryables::{
    cancel_create_new_task_retryable, create_new_task_retryable, get_user_balance_retryable,
    get_user_nonce_from_ethereum_retryable, simulate_create_new_task_retryable,
    user_balance_is_unlocked_retryable,
};
use retry::{retry_function, RetryError};
use tokio::time::{timeout, Instant};
use types::batch_state::BatchState;
use types::user_state::UserState;

use batch_queue::calculate_batch_size;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use aligned_sdk::core::constants::{
    ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF, BATCHER_SUBMISSION_BASE_GAS_COST,
    BUMP_BACKOFF_FACTOR, BUMP_MAX_RETRIES, BUMP_MAX_RETRY_DELAY, BUMP_MIN_RETRY_DELAY,
    CBOR_ARRAY_MAX_OVERHEAD, CONNECTION_TIMEOUT, DEFAULT_MAX_FEE_PER_PROOF,
    ETHEREUM_CALL_BACKOFF_FACTOR, ETHEREUM_CALL_MAX_RETRIES, ETHEREUM_CALL_MAX_RETRY_DELAY,
    ETHEREUM_CALL_MIN_RETRY_DELAY, GAS_PRICE_PERCENTAGE_MULTIPLIER, PERCENTAGE_DIVIDER,
    RESPOND_TO_TASK_FEE_LIMIT_PERCENTAGE_MULTIPLIER,
};
use aligned_sdk::core::types::{
    ClientMessage, GetNonceResponseMessage, NoncedVerificationData, ProofInvalidReason,
    ProvingSystemId, SubmitProofMessage, SubmitProofResponseMessage, VerificationCommitmentBatch,
    VerificationData, VerificationDataCommitment,
};

use aws_sdk_s3::client::Client as S3Client;
use eth::payment_service::{BatcherPaymentService, CreateNewTaskFeeParams, SignerMiddlewareT};
use ethers::prelude::{Middleware, Provider};
use ethers::types::{Address, Signature, TransactionReceipt, U256};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, MutexGuard, RwLock};
use tokio_tungstenite::tungstenite::{Error, Message};
use types::batch_queue::{self, BatchQueueEntry, BatchQueueEntryPriority};
use types::errors::{BatcherError, TransactionSendError};

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};
use crate::telemetry::sender::TelemetrySender;

mod config;
mod connection;
mod eth;
pub mod gnark;
pub mod metrics;
pub mod retry;
pub mod risc_zero;
pub mod s3;
pub mod sp1;
pub mod telemetry;
pub mod types;
mod zk_utils;

pub const LISTEN_NEW_BLOCKS_MAX_TIMES: usize = usize::MAX;

pub struct Batcher {
    s3_client: S3Client,
    s3_bucket_name: String,
    download_endpoint: String,
    eth_ws_url: String,
    eth_ws_url_fallback: String,
    batcher_signer: Arc<SignerMiddlewareT>,
    batcher_signer_fallback: Arc<SignerMiddlewareT>,
    chain_id: U256,
    payment_service: BatcherPaymentService,
    payment_service_fallback: BatcherPaymentService,
    service_manager: ServiceManager,
    service_manager_fallback: ServiceManager,
    batch_state: Mutex<BatchState>,
    min_block_interval: u64,
    transaction_wait_timeout: u64,
    max_proof_size: usize,
    max_batch_byte_size: usize,
    max_batch_proof_qty: usize,
    last_uploaded_batch_block: Mutex<u64>,
    pre_verification_is_enabled: bool,
    non_paying_config: Option<NonPayingConfig>,
    posting_batch: Mutex<bool>,
    disabled_verifiers: Mutex<U256>,
    aggregator_fee_percentage_multiplier: u128,
    aggregator_gas_cost: u128,
    pub metrics: metrics::BatcherMetrics,
    pub telemetry: TelemetrySender,
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
        // Ensure max_batch_bytes_size can at least hold one proof of max_proof_size,
        // including the overhead introduced by serialization
        assert!(
            config.batcher.max_proof_size + CBOR_ARRAY_MAX_OVERHEAD
                <= config.batcher.max_batch_byte_size,
            "max_batch_bytes_size ({}) not big enough for one max_proof_size ({}) proof",
            config.batcher.max_batch_byte_size,
            config.batcher.max_proof_size
        );

        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        log::info!(
            "Starting metrics server on port {}",
            config.batcher.metrics_port
        );
        let metrics = metrics::BatcherMetrics::start(config.batcher.metrics_port)
            .expect("Failed to start metrics server");

        let eth_http_provider =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        let eth_http_provider_fallback = eth::get_provider(config.eth_rpc_url_fallback.clone())
            .expect("Failed to get fallback provider");

        // FIXME(marian): We are getting just the last block number right now, but we should really
        // have the last submitted batch block registered and query it when the batcher is initialized.
        let last_uploaded_batch_block = match eth_http_provider.get_block_number().await {
            Ok(block_num) => block_num,
            Err(e) => {
                warn!(
                    "Failed to get block number with main rpc, trying with fallback rpc. Err: {:?}",
                    e
                );
                eth_http_provider_fallback
                    .get_block_number()
                    .await
                    .expect("Failed to get block number with fallback rpc")
            }
        };

        let last_uploaded_batch_block = last_uploaded_batch_block.as_u64();

        let chain_id = match eth_http_provider.get_chainid().await {
            Ok(chain_id) => chain_id,
            Err(e) => {
                warn!("Failed to get chain id with main rpc: {}", e);
                eth_http_provider_fallback
                    .get_chainid()
                    .await
                    .expect("Failed to get chain id with fallback rpc")
            }
        };

        let batcher_signer = get_batcher_signer(eth_http_provider.clone(), config.ecdsa.clone())
            .await
            .expect("Failed to get Batcher signer");

        let batcher_signer_fallback =
            get_batcher_signer(eth_http_provider_fallback.clone(), config.ecdsa.clone())
                .await
                .expect("Failed to get Batcher signer fallback");

        let payment_service = eth::payment_service::get_batcher_payment_service(
            batcher_signer.clone(),
            deployment_output.addresses.batcher_payment_service.clone(),
        )
        .await
        .expect("Failed to get Batcher Payment Service contract");

        let payment_service_fallback = eth::payment_service::get_batcher_payment_service(
            batcher_signer_fallback.clone(),
            deployment_output.addresses.batcher_payment_service,
        )
        .await
        .expect("Failed to get fallback Batcher Payment Service contract");

        let service_manager = eth::service_manager::get_service_manager(
            eth_http_provider.clone(),
            config.ecdsa.clone(),
            deployment_output.addresses.service_manager.clone(),
        )
        .await
        .expect("Failed to get Service Manager contract");

        let service_manager_fallback = eth::service_manager::get_service_manager(
            eth_http_provider_fallback.clone(),
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

        let telemetry = TelemetrySender::new(format!(
            "http://{}",
            config.batcher.telemetry_ip_port_address
        ));

        Self {
            s3_client,
            s3_bucket_name,
            download_endpoint,
            eth_ws_url: config.eth_ws_url,
            eth_ws_url_fallback: config.eth_ws_url_fallback,
            batcher_signer,
            batcher_signer_fallback,
            chain_id,
            payment_service,
            payment_service_fallback,
            service_manager,
            service_manager_fallback,
            min_block_interval: config.batcher.block_interval,
            transaction_wait_timeout: config.batcher.transaction_wait_timeout,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_byte_size: config.batcher.max_batch_byte_size,
            max_batch_proof_qty: config.batcher.max_batch_proof_qty,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
            non_paying_config,
            aggregator_fee_percentage_multiplier: config
                .batcher
                .aggregator_fee_percentage_multiplier,
            aggregator_gas_cost: config.batcher.aggregator_gas_cost,
            posting_batch: Mutex::new(false),
            batch_state: Mutex::new(batch_state),
            disabled_verifiers: Mutex::new(disabled_verifiers),
            metrics,
            telemetry,
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) -> Result<(), BatcherError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| BatcherError::TcpListenerError(e.to_string()))?;
        info!("Listening on: {}", address);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let batcher = self.clone();
                    // Let's spawn the handling of each connection in a separate task.
                    tokio::spawn(batcher.handle_connection(stream, addr));
                }
                Err(e) => {
                    self.metrics.user_error(&["connection_accept_error", ""]);
                    error!("Couldn't accept new connection: {}", e);
                }
            }
        }
    }

    /// Listen for Ethereum new blocks.
    /// Retries on recoverable errors using exponential backoff
    /// with the maximum number of retries and a `MAX_DELAY` of 1 hour.
    pub async fn listen_new_blocks(self: Arc<Self>) -> Result<(), BatcherError> {
        retry_function(
            || {
                let app = self.clone();
                async move { app.listen_new_blocks_retryable().await }
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            LISTEN_NEW_BLOCKS_MAX_TIMES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        .map_err(|e| e.inner())
    }

    pub async fn listen_new_blocks_retryable(
        self: Arc<Self>,
    ) -> Result<(), RetryError<BatcherError>> {
        let eth_ws_provider = Provider::connect(&self.eth_ws_url).await.map_err(|e| {
            warn!("Failed to instantiate Ethereum websocket provider");
            RetryError::Transient(BatcherError::EthereumSubscriptionError(e.to_string()))
        })?;

        let eth_ws_provider_fallback =
            Provider::connect(&self.eth_ws_url_fallback)
                .await
                .map_err(|e| {
                    warn!("Failed to instantiate fallback Ethereum websocket provider");
                    RetryError::Transient(BatcherError::EthereumSubscriptionError(e.to_string()))
                })?;

        let mut stream = eth_ws_provider.subscribe_blocks().await.map_err(|e| {
            warn!("Error subscribing to blocks.");
            RetryError::Transient(BatcherError::EthereumSubscriptionError(e.to_string()))
        })?;

        let mut stream_fallback =
            eth_ws_provider_fallback
                .subscribe_blocks()
                .await
                .map_err(|e| {
                    warn!("Error subscribing to blocks.");
                    RetryError::Transient(BatcherError::EthereumSubscriptionError(e.to_string()))
                })?;

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
        error!("Failed to fetch blocks");

        Err(RetryError::Transient(
            BatcherError::EthereumSubscriptionError("Could not get new blocks".to_string()),
        ))
    }

    async fn handle_connection(
        self: Arc<Self>,
        raw_stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), BatcherError> {
        info!("Incoming TCP connection from: {}", addr);
        self.metrics.open_connections.inc();

        let ws_stream_future = tokio_tungstenite::accept_async(raw_stream);
        let ws_stream =
            match timeout(Duration::from_secs(CONNECTION_TIMEOUT), ws_stream_future).await {
                Ok(Ok(stream)) => stream,
                Ok(Err(e)) => {
                    warn!("Error while establishing websocket connection: {}", e);
                    self.metrics.open_connections.dec();
                    return Ok(());
                }
                Err(e) => {
                    warn!("Error while establishing websocket connection: {}", e);
                    self.metrics.open_connections.dec();
                    self.metrics.user_error(&["user_timeout", ""]);
                    return Ok(());
                }
            };

        debug!("WebSocket connection established: {}", addr);
        let (outgoing, incoming) = ws_stream.split();
        let outgoing = Arc::new(RwLock::new(outgoing));

        let protocol_version_msg = SubmitProofResponseMessage::ProtocolVersion(
            aligned_sdk::communication::protocol::EXPECTED_PROTOCOL_VERSION,
        );

        let serialized_protocol_version_msg = cbor_serialize(&protocol_version_msg)
            .map_err(|e| BatcherError::SerializationError(e.to_string()))?;

        outgoing
            .write()
            .await
            .send(Message::binary(serialized_protocol_version_msg))
            .await?;

        let mut incoming_filter = incoming.try_filter(|msg| future::ready(msg.is_binary()));
        let future_msg = incoming_filter.try_next();

        // timeout to prevent a DOS attack
        match timeout(Duration::from_secs(CONNECTION_TIMEOUT), future_msg).await {
            Ok(Ok(Some(msg))) => {
                self.clone().handle_message(msg, outgoing.clone()).await?;
            }
            Err(elapsed) => {
                warn!("[{}] {}", &addr, elapsed);
                self.metrics.user_error(&["user_timeout", ""]);
                self.metrics.open_connections.dec();
                return Ok(());
            }
            Ok(Ok(None)) => {
                info!("[{}] Connection closed by the other side", &addr);
                self.metrics.open_connections.dec();
                return Ok(());
            }
            Ok(Err(e)) => {
                error!("Unexpected error: {}", e);
                self.metrics.open_connections.dec();
                return Ok(());
            }
        };

        match incoming_filter
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
                self.metrics.user_error(&["deserialize_error", ""]);
                return Ok(());
            }
        };
        info!("Received new client message of type: {}", client_msg);
        match client_msg {
            ClientMessage::GetNonceForAddress(address) => {
                self.clone()
                    .handle_get_nonce_for_address_msg(address, ws_conn_sink)
                    .await
            }
            ClientMessage::SubmitProof(msg) => {
                self.clone()
                    .handle_submit_proof_msg(msg, ws_conn_sink)
                    .await
            }
        }
    }

    async fn handle_get_nonce_for_address_msg(
        self: Arc<Self>,
        mut address: Address,
        ws_conn_sink: WsMessageSink,
    ) -> Result<(), Error> {
        if self.is_nonpaying(&address) {
            info!("Handling nonpaying message");
            let Some(non_paying_config) = self.non_paying_config.as_ref() else {
                warn!(
                    "There isn't a non-paying configuration loaded. This message will be ignored"
                );
                send_message(
                    ws_conn_sink.clone(),
                    GetNonceResponseMessage::InvalidRequest(
                        "There isn't a non-paying configuration loaded.".to_string(),
                    ),
                )
                .await;
                return Ok(());
            };
            let replacement_addr = non_paying_config.replacement.address();
            address = replacement_addr;
        }

        let cached_user_nonce = {
            let batch_state_lock = self.batch_state.lock().await;
            batch_state_lock.get_user_nonce(&address).await
        };

        let user_nonce = if let Some(user_nonce) = cached_user_nonce {
            user_nonce
        } else {
            match self.get_user_nonce_from_ethereum(address).await {
                Ok(ethereum_user_nonce) => ethereum_user_nonce,
                Err(e) => {
                    error!(
                        "Failed to get user nonce from Ethereum for address {address:?}. Error: {e:?}"
                    );
                    send_message(
                        ws_conn_sink.clone(),
                        GetNonceResponseMessage::EthRpcError("Eth RPC error".to_string()),
                    )
                    .await;
                    return Ok(());
                }
            }
        };

        send_message(
            ws_conn_sink.clone(),
            GetNonceResponseMessage::Nonce(user_nonce),
        )
        .await;

        Ok(())
    }

    async fn handle_submit_proof_msg(
        self: Arc<Self>,
        client_msg: Box<SubmitProofMessage>,
        ws_conn_sink: WsMessageSink,
    ) -> Result<(), Error> {
        let msg_nonce = client_msg.verification_data.nonce;
        debug!("Received message with nonce: {msg_nonce:?}");
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
                SubmitProofResponseMessage::InvalidChainId,
            )
            .await;
            self.metrics.user_error(&["invalid_chain_id", ""]);
            return Ok(());
        }

        // This checks saves against "Holesky" and "HoleskyStage", since each one has a different payment service address
        let msg_payment_service_addr = client_msg.verification_data.payment_service_addr;
        if msg_payment_service_addr != self.payment_service.address() {
            warn!("Received message with incorrect payment service address: {msg_payment_service_addr}");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidPaymentServiceAddress(
                    msg_payment_service_addr,
                    self.payment_service.address(),
                ),
            )
            .await;
            self.metrics
                .user_error(&["invalid_payment_service_address", ""]);
            return Ok(());
        }

        info!("Verifying message signature...");
        let Ok(addr) = client_msg.verify_signature() else {
            error!("Signature verification error");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidSignature,
            )
            .await;
            self.metrics.user_error(&["invalid_signature", ""]);
            return Ok(());
        };
        info!("Message signature verified");

        let proof_size = client_msg.verification_data.verification_data.proof.len();
        if proof_size > self.max_proof_size {
            error!("Proof size exceeds the maximum allowed size.");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::ProofTooLarge,
            )
            .await;
            self.metrics.user_error(&["proof_too_large", ""]);
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
                    SubmitProofResponseMessage::InvalidProof(ProofInvalidReason::DisabledVerifier(
                        verification_data.proving_system,
                    )),
                )
                .await;
                self.metrics.user_error(&[
                    "disabled_verifier",
                    &format!("{}", verification_data.proving_system),
                ]);
                return Ok(());
            }

            if !zk_utils::verify(verification_data).await {
                error!("Invalid proof detected. Verification failed");
                send_message(
                    ws_conn_sink.clone(),
                    SubmitProofResponseMessage::InvalidProof(ProofInvalidReason::RejectedProof),
                )
                .await;
                self.metrics.user_error(&[
                    "rejected_proof",
                    &format!("{}", verification_data.proving_system),
                ]);
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
                SubmitProofResponseMessage::InsufficientBalance(addr),
            )
            .await;
            self.metrics.user_error(&["insufficient_balance", ""]);
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
                    send_message(
                        ws_conn_sink.clone(),
                        SubmitProofResponseMessage::EthRpcError,
                    )
                    .await;
                    self.metrics.user_error(&["eth_rpc_error", ""]);
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
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::EthRpcError,
            )
            .await;
            self.metrics.user_error(&["eth_rpc_error", ""]);
            return Ok(());
        };

        // For now on until the message is fully processed, the batch state is locked
        // This is needed because we need to query the user state to make validations and
        // finally add the proof to the batch queue.

        let batch_state_lock = self.batch_state.lock().await;

        let msg_max_fee = nonced_verification_data.max_fee;
        let Some(user_last_max_fee_limit) =
            batch_state_lock.get_user_last_max_fee_limit(&addr).await
        else {
            std::mem::drop(batch_state_lock);
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::AddToBatchError,
            )
            .await;
            self.metrics.user_error(&["batcher_state_error", ""]);
            return Ok(());
        };

        let Some(user_accumulated_fee) = batch_state_lock.get_user_total_fees_in_queue(&addr).await
        else {
            std::mem::drop(batch_state_lock);
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::AddToBatchError,
            )
            .await;
            self.metrics.user_error(&["batcher_state_error", ""]);
            return Ok(());
        };

        if !self.verify_user_has_enough_balance(user_balance, user_accumulated_fee, msg_max_fee) {
            std::mem::drop(batch_state_lock);
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InsufficientBalance(addr),
            )
            .await;
            self.metrics.user_error(&["insufficient_balance", ""]);
            return Ok(());
        }

        let cached_user_nonce = batch_state_lock.get_user_nonce(&addr).await;
        let Some(expected_nonce) = cached_user_nonce else {
            error!("Failed to get cached user nonce: User not found in user states, but it should have been already inserted");
            std::mem::drop(batch_state_lock);
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::AddToBatchError,
            )
            .await;
            self.metrics.user_error(&["batcher_state_error", ""]);
            return Ok(());
        };

        if expected_nonce < msg_nonce {
            std::mem::drop(batch_state_lock);
            warn!("Invalid nonce for address {addr}, expected nonce: {expected_nonce:?}, received nonce: {msg_nonce:?}");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidNonce,
            )
            .await;
            self.metrics.user_error(&["invalid_nonce", ""]);
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

        if msg_max_fee > user_last_max_fee_limit {
            std::mem::drop(batch_state_lock);
            warn!("Invalid max fee for address {addr}, had fee limit of {user_last_max_fee_limit:?}, sent {msg_max_fee:?}");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidMaxFee,
            )
            .await;
            self.metrics.user_error(&["invalid_max_fee", ""]);
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
            send_message(ws_conn_sink, SubmitProofResponseMessage::AddToBatchError).await;
            self.metrics.user_error(&["add_to_batch_error", ""]);
            return Ok(());
        };

        info!("Verification data message handled");
        Ok(())
    }

    async fn is_verifier_disabled(&self, verifier: ProvingSystemId) -> bool {
        let disabled_verifiers = self.disabled_verifiers.lock().await;
        zk_utils::is_verifier_disabled(*disabled_verifiers, verifier)
    }

    // Verifies user has enough balance for paying all his proofs in the current batch.
    fn verify_user_has_enough_balance(
        &self,
        user_balance: U256,
        user_accumulated_fee: U256,
        new_msg_max_fee: U256,
    ) -> bool {
        let required_balance: U256 = user_accumulated_fee + new_msg_max_fee;
        user_balance >= required_balance
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
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidNonce,
            )
            .await;
            self.metrics.user_error(&["invalid_nonce", ""]);
            return;
        };

        let original_max_fee = entry.nonced_verification_data.max_fee;
        if original_max_fee > replacement_max_fee {
            std::mem::drop(batch_state_lock);
            warn!("Invalid replacement message for address {addr}, had max fee: {original_max_fee:?}, received fee: {replacement_max_fee:?}");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::InvalidReplacementMessage,
            )
            .await;
            self.metrics
                .user_error(&["invalid_replacement_message", ""]);
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
                } else {
                    info!("Old websocket sink closed");
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
                SubmitProofResponseMessage::InvalidReplacementMessage,
            )
            .await;
            self.metrics
                .user_error(&["invalid_replacement_message", ""]);
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

        // update max_fee_limit
        let updated_max_fee_limit_in_batch = batch_state_lock.get_user_min_fee_in_batch(&addr);
        if batch_state_lock
            .update_user_max_fee_limit(&addr, updated_max_fee_limit_in_batch)
            .is_none()
        {
            std::mem::drop(batch_state_lock);
            warn!("User state for address {addr:?} was not present in batcher user states, but it should be");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::AddToBatchError,
            )
            .await;
            return;
        };

        // update total_fees_in_queue
        if batch_state_lock
            .update_user_total_fees_in_queue_of_replacement_message(
                &addr,
                original_max_fee,
                replacement_max_fee,
            )
            .is_none()
        {
            std::mem::drop(batch_state_lock);
            warn!("User state for address {addr:?} was not present in batcher user states, but it should be");
            send_message(
                ws_conn_sink.clone(),
                SubmitProofResponseMessage::AddToBatchError,
            )
            .await;
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

    /// Gets the user nonce from Ethereum.
    /// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
    /// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
    async fn get_user_nonce_from_ethereum(
        &self,
        addr: Address,
    ) -> Result<U256, RetryError<String>> {
        retry_function(
            || {
                get_user_nonce_from_ethereum_retryable(
                    &self.payment_service,
                    &self.payment_service_fallback,
                    addr,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
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

        // Update metrics
        let queue_len = batch_state_lock.batch_queue.len();
        let queue_size_bytes = calculate_batch_size(&batch_state_lock.batch_queue)?;
        self.metrics
            .update_queue_metrics(queue_len as i64, queue_size_bytes as i64);

        info!("Current batch queue length: {}", queue_len);

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

        let Some(current_total_fees_in_queue) = batch_state_lock
            .get_user_total_fees_in_queue(&proof_submitter_addr)
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
                current_total_fees_in_queue + max_fee,
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
    /// This function doesn't remove the proofs from the queue.
    async fn is_batch_ready(
        &self,
        block_number: u64,
        gas_price: U256,
    ) -> Option<Vec<BatchQueueEntry>> {
        let batch_state_lock = self.batch_state.lock().await;
        let current_batch_len = batch_state_lock.batch_queue.len();
        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        if current_batch_len < 1 {
            info!(
                "Current batch has {} proofs. Waiting for more proofs...",
                current_batch_len
            );
            return None;
        }

        if block_number < *last_uploaded_batch_block_lock + self.min_block_interval {
            info!(
                "Current batch not ready to be posted. Minimium amount of {} blocks have not passed. Block passed: {}", self.min_block_interval,
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
        let finalized_batch = batch_queue::try_build_batch(
            batch_queue_copy,
            gas_price,
            self.max_batch_byte_size,
            self.max_batch_proof_qty,
            self.constant_gas_cost(),
        )
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

        Some(finalized_batch)
    }

    /// Takes the submitted proofs and removes them from the queue.
    /// This function should be called only AFTER the submission was confirmed onchain
    async fn remove_proofs_from_queue(
        &self,
        finalized_batch: Vec<BatchQueueEntry>,
    ) -> Result<(), BatcherError> {
        info!("Removing proofs from queue...");
        let mut batch_state_lock = self.batch_state.lock().await;

        finalized_batch.iter().for_each(|entry| {
            if batch_state_lock.batch_queue.remove(entry).is_none() {
                // If this happens, we have a bug in our code
                error!("Some proofs were not found in the queue. This should not happen.");
            }
        });

        // now we calculate the new user_states
        let new_user_states = // proofs, max_fee_limit, total_fees_in_queue
            batch_state_lock.calculate_new_user_states_data();

        let user_addresses: Vec<Address> = batch_state_lock.user_states.keys().cloned().collect();
        let default_value = (0, U256::MAX, U256::zero());
        for addr in user_addresses.iter() {
            let (proof_count, max_fee_limit, total_fees_in_queue) =
                new_user_states.get(addr).unwrap_or(&default_value);

            // FIXME: The case where a the update functions return `None` can only happen when the user was not found
            // in the `user_states` map should not really happen here, but doing this check so that we don't unwrap.
            // Once https://github.com/yetanotherco/aligned_layer/issues/1046 is done we could return a more
            // informative error.

            // Now we update the user states related to the batch (proof count in batch and min fee in batch)
            batch_state_lock
                .update_user_proof_count(addr, *proof_count)
                .ok_or(BatcherError::QueueRemoveError(
                    "Could not update_user_proof_count".into(),
                ))?;
            batch_state_lock
                .update_user_max_fee_limit(addr, *max_fee_limit)
                .ok_or(BatcherError::QueueRemoveError(
                    "Could not update_user_max_fee_limit".into(),
                ))?;
            batch_state_lock
                .update_user_total_fees_in_queue(addr, *total_fees_in_queue)
                .ok_or(BatcherError::QueueRemoveError(
                    "Could not update_user_total_fees_in_queue".into(),
                ))?;
        }

        // Update metrics
        let queue_len = batch_state_lock.batch_queue.len();
        let queue_size_bytes = calculate_batch_size(&batch_state_lock.batch_queue)?;

        self.metrics
            .update_queue_metrics(queue_len as i64, queue_size_bytes as i64);

        Ok(())
    }

    /// Takes the finalized batch as input and:
    ///     builds the merkle tree
    ///     posts verification data batch to s3
    ///     creates new task in Aligned contract
    ///     removes the proofs from the queue, once they are succesfully submitted on-chain
    ///     sends responses to all clients that added proofs to the batch.
    /// The last uploaded batch block is updated once the task is created in Aligned.
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
            .telemetry
            .init_task_trace(&hex::encode(batch_merkle_tree.root))
            .await
        {
            warn!("Failed to initialize task trace on telemetry: {:?}", e);
        }

        // Here we submit the batch on-chain
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
            let reason = format!("{:?}", e);
            if let Err(e) = self
                .telemetry
                .task_creation_failed(&hex::encode(batch_merkle_tree.root), &reason)
                .await
            {
                warn!("Failed to send task status to telemetry: {:?}", e);
            }

            // decide if i want to flush the queue:
            match e {
                BatcherError::TransactionSendError(
                    TransactionSendError::SubmissionInsufficientBalance,
                ) => {
                    // TODO calling remove_proofs_from_queue here is a better solution, flushing only the failed batch
                    // this would also need a message sent to the clients
                    self.flush_queue_and_clear_nonce_cache().await;
                }
                _ => {
                    // Add more cases here if we want in the future
                }
            }

            return Err(e);
        };

        // Once the submit is succesfull, we remove the submitted proofs from the queue
        // TODO handle error case:
        if let Err(e) = self.remove_proofs_from_queue(finalized_batch.clone()).await {
            error!("Unexpected error while updating queue: {:?}", e);
        }

        connection::send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await
    }

    async fn flush_queue_and_clear_nonce_cache(&self) {
        warn!("Resetting state... Flushing queue and nonces");
        let mut batch_state_lock = self.batch_state.lock().await;
        for (entry, _) in batch_state_lock.batch_queue.iter() {
            if let Some(ws_sink) = entry.messaging_sink.as_ref() {
                send_message(ws_sink.clone(), SubmitProofResponseMessage::BatchReset).await;
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

        self.metrics.update_queue_metrics(0, 0);
    }

    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) -> Result<(), BatcherError> {
        let gas_price_future = get_gas_price(
            self.batcher_signer.provider(),
            self.batcher_signer_fallback.provider(),
        );
        let disabled_verifiers_future = self.disabled_verifiers();

        let (gas_price, disable_verifiers) =
            tokio::join!(gas_price_future, disabled_verifiers_future);
        let gas_price = gas_price.map_err(|_| BatcherError::GasPriceError)?;

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
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: 0x{}", batch_merkle_root_hex);
        let file_name = batch_merkle_root_hex.clone() + ".json";
        let batch_data_pointer: String = "".to_owned() + &self.download_endpoint + "/" + &file_name;

        let num_proofs_in_batch = leaves.len();
        let gas_per_proof = (self.constant_gas_cost()
            + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs_in_batch as u128)
            / num_proofs_in_batch as u128;
        let fee_per_proof = U256::from(gas_per_proof) * gas_price;
        let fee_for_aggregator = (U256::from(self.aggregator_gas_cost)
            * gas_price
            * U256::from(self.aggregator_fee_percentage_multiplier))
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

        let proof_submitters: Vec<Address> =
            finalized_batch.iter().map(|entry| entry.sender).collect();

        self.simulate_create_new_task(
            *batch_merkle_root,
            batch_data_pointer.clone(),
            proof_submitters.clone(),
            fee_params.clone(),
        )
        .await?;

        self.metrics
            .gas_price_used_on_latest_batch
            .set(gas_price.as_u64() as i64);

        info!("Uploading batch to S3...");
        self.upload_batch_to_s3(batch_bytes, &file_name).await?;
        if let Err(e) = self
            .telemetry
            .task_uploaded_to_s3(&batch_merkle_root_hex)
            .await
        {
            warn!("Failed to send task status to telemetry: {:?}", e);
        };
        info!("Batch sent to S3 with name: {}", file_name);
        if let Err(e) = self
            .telemetry
            .task_created(
                &hex::encode(batch_merkle_root),
                ethers::utils::format_ether(fee_per_proof),
                num_proofs_in_batch,
            )
            .await
        {
            warn!("Failed to send task status to telemetry: {:?}", e);
        };

        info!("Submitting batch to contract");
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
                error!("Failed to send batch to contract: {:?}", e);

                self.metrics.reverted_batches.inc();
                Err(e)
            }
        }
    }

    /// Sends a `create_new_task` transaction to Ethereum and waits for a maximum of 8 blocks for the receipt.
    /// Retries up to `ETHEREUM_CALL_MAX_RETRIES` times using exponential backoff on recoverable errors while trying to send the transaction:
    /// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
    /// `ReceiptNotFoundError` is treated as non-recoverable, and the transaction will be canceled using `cancel_create_new_task_tx` in that case.
    async fn create_new_task(
        &self,
        batch_merkle_root: [u8; 32],
        batch_data_pointer: String,
        proof_submitters: Vec<Address>,
        fee_params: CreateNewTaskFeeParams,
    ) -> Result<TransactionReceipt, BatcherError> {
        let start = Instant::now();
        let result = retry_function(
            || {
                create_new_task_retryable(
                    batch_merkle_root,
                    batch_data_pointer.clone(),
                    proof_submitters.clone(),
                    fee_params.clone(),
                    self.transaction_wait_timeout,
                    &self.payment_service,
                    &self.payment_service_fallback,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await;
        self.metrics
            .create_new_task_duration
            .set(start.elapsed().as_millis() as i64);
        // Set to zero since it is not always executed
        self.metrics.cancel_create_new_task_duration.set(0);
        match result {
            Ok(receipt) => {
                if let Err(e) = self
                    .telemetry
                    .task_sent(&hex::encode(batch_merkle_root), receipt.transaction_hash)
                    .await
                {
                    warn!("Failed to send task status to telemetry: {:?}", e);
                }
                let gas_cost = Self::gas_cost_in_eth(receipt.effective_gas_price, receipt.gas_used);
                self.metrics
                    .batcher_gas_cost_create_task_total
                    .inc_by(gas_cost);
                Ok(receipt)
            }
            Err(RetryError::Permanent(BatcherError::ReceiptNotFoundError)) => {
                self.metrics.canceled_batches.inc();
                self.cancel_create_new_task_tx(fee_params.gas_price).await;
                Err(BatcherError::ReceiptNotFoundError)
            }
            Err(RetryError::Permanent(e)) | Err(RetryError::Transient(e)) => Err(e),
        }
    }

    /// Simulates the `create_new_task` transaction by sending an `eth_call` to the RPC node.
    /// This function does not mutate the state but verifies if it will revert under the given conditions.
    async fn simulate_create_new_task(
        &self,
        batch_merkle_root: [u8; 32],
        batch_data_pointer: String,
        proof_submitters: Vec<Address>,
        fee_params: CreateNewTaskFeeParams,
    ) -> Result<(), BatcherError> {
        retry_function(
            || {
                simulate_create_new_task_retryable(
                    batch_merkle_root,
                    batch_data_pointer.clone(),
                    proof_submitters.clone(),
                    fee_params.clone(),
                    &self.payment_service,
                    &self.payment_service_fallback,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        .map_err(|e| e.inner())?;

        Ok(())
    }

    /// Sends a transaction to Ethereum with the same nonce as the previous one to override it.
    /// Retries on recoverable errors with exponential backoff.
    /// Bumps the fee if not included in 6 blocks, using `calculate_bumped_gas_price`.
    /// In the first 5 attemps, bumps the fee every 3 blocks. Then exponential backoff takes over.
    /// After 2 hours (attempt 13), retries occur hourly for 1 day (33 retries).
    pub async fn cancel_create_new_task_tx(&self, old_tx_gas_price: U256) {
        info!("Cancelling createNewTask transaction...");
        let start = Instant::now();
        let iteration = Arc::new(Mutex::new(0));
        let previous_gas_price = Arc::new(Mutex::new(old_tx_gas_price));

        match retry_function(
            || async {
                let mut iteration = iteration.lock().await;
                let mut previous_gas_price = previous_gas_price.lock().await;

                let current_gas_price = match get_gas_price(
                    self.batcher_signer.provider(),
                    self.batcher_signer_fallback.provider(),
                )
                .await
                {
                    Ok(gas_price) => gas_price,
                    Err(e) => return Err(RetryError::Transient(e)),
                };

                let bumped_gas_price =
                    calculate_bumped_gas_price(*previous_gas_price, current_gas_price, *iteration);

                *iteration += 1;
                *previous_gas_price = bumped_gas_price;

                cancel_create_new_task_retryable(
                    &self.batcher_signer,
                    &self.batcher_signer_fallback,
                    bumped_gas_price,
                    self.transaction_wait_timeout,
                )
                .await
            },
            BUMP_MIN_RETRY_DELAY,
            BUMP_BACKOFF_FACTOR,
            BUMP_MAX_RETRIES,
            BUMP_MAX_RETRY_DELAY,
        )
        .await
        {
            Ok(receipt) => {
                info!("createNewTask transaction successfully canceled");
                let gas_cost = Self::gas_cost_in_eth(receipt.effective_gas_price, receipt.gas_used);
                self.metrics
                    .batcher_gas_cost_cancel_task_total
                    .inc_by(gas_cost);
            }
            Err(e) => error!("Could not cancel createNewTask transaction: {e}"),
        };
        self.metrics
            .cancel_create_new_task_duration
            .set(start.elapsed().as_millis() as i64);
    }

    fn gas_cost_in_eth(gas_price: Option<U256>, gas_used: Option<U256>) -> f64 {
        if let (Some(gas_price), Some(gas_used)) = (gas_price, gas_used) {
            let wei_gas_cost = gas_price
                .checked_mul(gas_used)
                .unwrap_or_else(U256::max_value);

            // f64 is typically sufficient for transaction gas costs.
            let max_f64_u256 = U256::from(f64::MAX as u64);
            if wei_gas_cost > max_f64_u256 {
                return f64::MAX;
            }

            let wei_gas_cost_f64 = wei_gas_cost.low_u128() as f64;
            let eth_gas_cost = wei_gas_cost_f64 / 1e18;

            return eth_gas_cost;
        }
        0.0
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
        client_msg: &SubmitProofMessage,
    ) -> Result<(), Error> {
        info!("Handling nonpaying message");
        let Some(non_paying_config) = self.non_paying_config.as_ref() else {
            warn!("There isn't a non-paying configuration loaded. This message will be ignored");
            send_message(ws_sink.clone(), SubmitProofResponseMessage::InvalidNonce).await;
            return Ok(());
        };

        let replacement_addr = non_paying_config.replacement.address();
        let Some(replacement_user_balance) = self.get_user_balance(&replacement_addr).await else {
            error!("Could not get balance for non-paying address {replacement_addr:?}");
            send_message(
                ws_sink.clone(),
                SubmitProofResponseMessage::InsufficientBalance(replacement_addr),
            )
            .await;
            return Ok(());
        };

        if replacement_user_balance == U256::from(0) {
            error!("Insufficient funds for non-paying address {replacement_addr:?}");
            send_message(
                ws_sink.clone(),
                SubmitProofResponseMessage::InsufficientBalance(replacement_addr),
            )
            .await;
            return Ok(());
        }

        let batch_state_lock = self.batch_state.lock().await;

        let nonced_verification_data = NoncedVerificationData::new(
            client_msg.verification_data.verification_data.clone(),
            client_msg.verification_data.nonce,
            DEFAULT_MAX_FEE_PER_PROOF.into(), // 2_000 gas per proof * 100 gwei gas price (upper bound)
            self.chain_id,
            self.payment_service.address(),
        );

        let client_msg = SubmitProofMessage::new(
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
            send_message(ws_sink, SubmitProofResponseMessage::AddToBatchError).await;
            return Ok(());
        };

        info!("Non-paying verification data message handled");
        Ok(())
    }

    /// Gets the balance of user with address `addr` from Ethereum.
    /// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
    /// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs)
    /// Returns `None` if the balance couldn't be returned
    /// FIXME: This should return a `Result` instead.
    async fn get_user_balance(&self, addr: &Address) -> Option<U256> {
        retry_function(
            || {
                get_user_balance_retryable(
                    &self.payment_service,
                    &self.payment_service_fallback,
                    addr,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        .ok()
    }

    /// Checks if the user's balance is unlocked for a given address.
    /// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
    /// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
    /// Returns `false` if an error occurs during the retries.
    async fn user_balance_is_unlocked(&self, addr: &Address) -> bool {
        let Ok(unlocked) = retry_function(
            || {
                user_balance_is_unlocked_retryable(
                    &self.payment_service,
                    &self.payment_service_fallback,
                    addr,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        else {
            warn!("Could not get user locking state.");
            return false;
        };
        unlocked
    }

    /// Uploads the batch to s3.
    /// Retries on recoverable errors using exponential backoff up to `ETHEREUM_CALL_MAX_RETRIES` times:
    /// (0,5 secs - 1 secs - 2 secs - 4 secs - 8 secs).
    async fn upload_batch_to_s3(
        &self,
        batch_bytes: &[u8],
        file_name: &str,
    ) -> Result<(), BatcherError> {
        let start = Instant::now();
        let result = retry_function(
            || {
                Self::upload_batch_to_s3_retryable(
                    batch_bytes,
                    file_name,
                    self.s3_client.clone(),
                    &self.s3_bucket_name,
                )
            },
            ETHEREUM_CALL_MIN_RETRY_DELAY,
            ETHEREUM_CALL_BACKOFF_FACTOR,
            ETHEREUM_CALL_MAX_RETRIES,
            ETHEREUM_CALL_MAX_RETRY_DELAY,
        )
        .await
        .map_err(|e| BatcherError::BatchUploadError(e.to_string()));

        self.metrics
            .s3_duration
            .set(start.elapsed().as_micros() as i64);

        result
    }

    async fn upload_batch_to_s3_retryable(
        batch_bytes: &[u8],
        file_name: &str,
        s3_client: S3Client,
        s3_bucket_name: &str,
    ) -> Result<(), RetryError<String>> {
        s3::upload_object(&s3_client, s3_bucket_name, batch_bytes.to_vec(), file_name)
            .await
            .map_err(|e| {
                warn!("Error uploading batch to s3 {e}");
                RetryError::Transient(e.to_string())
            })?;
        Ok(())
    }

    fn constant_gas_cost(&self) -> u128 {
        (self.aggregator_fee_percentage_multiplier * self.aggregator_gas_cost) / PERCENTAGE_DIVIDER
            + BATCHER_SUBMISSION_BASE_GAS_COST
    }
}
