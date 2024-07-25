extern crate core;

use aligned_sdk::eth::batcher_payment_service::SignatureData;
use config::NonPayingConfig;
use dotenv::dotenv;
use ethers::signers::Signer;

use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use aligned_sdk::core::types::{
    BatchInclusionData, ClientMessage, NoncedVerificationData, ResponseMessage,
    VerificationCommitmentBatch, VerificationDataCommitment,
};
use aws_sdk_s3::client::Client as S3Client;
use eth::BatcherPaymentService;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use ethers::types::{Address, Signature, U256};
use futures_util::stream::{self, SplitSink};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use lambdaworks_crypto::merkle_tree::traits::IsMerkleTreeBackend;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::WebSocketStream;
use types::batch_queue::BatchQueue;
use types::errors::BatcherError;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput};

mod config;
mod eth;
pub mod gnark;
pub mod halo2;
pub mod risc_zero;
pub mod s3;
pub mod sp1;
pub mod jolt;
pub mod types;
mod zk_utils;

const AGGREGATOR_COST: u128 = 400000;
const BATCHER_SUBMISSION_BASE_COST: u128 = 100000;
const ADDITIONAL_SUBMISSION_COST_PER_PROOF: u128 = 13_000;
const CONSTANT_COST: u128 = AGGREGATOR_COST + BATCHER_SUBMISSION_BASE_COST;
const MIN_BALANCE_PER_PROOF: u128 = ADDITIONAL_SUBMISSION_COST_PER_PROOF * 100_000_000_000; // 100 Gwei = 0.0000001 ether (high gas price)

pub struct Batcher {
    s3_client: S3Client,
    s3_bucket_name: String,
    eth_ws_provider: Provider<Ws>,
    payment_service: BatcherPaymentService,
    batch_queue: Mutex<BatchQueue>,
    max_block_interval: u64,
    min_batch_len: usize,
    max_proof_size: usize,
    max_batch_size: usize,
    last_uploaded_batch_block: Mutex<u64>,
    pre_verification_is_enabled: bool,
    non_paying_config: Option<NonPayingConfig>,
    user_nonces: Mutex<HashMap<Address, U256>>,
    user_proof_count_in_batch: Mutex<HashMap<Address, u64>>,
}

