extern crate core;

use aligned_sdk::communication::serialization::{cbor_deserialize, cbor_serialize};
use aligned_sdk::eth::batcher_payment_service::SignatureData;
use config::NonPayingConfig;
use dotenv::dotenv;
use ethers::contract::ContractError;
use ethers::signers::Signer;
use priority_queue::PriorityQueue;
use serde::Serialize;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::iter::repeat;
use std::net::SocketAddr;
use std::sync::Arc;

use aligned_sdk::core::types::{
    BatchInclusionData, ClientMessage, NoncedVerificationData, ResponseMessage,
    ValidityResponseMessage, VerificationCommitmentBatch, VerificationData,
    VerificationDataCommitment,
};
use aws_sdk_s3::client::Client as S3Client;
use eth::{try_create_new_task, BatcherPaymentService, CreateNewTaskFeeParams, SignerMiddlewareT};
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use ethers::types::{Address, Signature, TransactionReceipt, U256};
use futures_util::stream::SplitSink;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::WebSocketStream;
use types::batch_queue::{BatchQueue, BatchQueueEntry, BatchQueueEntryPriority};
use types::errors::{BatcherError, BatcherSendError};

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};

mod config;
mod eth;
pub mod gnark;
pub mod halo2;
pub mod mina;
pub mod risc_zero;
pub mod s3;
pub mod sp1;
pub mod types;
mod zk_utils;

const AGGREGATOR_GAS_COST: u128 = 400_000;
const BATCHER_SUBMISSION_BASE_GAS_COST: u128 = 100_000;
const ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF: u128 = 13_000;
const CONSTANT_GAS_COST: u128 = AGGREGATOR_GAS_COST + BATCHER_SUBMISSION_BASE_GAS_COST;
const DEFAULT_MAX_FEE_PER_PROOF: u128 = ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000_000; // gas_price = 100 Gwei = 0.0000001 ether (high gas price)
const MIN_FEE_PER_PROOF: u128 = ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * 100_000_000; // gas_price = 0.1 Gwei = 0.0000000001 ether (low gas price)

struct BatchState {
    batch_queue: BatchQueue,
    user_nonces: HashMap<Address, U256>,
    /// The minimum fee of a pending proof for a user.
    /// This should always be the fee of the biggest pending nonce by the user.
    /// This is used to check if a user is submitting a proof with a higher nonce and higher fee,
    /// which is invalid and should be rejected.
    user_min_fee: HashMap<Address, U256>,
    user_proof_count_in_batch: HashMap<Address, u64>,
}

impl BatchState {
    fn new() -> Self {
        Self {
            batch_queue: BatchQueue::new(),
            user_nonces: HashMap::new(),
            user_min_fee: HashMap::new(),
            user_proof_count_in_batch: HashMap::new(),
        }
    }

    fn get_user_proof_count(&self, addr: &Address) -> u64 {
        *self.user_proof_count_in_batch.get(addr).unwrap_or(&0)
    }