impl Batcher {
    pub async fn new(config_file: String) -> Self {
        dotenv().ok();
        let s3_bucket_name =
            env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME not found in environment");

        let s3_client = s3::create_client().await;

        let config = ConfigFromYaml::new(config_file);
        let deployment_output =
            ContractDeploymentOutput::new(config.aligned_layer_deployment_config_file_path);

        let eth_ws_provider =
            Provider::connect_with_reconnects(&config.eth_ws_url, config.batcher.eth_ws_reconnects)
                .await
                .expect("Failed to get ethereum websocket provider");

        let eth_rpc_provider =
            eth::get_provider(config.eth_rpc_url.clone()).expect("Failed to get provider");

        // FIXME(marian): We are getting just the last block number right now, but we should really
        // have the last submitted batch block registered and query it when the batcher is initialized.
        let last_uploaded_batch_block = eth_rpc_provider
            .get_block_number()
            .await
            .expect("Failed to get block number")
            .try_into()
            .unwrap();

        let payment_service = eth::get_batcher_payment_service(
            eth_rpc_provider,
            config.ecdsa,
            deployment_output.addresses.batcher_payment_service,
        )
        .await
        .expect("Failed to get Batcher Payment Service contract");

        let non_paying_config = if let Some(non_paying_config) = config.batcher.non_paying {
            warn!("Non-paying address configuration detected. Will replace non-paying address {} with configured address.",
                non_paying_config.address);
            Some(NonPayingConfig::from_yaml_config(non_paying_config, &payment_service).await)
        } else {
            None
        };

        let user_nonces = Mutex::new(HashMap::new());

        Self {
            s3_client,
            s3_bucket_name,
            eth_ws_provider,
            payment_service,
            batch_queue: Mutex::new(BatchQueue::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_len: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
            non_paying_config,
            user_nonces,
            user_proof_count_in_batch: Mutex::new(HashMap::new()),
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

        while let Some(block) = stream.next().await {
            let batcher = self.clone();
            let block_number = block.number.unwrap();
            let block_number = u64::try_from(block_number).unwrap();
            info!("Received new block: {}", block_number);
            tokio::spawn(async move {
                if let Err(e) = batcher.handle_new_block(block_number).await {
                    error!("Error when handling new block: {:?}", e);
                };
            });
        }

        Ok(())
    }

    async fn handle_connection(self: Arc<Self>, raw_stream: TcpStream, addr: SocketAddr) {
        info!("Incoming TCP connection from: {}", addr);
        let ws_stream = tokio_tungstenite::accept_async(raw_stream)
            .await
            .expect("Error during the websocket handshake occurred");

        debug!("WebSocket connection established: {}", addr);
        let (outgoing, incoming) = ws_stream.split();
        let outgoing = Arc::new(RwLock::new(outgoing));

        let protocol_version_msg = ResponseMessage::ProtocolVersion(
            aligned_sdk::communication::protocol::EXPECTED_PROTOCOL_VERSION,
        );

        let serialized_protocol_version_msg = serde_json::to_vec(&protocol_version_msg)
            .expect("Could not serialize protocol version message");

        outgoing
            .write()
            .await
            .send(Message::binary(serialized_protocol_version_msg))
            .await
            .expect("Could not send protocol version message");

        match incoming
            .try_filter(|msg| future::ready(msg.is_text()))
            .try_for_each(|msg| self.clone().handle_message(msg, outgoing.clone()))
            .await
        {
            Err(e) => error!("Unexpected error: {}", e),
            Ok(_) => info!("{} disconnected", &addr),
        }
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Result<(), Error> {
        // Deserialize verification data from message
        let client_msg: ClientMessage =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        info!("Verifying message signature...");
        if let Ok(addr) = client_msg.verify_signature() {
            info!("Message signature verified");
            if self.is_nonpaying(&addr) {
                return self
                    .handle_nonpaying_msg(ws_conn_sink.clone(), client_msg)
                    .await;
            } else {
                if !self.check_user_balance(&addr).await {
                    send_error_message(
                        ws_conn_sink.clone(),
                        ResponseMessage::InsufficientBalanceError(addr),
                    )
                    .await;

                    return Ok(());
                }

                let nonce = U256::from_big_endian(client_msg.verification_data.nonce.as_slice());
                let nonced_verification_data = client_msg.verification_data;
                if nonced_verification_data.verification_data.proof.len() <= self.max_proof_size {
                    // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
                    if self.pre_verification_is_enabled
                        && !zk_utils::verify(&nonced_verification_data.verification_data)
                    {
                        error!("Invalid proof detected. Verification failed.");
                        send_error_message(
                            ws_conn_sink.clone(),
                            ResponseMessage::VerificationError(),
                        )
                        .await;
                        return Ok(()); // Send error message to the client and return
                    }

                    // Doing nonce verification after proof verification to avoid unnecessary nonce increment
                    if !self.check_nonce_and_increment(addr, nonce).await {
                        send_error_message(
                            ws_conn_sink.clone(),
                            ResponseMessage::InvalidNonceError,
                        )
                        .await;
                        return Ok(()); // Send error message to the client and return
                    }

                    self.add_to_batch(
                        nonced_verification_data,
                        ws_conn_sink.clone(),
                        client_msg.signature,
                    )
                    .await;
                } else {
                    error!("Proof is too large");
                    send_error_message(ws_conn_sink.clone(), ResponseMessage::ProofTooLargeError())
                        .await;
                    return Ok(()); // Send error message to the client and return
                };

                info!("Verification data message handled");

                return Ok(());
            }
        } else {
            error!("Signature verification error");
            send_error_message(
                ws_conn_sink.clone(),
                ResponseMessage::SignatureVerificationError(),
            )
            .await;
            Ok(()) // Send error message to the client and return
        }
    }

    // Checks user has sufficient balance
    // If user has sufficient balance, increments the user's proof count in the batch
    async fn check_user_balance(&self, addr: &Address) -> bool {
        let mut user_proof_counts = self.user_proof_count_in_batch.lock().await;
        let user_proofs_in_batch = user_proof_counts.get(addr).unwrap_or(&0).clone() + 1;

        let user_balance = self.get_user_balance(addr).await;

        let min_balance = U256::from(user_proofs_in_batch) * U256::from(MIN_BALANCE_PER_PROOF);
        if user_balance < min_balance {
            return false;
        }

        user_proof_counts.insert(*addr, user_proofs_in_batch);
        true
    }

    async fn check_nonce_and_increment(&self, addr: Address, nonce: U256) -> bool {
        let mut user_nonces = self.user_nonces.lock().await;

        let expected_user_nonce = match user_nonces.get(&addr) {
            Some(nonce) => *nonce,
            None => {
                let user_nonce = match self.payment_service.user_nonces(addr).call().await {
                    Ok(nonce) => nonce,
                    Err(e) => {
                        error!("Failed to get user nonce for address {:?}: {:?}", addr, e);
                        return false;
                    }
                };

                user_nonces.insert(addr, user_nonce);
                user_nonce
            }
        };

        if nonce != expected_user_nonce {
            error!(
                "Invalid nonce for address {addr} Expected: {:?}, got: {:?}",
                expected_user_nonce, nonce
            );
            return false;
        }

        user_nonces.insert(addr, nonce + U256::one());
        true
    }

    /// Adds verification data to the current batch queue.
    async fn add_to_batch(
        self: Arc<Self>,
        verification_data: NoncedVerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        proof_submitter_sig: Signature,
    ) {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        info!("Calculating verification data commitments...");
        let verification_data_comm = verification_data.clone().into();
        info!("Adding verification data to batch...");
        batch_queue_lock.push((
            verification_data,
            verification_data_comm,
            ws_conn_sink,
            proof_submitter_sig,
        ));
        info!("Current batch queue length: {}", batch_queue_lock.len());
    }

    /// Given a new block number listened from the blockchain, checks if the current batch is ready to be posted.
    /// There are essentially two conditions to be checked:
    ///     * Has the current batch reached the minimum size to be posted?
    ///     * Has the received block number surpassed the maximum interval with respect to the last posted batch block?
    /// An extra sanity check is made to check if the batch size is 0, since it does not make sense to post
    /// an empty batch, even if the block interval has been reached.
    /// Once the batch meets the conditions for submission, it check if it needs to be splitted into smaller batches,
    /// depending on the configured maximum batch size. The batch is splitted at the index where the max size is surpassed,
    /// and all the elements up to that index are copied and cleared from the batch queue. The copy is then passed to the
    /// `finalize_batch` function.
    async fn is_batch_ready(&self, block_number: u64) -> Option<BatchQueue> {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        let current_batch_len = batch_queue_lock.len();

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

        let batch_verification_data: Vec<NoncedVerificationData> = batch_queue_lock
            .iter()
            .map(|(vd, _, _, _)| vd.clone())
            .collect();

        let current_batch_size = serde_json::to_vec(&batch_verification_data).unwrap().len();

        // check if the current batch needs to be splitted into smaller batches
        if current_batch_size > self.max_batch_size {
            info!("Batch max size exceded. Splitting current batch...");
            let mut acc_batch_size = 0;
            let mut finalized_batch_idx = 0;
            for (idx, (verification_data, _, _, _)) in batch_queue_lock.iter().enumerate() {
                acc_batch_size += serde_json::to_vec(verification_data).unwrap().len();
                if acc_batch_size > self.max_batch_size {
                    finalized_batch_idx = idx;
                    break;
                }
            }
            let finalized_batch = batch_queue_lock.drain(..finalized_batch_idx).collect();
            return Some(finalized_batch);
        }

        // A copy of the batch is made to be returned and the current batch is cleared
        let finalized_batch = batch_queue_lock.clone();
        batch_queue_lock.clear();

        // Clear the user proofs in batch as well
        self.user_proof_count_in_batch.lock().await.clear();

        Some(finalized_batch)
    }

    /// Takes the finalized batch as input and builds the merkle tree, posts verification data batch
    /// to s3, creates new task in Aligned contract and sends responses to all clients that added proofs
    /// to the batch. The last uploaded batch block is updated once the task is created in Aligned.
    async fn finalize_batch(
        &self,
        block_number: u64,
        finalized_batch: BatchQueue,
    ) -> Result<(), BatcherError> {
        let batch_verification_data: Vec<NoncedVerificationData> = finalized_batch
            .clone()
            .into_iter()
            .map(|(data, _, _, _)| data)
            .collect();

        let batch_bytes = serde_json::to_vec(batch_verification_data.as_slice())
            .expect("Failed to serialize batch");

        info!("Finalizing batch. Length: {}", finalized_batch.len());
        let batch_data_comm: Vec<VerificationDataCommitment> = finalized_batch
            .clone()
            .into_iter()
            .map(|(_, data_comm, _, _)| data_comm)
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

        let signatures = finalized_batch
            .iter()
            .map(|(_, _, _, sig)| sig)
            .cloned()
            .collect();

        let nonces = finalized_batch
            .iter()
            .map(|(nonced_vd, _, _, _)| nonced_vd.nonce)
            .collect();

        if let Err(e) = self
            .submit_batch(
                &batch_bytes,
                &batch_merkle_tree.root,
                leaves,
                signatures,
                nonces,
            )
            .await
        {
            for (_, _, ws_sink, _) in finalized_batch.iter() {
                let merkle_root = hex::encode(batch_merkle_tree.root);
                send_error_message(
                    ws_sink.clone(),
                    ResponseMessage::CreateNewTaskError(merkle_root),
                )
                .await
            }
            return Err(e);
        };

        send_batch_inclusion_data_responses(finalized_batch, &batch_merkle_tree).await;

        Ok(())
    }

    /// Receives new block numbers, checks if conditions are met for submission and
    /// finalizes the batch.
    async fn handle_new_block(&self, block_number: u64) -> Result<(), BatcherError> {
        while let Some(finalized_batch) = self.is_batch_ready(block_number).await {
            self.finalize_batch(block_number, finalized_batch).await?;
        }
        Ok(())
    }

    /// Post batch to s3 and submit new task to Ethereum
    async fn submit_batch(
        &self,
        batch_bytes: &[u8],
        batch_merkle_root: &[u8; 32],
        leaves: Vec<[u8; 32]>,
        signatures: Vec<Signature>,
        nonces: Vec<[u8; 32]>,
    ) -> Result<(), BatcherError> {
        let s3_client = self.s3_client.clone();
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);
        let file_name = batch_merkle_root_hex.clone() + ".json";

        info!("Uploading batch to S3...");
        s3::upload_object(
            &s3_client,
            &self.s3_bucket_name,
            batch_bytes.to_vec(),
            &file_name,
        )
        .await
        .expect("Failed to upload object to S3");

        info!("Batch sent to S3 with name: {}", file_name);

        info!("Uploading batch to contract");
        let payment_service = &self.payment_service;
        let batch_data_pointer = "https://".to_owned() + &self.s3_bucket_name + "/" + &file_name;

        let num_proofs_in_batch = leaves.len();

        let gas_per_proof = (CONSTANT_COST
            + ADDITIONAL_SUBMISSION_COST_PER_PROOF * num_proofs_in_batch as u128)
            / num_proofs_in_batch as u128;

        let signatures = signatures
            .iter()
            .enumerate()
            .map(|(i, signature)| SignatureData::new(signature, nonces[i]))
            .collect();

        if let Err(e) = eth::create_new_task(
            payment_service,
            *batch_merkle_root,
            batch_data_pointer,
            leaves,
            signatures,
            AGGREGATOR_COST.into(), // FIXME(uri): This value should be read from aligned_layer/contracts/script/deploy/config/devnet/batcher-payment-service.devnet.config.json
            gas_per_proof.into(), //FIXME(uri): This value should be read from aligned_layer/contracts/script/deploy/config/devnet/batcher-payment-service.devnet.config.json
        )
        .await
        {
            error!("Failed to create batch verification task: {}", e);
            return Err(BatcherError::TaskCreationError(e.to_string()));
        }

        info!("Batch verification task created on Aligned contract");
        Ok(())
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

        // The nonpaying nonce is locked through the entire message processing so that
        // another incoming connections using the nonpaying address don't desync its nonce
        let mut nonpaying_nonce = non_paying_config.nonce.lock().await;
        let addr = non_paying_config.replacement.address();

        let mut nonce_bytes = [0u8; 32];
        nonpaying_nonce.to_big_endian(&mut nonce_bytes);
        *nonpaying_nonce += U256::one();

        let verifcation_data = NoncedVerificationData::new(
            client_msg.verification_data.verification_data.clone(),
            nonce_bytes,
        );

        let client_msg =
            ClientMessage::new(verifcation_data, non_paying_config.replacement.clone());

        let user_balance = self
            .payment_service
            .user_balances(addr)
            .call()
            .await
            .unwrap_or_default();

        if user_balance == U256::from(0) {
            error!("Insufficient funds for address {:?}", addr);
            send_error_message(
                ws_conn_sink.clone(),
                ResponseMessage::InsufficientBalanceError(addr),
            )
            .await;
            return Ok(()); // Send error message to the client and return
        }

        let nonce = U256::from_big_endian(client_msg.verification_data.nonce.as_slice());
        let nonced_verification_data = client_msg.verification_data;
        if nonced_verification_data.verification_data.proof.len() <= self.max_proof_size {
            // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
            if self.pre_verification_is_enabled
                && !zk_utils::verify(&nonced_verification_data.verification_data)
            {
                error!("Invalid proof detected. Verification failed.");
                send_error_message(ws_conn_sink.clone(), ResponseMessage::VerificationError())
                    .await;
                return Ok(()); // Send error message to the client and return
            }

            // Doing nonce verification after proof verification to avoid unnecessary nonce increment
            if !self.check_nonce_and_increment(addr, nonce).await {
                send_error_message(ws_conn_sink.clone(), ResponseMessage::InvalidNonceError).await;
                return Ok(()); // Send error message to the client and return
            }

            self.clone()
                .add_to_batch(
                    nonced_verification_data,
                    ws_conn_sink.clone(),
                    client_msg.signature,
                )
                .await;
        } else {
            error!("Proof is too large");
            send_error_message(ws_conn_sink.clone(), ResponseMessage::ProofTooLargeError()).await;
            return Ok(()); // Send error message to the client and return
        };

        info!("Verification data message handled");

        Ok(())
    }

    async fn get_user_balance(&self, addr: &Address) -> U256 {
        self.payment_service
            .user_balances(*addr)
            .call()
            .await
            .unwrap_or_default()
    }
}

async fn send_batch_inclusion_data_responses(
    finalized_batch: BatchQueue,
    batch_merkle_tree: &MerkleTree<VerificationCommitmentBatch>,
) {
    stream::iter(finalized_batch.iter())
        .enumerate()
        .for_each(|(vd_batch_idx, (_, _, ws_sink, _))| async move {
            let batch_inclusion_data = BatchInclusionData::new(vd_batch_idx, batch_merkle_tree);
            let response = ResponseMessage::BatchInclusionData(batch_inclusion_data);

            let serialized_response =
                serde_json::to_vec(&response).expect("Could not serialize response");

            let sending_result = ws_sink
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
        })
        .await;
}

async fn send_error_message(
    ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    error_message: ResponseMessage,
) {
    let serialized_response =
        serde_json::to_vec(&error_message).expect("Could not serialize response");

    // Send error message
    ws_conn_sink
        .write()
        .await
        .send(Message::binary(serialized_response))
        .await
        .expect("Failed to send error message");
}