    /*
       Increments the user proof count in the batch, if the user is already in the hashmap.
       If the user is not in the hashmap, it adds the user to the hashmap with a count of 1 to represent the first proof.
    */
    fn increment_user_proof_count(&mut self, addr: &Address) {
        self.user_proof_count_in_batch
            .entry(*addr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    fn get_entry(&self, sender: Address, nonce: U256) -> Option<&BatchQueueEntry> {
        self.batch_queue
            .iter()
            .map(|(entry, _)| entry)
            .find(|entry| {
                entry.sender == sender
                    && U256::from_big_endian(entry.nonced_verification_data.nonce.as_slice())
                        == nonce
            })
    }

    /// Checks if the entry is valid
    /// An entry is valid if there is no entry with the same sender,
    /// lower nonce and a lower fee
    /// If the entry is valid, it replaces the entry in the queue
    /// to increment the max fee, then it updates the user min fee if necessary
    /// If the entry is invalid, it returns a validity response message.
    /// If the entry is valid, it returns None.
    fn validate_and_increment_max_fee(
        &mut self,
        replacement_entry: BatchQueueEntry,
    ) -> Option<ValidityResponseMessage> {
        let replacement_max_fee = replacement_entry.nonced_verification_data.max_fee;
        let nonce =
            U256::from_big_endian(replacement_entry.nonced_verification_data.nonce.as_slice());
        let sender = replacement_entry.sender;

        debug!(
            "Checking validity of entry with sender: {:?}, nonce: {:?}, max_fee: {:?}",
            sender, nonce, replacement_max_fee
        );

        // it is a valid entry only if there is no entry with the same sender, lower nonce and a lower fee
        let is_valid = !self.batch_queue.iter().any(|(entry, _)| {
            entry.sender == sender
                && U256::from_big_endian(entry.nonced_verification_data.nonce.as_slice()) < nonce
                && entry.nonced_verification_data.max_fee < replacement_max_fee
        });

        if !is_valid {
            return Some(ValidityResponseMessage::InvalidReplacementMessage);
        }

        info!(
            "Entry is valid, incrementing fee for sender: {:?}, nonce: {:?}, max_fee: {:?}",
            sender, nonce, replacement_max_fee
        );

        // remove the old entry and insert the new one
        // note that the entries are considered equal for the priority queue
        // if they have the same nonce and sender, so we can remove the old entry
        // by calling remove with the new entry
        self.batch_queue.remove(&replacement_entry);
        self.batch_queue.push(
            replacement_entry.clone(),
            BatchQueueEntryPriority::new(replacement_max_fee, nonce),
        );

        let user_min_fee = self
            .batch_queue
            .iter()
            .filter(|(e, _)| e.sender == sender)
            .map(|(e, _)| e.nonced_verification_data.max_fee)
            .min()
            .unwrap_or(U256::max_value());

        self.user_min_fee.insert(sender, user_min_fee);

        None
    }

    /// Updates:
    ///     * The user proof count in batch
    ///     * The user min fee pending in batch (which is the one with the highest nonce)
    /// based on whats currenlty in the batch queue.
    /// This is necessary because the whole batch may not be included in the finalized batch,
    /// This caches are needed to validate user messages.
    fn update_user_proofs_in_batch_and_min_fee(&mut self) {
        let mut updated_user_min_fee = HashMap::new();
        let mut updated_user_proof_count_in_batch = HashMap::new();

        for (entry, _) in self.batch_queue.iter() {
            *updated_user_proof_count_in_batch
                .entry(entry.sender)
                .or_insert(0) += 1;

            let min_fee = updated_user_min_fee
                .entry(entry.sender)
                .or_insert(entry.nonced_verification_data.max_fee);

            if entry.nonced_verification_data.max_fee < *min_fee {
                *min_fee = entry.nonced_verification_data.max_fee;
            }
        }

        self.user_proof_count_in_batch = updated_user_proof_count_in_batch;
        self.user_min_fee = updated_user_min_fee;
    }
}

pub struct Batcher {
    s3_client: S3Client,
    s3_bucket_name: String,
    download_endpoint: String,
    eth_ws_provider: Provider<Ws>,
    eth_ws_provider_fallback: Provider<Ws>,
    chain_id: U256,
    payment_service: BatcherPaymentService,
    payment_service_fallback: BatcherPaymentService,
    batch_state: Mutex<BatchState>,
    max_block_interval: u64,
    min_batch_len: usize,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
    pre_verification_is_enabled: bool,
    non_paying_config: Option<NonPayingConfig>,
    posting_batch: Mutex<bool>,
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

        let payment_service = eth::get_batcher_payment_service(
            eth_rpc_provider,
            config.ecdsa.clone(),
            deployment_output.addresses.batcher_payment_service.clone(),
        )
        .await
        .expect("Failed to get Batcher Payment Service contract");

        let payment_service_fallback = eth::get_batcher_payment_service(
            eth_rpc_provider_fallback,
            config.ecdsa,
            deployment_output.addresses.batcher_payment_service,
        )
        .await
        .expect("Failed to get fallback Batcher Payment Service contract");

        let non_paying_config = if let Some(non_paying_config) = config.batcher.non_paying {
            warn!("Non-paying address configuration detected. Will replace non-paying address {} with configured address.",
                non_paying_config.address);
            Some(NonPayingConfig::from_yaml_config(non_paying_config).await)
        } else {
            None
        };

        Self {
            s3_client,
            s3_bucket_name,
            download_endpoint,
            eth_ws_provider,
            eth_ws_provider_fallback,
            chain_id,
            payment_service,
            payment_service_fallback,
            batch_state: Mutex::new(BatchState::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_len: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
            non_paying_config,
            posting_batch: Mutex::new(false),
        }
    }

    pub async fn listen_connections(self: Arc<Self>, address: &str) -> Result<(), BatcherError> {
        // Create the event loop and TCP listener we'll accept connections on.
        let listener = TcpListener::bind(address).await.expect("Failed to build");
        info!("Listening on: {}", address);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
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
            Err(e) => error!("Unexpected error: {}", e),
            Ok(_) => info!("{} disconnected", &addr),
        }

        Ok(())
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Result<(), Error> {
        // Deserialize verification data from message
        let client_msg: ClientMessage = match cbor_deserialize(message.into_data().as_slice()) {
            Ok(msg) => msg,
            Err(e) => {
                warn!("Failed to deserialize message: {}", e);
                return Ok(());
            }
        };

        info!(
            "Received message with nonce: {}",
            U256::from_big_endian(client_msg.verification_data.nonce.as_slice())
        );

        if client_msg.verification_data.chain_id != self.chain_id {
            warn!(
                "Received message with incorrect chain id: {}",
                client_msg.verification_data.chain_id
            );

            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidChainId,
            )
            .await;

            return Ok(());
        }

        info!("Verifying message signature...");
        if let Ok(addr) = client_msg.verify_signature() {
            info!("Message signature verified");
            if self.is_nonpaying(&addr) {
                self.handle_nonpaying_msg(ws_conn_sink.clone(), client_msg)
                    .await
            } else {
                if !self
                    .check_user_balance_and_increment_proof_count(&addr)
                    .await
                {
                    send_message(
                        ws_conn_sink.clone(),
                        ValidityResponseMessage::InsufficientBalance(addr),
                    )
                    .await;

                    return Ok(());
                }

                let nonced_verification_data = client_msg.verification_data;
                if nonced_verification_data.verification_data.proof.len() > self.max_proof_size {
                    error!("Proof size exceeds the maximum allowed size.");
                    send_message(ws_conn_sink.clone(), ValidityResponseMessage::ProofTooLarge)
                        .await;
                    return Ok(());
                }

                // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
                if self.pre_verification_is_enabled
                    && !zk_utils::verify(&nonced_verification_data.verification_data).await
                {
                    error!("Invalid proof detected. Verification failed.");
                    send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidProof).await;
                    return Ok(()); // Send error message to the client and return
                }

                // Nonce and max fee verification
                let nonce = U256::from_big_endian(nonced_verification_data.nonce.as_slice());
                let max_fee = nonced_verification_data.max_fee;

                if max_fee < U256::from(MIN_FEE_PER_PROOF) {
                    error!("The max fee signed in the message is less than the accepted minimum fee to be included in the batch.");
                    send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidMaxFee)
                        .await;
                    return Ok(());
                }

                let mut batch_state = self.batch_state.lock().await;

                let expected_user_nonce = match batch_state.user_nonces.get(&addr) {
                    Some(nonce) => *nonce,
                    None => {
                        let user_nonce = match self.get_user_nonce(addr).await {
                            Ok(nonce) => nonce,
                            Err(e) => {
                                error!("Failed to get user nonce for address {:?}: {:?}", addr, e);
                                send_message(
                                    ws_conn_sink.clone(),
                                    ValidityResponseMessage::InvalidNonce,
                                )
                                .await;

                                return Ok(());
                            }
                        };

                        batch_state.user_nonces.insert(addr, user_nonce);
                        user_nonce
                    }
                };

                let min_fee = match batch_state.user_min_fee.get(&addr) {
                    Some(fee) => *fee,
                    None => U256::max_value(),
                };

                match expected_user_nonce.cmp(&nonce) {
                    std::cmp::Ordering::Less => {
                        // invalid, expected user nonce < nonce
                        warn!(
                            "Invalid nonce for address {addr}, had nonce {:?} < {:?}",
                            expected_user_nonce, nonce
                        );
                        send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce)
                            .await;
                        return Ok(());
                    }
                    std::cmp::Ordering::Equal => {
                        // if we are here nonce == expected_user_nonce
                        if !self
                            .handle_expected_nonce_message(
                                batch_state,
                                min_fee,
                                nonced_verification_data,
                                ws_conn_sink.clone(),
                                client_msg.signature,
                                addr,
                            )
                            .await
                        {
                            // message should not be added to batch
                            return Ok(());
                        };
                    }
                    std::cmp::Ordering::Greater => {
                        // might be replacement message
                        // if the message is already in the batch
                        // we can check if we need to increment the fee
                        // get the entry with the same sender and nonce
                        if !self
                            .handle_replacement_message(
                                batch_state,
                                nonced_verification_data,
                                ws_conn_sink.clone(),
                                client_msg.signature,
                                addr,
                                expected_user_nonce,
                            )
                            .await
                        {
                            // message should not be added to batch
                            return Ok(());
                        }
                    }
                }

                info!("Verification data message handled");

                send_message(ws_conn_sink, ValidityResponseMessage::Valid).await;
                Ok(())
            }
        } else {
            error!("Signature verification error");
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InvalidSignature,
            )
            .await;
            Ok(()) // Send error message to the client and return
        }
    }

    // Checks user has sufficient balance
    // If user has sufficient balance, increments the user's proof count in the batch
    async fn check_user_balance_and_increment_proof_count(&self, addr: &Address) -> bool {
        if self.user_balance_is_unlocked(addr).await {
            return false;
        }
        let mut batch_state = self.batch_state.lock().await;

        let user_proofs_in_batch = batch_state.get_user_proof_count(addr) + 1;

        let user_balance = self.get_user_balance(addr).await;

        let min_balance = U256::from(user_proofs_in_batch) * U256::from(MIN_FEE_PER_PROOF);
        if user_balance < min_balance {
            return false;
        }

        batch_state.increment_user_proof_count(addr);
        true
    }

    /// Handles a message with an expected nonce.
    /// If the max_fee is valid, it is added to the batch.
    /// If the max_fee is invalid, a message is sent to the client.
    /// Returns true if the message was added to the batch, false otherwise.
    async fn handle_expected_nonce_message(
        &self,
        mut batch_state: tokio::sync::MutexGuard<'_, BatchState>,
        min_fee: U256,
        nonced_verification_data: NoncedVerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        signature: Signature,
        addr: Address,
    ) -> bool {
        let max_fee = nonced_verification_data.max_fee;
        if max_fee > min_fee {
            warn!(
                "Invalid max fee for address {addr}, had fee {:?} < {:?}",
                min_fee, max_fee
            );
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidMaxFee).await;
            return false;
        }

        let nonce = U256::from_big_endian(&nonced_verification_data.nonce);

        batch_state.user_nonces.insert(addr, nonce + U256::one());
        batch_state.user_min_fee.insert(addr, max_fee);

        self.add_to_batch(
            batch_state,
            nonced_verification_data,
            ws_conn_sink.clone(),
            signature,
            addr,
        )
        .await;

        true
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
        mut batch_state: tokio::sync::MutexGuard<'_, BatchState>,
        nonced_verification_data: NoncedVerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        signature: Signature,
        addr: Address,
        expected_user_nonce: U256,
    ) -> bool {
        let replacement_max_fee = nonced_verification_data.max_fee;
        let nonce = U256::from_big_endian(&nonced_verification_data.nonce);

        let mut replacement_entry = match batch_state.get_entry(addr, nonce) {
            Some(entry) => {
                if entry.nonced_verification_data.max_fee < replacement_max_fee {
                    entry.clone()
                } else {
                    warn!(
                        "Invalid replacement message for address {addr}, had fee {:?} < {:?}",
                        entry.nonced_verification_data.max_fee, replacement_max_fee
                    );
                    send_message(
                        ws_conn_sink.clone(),
                        ValidityResponseMessage::InvalidReplacementMessage,
                    )
                    .await;

                    return false;
                }
            }
            None => {
                warn!(
                    "Invalid nonce for address {addr} Expected: {:?}, got: {:?}",
                    expected_user_nonce, nonce
                );
                send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidNonce).await;
                return false;
            }
        };

        info!(
            "Replacing message for address {} with nonce {} and max fee {}",
            addr, nonce, replacement_max_fee
        );

        replacement_entry.signature = signature;
        replacement_entry.verification_data_commitment =
            nonced_verification_data.verification_data.clone().into();
        replacement_entry.nonced_verification_data = nonced_verification_data;

        // close old sink and replace with new one
        {
            let mut old_sink = replacement_entry.messaging_sink.write().await;
            if let Err(e) = old_sink.close().await {
                // we dont want to exit here, just log the error
                warn!("Error closing sink: {:?}", e);
            }
        }
        replacement_entry.messaging_sink = ws_conn_sink.clone();

        if let Some(msg) = batch_state.validate_and_increment_max_fee(replacement_entry) {
            warn!("Invalid max fee");
            send_message(ws_conn_sink.clone(), msg).await;
            return false;
        }

        true
    }

    async fn get_user_nonce(
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
        mut batch_state: tokio::sync::MutexGuard<'_, BatchState>,
        verification_data: NoncedVerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        proof_submitter_sig: Signature,
        proof_submiter_addr: Address,
    ) {
        info!("Calculating verification data commitments...");
        let verification_data_comm = verification_data.clone().into();
        info!("Adding verification data to batch...");

        let max_fee = verification_data.max_fee;
        let nonce = U256::from_big_endian(verification_data.nonce.as_slice());

        batch_state.batch_queue.push(
            BatchQueueEntry::new(
                verification_data,
                verification_data_comm,
                ws_conn_sink,
                proof_submitter_sig,
                proof_submiter_addr,
            ),
            BatchQueueEntryPriority::new(max_fee, nonce),
        );
        info!(
            "Current batch queue length: {}",
            batch_state.batch_queue.len()
        );
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// Then the batch will be made as big as possible given this two conditions:
    ///     * The serialized batch size needs to be smaller than the maximum batch size
    ///     * The batch submission fee is less than the lowest `max fee` included the batch,
    ///     * And the batch submission fee is more than the highest `max fee` not included the batch.
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    /// Once the batch meets the conditions for submission, the finalized batch is then passed to the
    /// `finalize_batch` function.
    async fn is_batch_ready(
        &self,
        block_number: u64,
        gas_price: U256,
    ) -> Option<Vec<BatchQueueEntry>> {
        let mut batch_state = self.batch_state.lock().await;
        let current_batch_len = batch_state.batch_queue.len();

        let last_uploaded_batch_block_lock = self.last_uploaded_batch_block.lock().await;

        // FIXME(marian): This condition should be changed to current_batch_size == 0
        // once the bug in Lambdaworks merkle tree is fixed.
        if current_batch_len < 2 {
            info!("Current batch is empty or length 1. Waiting for more proofs...");
            return None;
        }

        if current_batch_len < self.min_batch_len
            && block_number < *last_uploaded_batch_block_lock + self.max_block_interval
        {
            info!(
                "Current batch not ready to be posted. Current block: {} - Last uploaded block: {}. Current batch length: {} - Minimum batch length: {}",
                block_number, *last_uploaded_batch_block_lock, current_batch_len, self.min_batch_len
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

        let mut batch_queue_copy = batch_state.batch_queue.clone();

        match self.try_build_batch(&mut batch_queue_copy, gas_price) {
            Some(finalized_batch) => {
                // Set the batch queue to batch queue copy
                batch_state.batch_queue = batch_queue_copy;
                batch_state.update_user_proofs_in_batch_and_min_fee();

                Some(finalized_batch)
            }
            None => {
                // We cant post a batch since users are not willing to pay the needed fee, wait for more proofs
                info!("No working batch found. Waiting for more proofs...");
                *batch_posting = false;
                None
            }
        }
    }

    /// Tries to build a batch from the current batch queue.
    /// The function iterates over the batch queue and tries to build a batch that satisfies the gas price
    /// and the max_fee set by the users.
    /// If a working batch is found, the function tries to make it as big as possible by adding more proofs,
    /// until a user is not willing to pay the required fee.
    /// The extra check is that the batch size does not surpass the maximum batch size.
    /// Note that the batch queue is sorted descending by the max_fee set by the users.
    /// We use a copy of the batch queue because we might not find a working batch,
    /// and we want to keep the original batch queue intact.
    /// Returns Some(working_batch) if found, None otherwise.
    fn try_build_batch(
        &self,
        batch_queue_copy: &mut PriorityQueue<BatchQueueEntry, BatchQueueEntryPriority>,
        gas_price: U256,
    ) -> Option<Vec<BatchQueueEntry>> {
        let mut finalized_batch = vec![];
        let mut finalized_batch_size = 2; // at most two extra bytes for cbor encoding array markers
        let mut finalized_batch_works = false;

        while let Some((entry, _)) = batch_queue_copy.peek() {
            let serialized_vd_size =
                match cbor_serialize(&entry.nonced_verification_data.verification_data) {
                    Ok(val) => val.len(),
                    Err(e) => {
                        warn!("Serialization error: {:?}", e);
                        break;
                    }
                };

            if finalized_batch_size + serialized_vd_size > self.max_batch_size {
                break;
            }

            let num_proofs = finalized_batch.len() + 1;

            let gas_per_proof = (CONSTANT_GAS_COST
                + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs as u128)
                / num_proofs as u128;

            let fee_per_proof = U256::from(gas_per_proof) * gas_price;

            debug!(
                "Validating that batch submission fee {} is less than max fee {} for sender {}",
                fee_per_proof, entry.nonced_verification_data.max_fee, entry.sender,
            );

            // it is sufficient to check this max fee because it will be the lowest since its sorted
            if fee_per_proof < entry.nonced_verification_data.max_fee && num_proofs >= 2 {
                finalized_batch_works = true;
            } else if finalized_batch_works {
                // Can not add latest element since it is not willing to pay the corresponding fee
                // Could potentially still find another working solution later with more elements,
                // maybe we can explore all lengths in a future version
                // or do the reverse from this, try with whole batch,
                // then with whole batch minus last element, etc
                break;
            }

            // Either max fee is insufficient but we have not found a working solution yet,
            // or we can keep adding to a working batch,
            // Either way we need to keep iterating
            finalized_batch_size += serialized_vd_size;

            // We can unwrap here because we have already peeked to check there is a value
            let (entry, _) = batch_queue_copy.pop().unwrap();
            finalized_batch.push(entry);
        }

        if finalized_batch_works {
            Some(finalized_batch)
        } else {
            None
        }
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
            MerkleTree::build(&batch_data_comm);

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
            for entry in finalized_batch.iter() {
                let merkle_root = hex::encode(batch_merkle_tree.root);
                send_message(
                    entry.messaging_sink.clone(),
                    ResponseMessage::CreateNewTaskError(merkle_root),
                )
                .await
            }

            self.flush_queue_and_clear_nonce_cache().await;

            return Err(e);
        };

        send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await
    }

    async fn flush_queue_and_clear_nonce_cache(&self) {
        warn!("Resetting state... Flushing queue and nonces");
        let mut batch_state = self.batch_state.lock().await;

        for (entry, _) in batch_state.batch_queue.iter() {
            send_message(entry.messaging_sink.clone(), ResponseMessage::BatchReset).await;
        }

        batch_state.batch_queue.clear();
        batch_state.user_nonces.clear();
        batch_state.user_proof_count_in_batch.clear();
        batch_state.user_min_fee.clear();
    }

    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) -> Result<(), BatcherError> {
        let gas_price = match self.get_gas_price().await {
            Some(price) => price,
            None => {
                error!("Failed to get gas price");
                return Err(BatcherError::GasPriceError);
            }
        };

        while let Some(finalized_batch) = self.is_batch_ready(block_number, gas_price).await {
            let batch_finalization_result = self
                .finalize_batch(block_number, finalized_batch, gas_price)
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
        let signatures: Vec<_> = finalized_batch
            .iter()
            .map(|entry| &entry.signature)
            .cloned()
            .collect();

        let nonces: Vec<_> = finalized_batch
            .iter()
            .map(|entry| entry.nonced_verification_data.nonce)
            .collect();

        let max_fees: Vec<_> = finalized_batch
            .iter()
            .map(|entry| entry.nonced_verification_data.max_fee)
            .collect();

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
        .map_err(|e| BatcherError::TaskCreationError(e.to_string()))?;

        info!("Batch sent to S3 with name: {}", file_name);

        info!("Uploading batch to contract");
        let batch_data_pointer: String = "".to_owned() + &self.download_endpoint + "/" + &file_name;

        let num_proofs_in_batch = leaves.len();

        let gas_per_proof = (CONSTANT_GAS_COST
            + ADDITIONAL_SUBMISSION_GAS_COST_PER_PROOF * num_proofs_in_batch as u128)
            / num_proofs_in_batch as u128;

        let fee_per_proof = U256::from(gas_per_proof) * gas_price;
        let fee_for_aggregator = U256::from(AGGREGATOR_GAS_COST) * gas_price;

        let fee_params = CreateNewTaskFeeParams::new(fee_for_aggregator, fee_per_proof, gas_price);

        let signatures = signatures
            .iter()
            .enumerate()
            .map(|(i, signature)| SignatureData::new(signature, nonces[i], max_fees[i]))
            .collect();

        match self
            .create_new_task(
                *batch_merkle_root,
                batch_data_pointer,
                leaves,
                signatures,
                fee_params,
            )
            .await
        {
            Ok(receipt) => {
                if let Some(gas_used) = receipt.gas_used {
                    info!("Gas used to create new task: {}", gas_used);
                }
                info!("Batch verification task created on Aligned contract");
                Ok(())
            }
            Err(e) => {
                error!(
                    "Failed to send batch to contract, batch will be lost: {:?}",
                    e
                );

                Err(e)
            }
        }
    }

    async fn create_new_task(
        &self,
        batch_merkle_root: [u8; 32],
        batch_data_pointer: String,
        leaves: Vec<[u8; 32]>,
        signatures: Vec<SignatureData>,
        fee_params: CreateNewTaskFeeParams,
    ) -> Result<TransactionReceipt, BatcherError> {
        // pad leaves to next power of 2
        let padded_leaves = Self::pad_leaves(leaves);

        info!("Creating task for: 0x{}", hex::encode(batch_merkle_root));

        match try_create_new_task(
            batch_merkle_root,
            batch_data_pointer.clone(),
            padded_leaves.clone(),
            signatures.clone(),
            fee_params.clone(),
            &self.payment_service,
        )
        .await
        {
            Ok(receipt) => Ok(receipt),
            Err(BatcherSendError::TransactionReverted(err)) => {
                // dont retry with fallback
                // just return the error
                warn!("Transaction reverted {:?}", err);

                Err(BatcherError::TransactionSendError)
            }
            Err(_) => {
                let receipt = try_create_new_task(
                    batch_merkle_root,
                    batch_data_pointer,
                    padded_leaves,
                    signatures,
                    fee_params,
                    &self.payment_service_fallback,
                )
                .await?;

                Ok(receipt)
            }
        }
    }

    fn pad_leaves(leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
        let leaves_len = leaves.len();
        let last_leaf = leaves[leaves_len - 1];
        leaves
            .into_iter()
            .chain(repeat(last_leaf).take(leaves_len.next_power_of_two() - leaves_len))
            .collect()
    }

    /// Only relevant for testing and for users to easily use Aligned
    fn is_nonpaying(&self, addr: &Address) -> bool {
        self.non_paying_config
            .as_ref()
            .is_some_and(|non_paying_config| non_paying_config.address == *addr)
    }

    /// Only relevant for testing and for users to easily use Aligned
    async fn handle_nonpaying_msg(
        self: Arc<Self>,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        client_msg: ClientMessage,
    ) -> Result<(), Error> {
        let non_paying_config = self.non_paying_config.as_ref().unwrap();
        let addr = non_paying_config.replacement.address();

        let user_balance = self.get_user_balance(&addr).await;

        if user_balance == U256::from(0) {
            error!("Insufficient funds for address {:?}", addr);
            send_message(
                ws_conn_sink.clone(),
                ValidityResponseMessage::InsufficientBalance(addr),
            )
            .await;
            return Ok(()); // Send error message to the client and return
        }

        if client_msg.verification_data.verification_data.proof.len() <= self.max_proof_size {
            // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
            if self.pre_verification_is_enabled
                && !zk_utils::verify(&client_msg.verification_data.verification_data).await
            {
                error!("Invalid proof detected. Verification failed.");
                send_message(ws_conn_sink.clone(), ValidityResponseMessage::InvalidProof).await;
                return Ok(()); // Send error message to the client and return
            }

            let nonced_verification_data = {
                let mut batch_state = self.batch_state.lock().await;

                let nonpaying_nonce = match batch_state.user_nonces.entry(addr) {
                    Entry::Occupied(o) => o.into_mut(),
                    Entry::Vacant(vacant) => {
                        let nonce = match self.payment_service.user_nonces(addr).call().await {
                            Ok(nonce) => nonce,
                            Err(e) => {
                                error!("Failed to get nonce for address {:?}: {:?}", addr, e);
                                send_message(
                                    ws_conn_sink.clone(),
                                    ValidityResponseMessage::InvalidNonce,
                                )
                                .await;

                                return Ok(());
                            }
                        };

                        vacant.insert(nonce)
                    }
                };

                debug!("non paying nonce: {:?}", nonpaying_nonce);

                let mut nonce_bytes = [0u8; 32];
                nonpaying_nonce.to_big_endian(&mut nonce_bytes);
                *nonpaying_nonce += U256::one();

                NoncedVerificationData::new(
                    client_msg.verification_data.verification_data.clone(),
                    nonce_bytes,
                    DEFAULT_MAX_FEE_PER_PROOF.into(), // 13_000 gas per proof * 100 gwei gas price (upper bound)
                    self.chain_id,
                )
            };

            let client_msg = ClientMessage::new(
                nonced_verification_data.clone(),
                non_paying_config.replacement.clone(),
            );

            let batch_state = self.batch_state.lock().await;
            self.clone()
                .add_to_batch(
                    batch_state,
                    nonced_verification_data,
                    ws_conn_sink.clone(),
                    client_msg.signature,
                    non_paying_config.address,
                )
                .await;
        } else {
            error!("Proof is too large");
            send_message(ws_conn_sink.clone(), ValidityResponseMessage::ProofTooLarge).await;
            return Ok(()); // Send error message to the client and return
        };

        info!("Verification data message handled");

        send_message(ws_conn_sink, ValidityResponseMessage::Valid).await;
        Ok(())
    }

    async fn get_user_balance(&self, addr: &Address) -> U256 {
        match self.payment_service.user_balances(*addr).call().await {
            Ok(val) => val,
            Err(_) => match self
                .payment_service_fallback
                .user_balances(*addr)
                .call()
                .await
            {
                Ok(balance) => balance,
                Err(_) => {
                    warn!("Failed to get balance for address {:?}", addr);
                    U256::zero()
                }
            },
        }
    }

    async fn user_balance_is_unlocked(&self, addr: &Address) -> bool {
        let unlock_block = match self.payment_service.user_unlock_block(*addr).call().await {
            Ok(val) => val,
            Err(_) => match self
                .payment_service_fallback
                .user_unlock_block(*addr)
                .call()
                .await
            {
                Ok(unlock_block) => unlock_block,
                Err(_) => {
                    warn!("Failed to get unlock block for address {:?}", addr);
                    U256::zero()
                }
            },
        };

        unlock_block != U256::zero()
    }

    async fn get_gas_price(&self) -> Option<U256> {
        match self.eth_ws_provider.get_gas_price().await {
            Ok(gas_price) => Some(gas_price), // this is the block's max priority gas price, not the base fee
            Err(_) => match self.eth_ws_provider_fallback.get_gas_price().await {
                Ok(gas_price) => Some(gas_price),
                Err(_) => {
                    warn!("Failed to get gas price");
                    None
                }
            },
        }
    }
}

async fn send_batch_inclusion_data_responses(
    finalized_batch: Vec<BatchQueueEntry>,
    batch_merkle_tree: &MerkleTree<VerificationCommitmentBatch>,
) -> Result<(), BatcherError> {
    for (vd_batch_idx, entry) in finalized_batch.iter().enumerate() {
        let batch_inclusion_data = BatchInclusionData::new(vd_batch_idx, batch_merkle_tree);
        let response = ResponseMessage::BatchInclusionData(batch_inclusion_data);

        let serialized_response = cbor_serialize(&response)
            .map_err(|e| BatcherError::SerializationError(e.to_string()))?;

        let sending_result = entry
            .messaging_sink
            .write()
            .await
            .send(Message::binary(serialized_response))
            .await;

        match sending_result {
            Err(Error::AlreadyClosed) => (),
            Err(e) => error!("Error while sending batch inclusion data response: {}", e),
            Ok(_) => (),
        }

        info!("Response sent");
    }

    Ok(())
}

async fn send_message<T: Serialize>(
    ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    message: T,
) {
    match cbor_serialize(&message) {
        Ok(serialized_response) => {
            if let Err(err) = ws_conn_sink
                .write()
                .await
                .send(Message::binary(serialized_response))
                .await
            {
                error!("Error while sending message: {}", err)
            }
        }
        Err(e) => error!("Error while serializing message: {}", e),
    }
}
