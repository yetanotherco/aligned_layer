extern crate core;

use std::net::SocketAddr;
use std::sync::Arc;

use aligned_sdk::core::types::{
    BatchInclusionData, ClientMessage, ResponseMessage, VerificationCommitmentBatch,
    VerificationData, VerificationDataCommitment,
};
use aws_sdk_s3::client::Client as S3Client;
use eth::BatcherPaymentService;
use ethers::prelude::{Middleware, Provider};
use ethers::providers::Ws;
use ethers::types::{Address, U256};
use futures_util::stream::{self, SplitSink};
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use log::{debug, error, info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::{Error, Message};
use tokio_tungstenite::WebSocketStream;
use types::batch_queue::BatchQueue;
use types::errors::BatcherError;

use crate::config::{ConfigFromYaml, ContractDeploymentOutput, NonPayingConfig};

mod config;
mod eth;
pub mod gnark;
pub mod halo2;
pub mod risc_zero;
pub mod s3;
pub mod sp1;
pub mod types;
mod zk_utils;

const S3_BUCKET_NAME: &str = "storage.alignedlayer.com";

pub struct Batcher {
    s3_client: S3Client,
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
}

impl Batcher {
    pub async fn new(config_file: String) -> Self {
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

        if let Some(non_paying_config) = &config.batcher.non_paying {
            warn!("Non-paying address configuration detected. Will replace non-paying address {} with configured address {}.",
                non_paying_config.address, non_paying_config.replacement);
        }

        Self {
            s3_client,
            eth_ws_provider,
            payment_service,
            batch_queue: Mutex::new(BatchQueue::new()),
            max_block_interval: config.batcher.block_interval,
            min_batch_len: config.batcher.batch_size_interval,
            max_proof_size: config.batcher.max_proof_size,
            max_batch_size: config.batcher.max_batch_size,
            last_uploaded_batch_block: Mutex::new(last_uploaded_batch_block),
            pre_verification_is_enabled: config.batcher.pre_verification_is_enabled,
            non_paying_config: config.batcher.non_paying,
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
            aligned_sdk::communication::protocol::CURRENT_PROTOCOL_VERSION,
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
            Err(Error::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                info!("Client {} reset connection", &addr)
            }
            Err(e) => error!("Unexpected error: {}", e),
            Ok(_) => info!("{} disconnected", &addr),
        }
    }

    /// Handle an individual message from the client.
    async fn handle_message(
        self: Arc<Self>,
        message: Message,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
    ) -> Result<(), tokio_tungstenite::tungstenite::Error> {
        // Deserialize verification data from message
        let client_msg: ClientMessage =
            serde_json::from_str(message.to_text().expect("Message is not text"))
                .expect("Failed to deserialize task");

        info!("Verifying message signature...");
        let submitter_addr = if let Ok(addr) = client_msg.verify_signature() {
            info!("Message signature verified");

            let mut addr = addr;
            if let Some(non_paying_config) = &self.non_paying_config {
                if addr == non_paying_config.address {
                    info!("Non-paying address detected. Replacing with configured address");
                    addr = non_paying_config.replacement;
                }
            }

            let user_balance = self
                .payment_service
                .user_balances(addr)
                .call()
                .await
                .unwrap_or_default();

            if user_balance == U256::from(0) {
                error!("Insufficient funds for address {:?}", addr);
                return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                    ProtocolError::HandshakeIncomplete,
                ));
            }

            addr
        } else {
            error!("Signature verification error");
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                ProtocolError::HandshakeIncomplete,
            ));
        };

        let verification_data = client_msg.verification_data;
        if verification_data.proof.len() <= self.max_proof_size {
            // When pre-verification is enabled, batcher will verify proofs for faster feedback with clients
            if self.pre_verification_is_enabled && !zk_utils::verify(&verification_data) {
                return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                    ProtocolError::HandshakeIncomplete,
                ));
            }
            self.add_to_batch(verification_data, ws_conn_sink.clone(), submitter_addr)
                .await;
        } else {
            // FIXME(marian): Handle this error correctly
            return Err(tokio_tungstenite::tungstenite::Error::Protocol(
                ProtocolError::HandshakeIncomplete,
            ));
        };

        info!("Verification data message handled");

        Ok(())
    }

    /// Adds verification data to the current batch queue.
    async fn add_to_batch(
        self: Arc<Self>,
        verification_data: VerificationData,
        ws_conn_sink: Arc<RwLock<SplitSink<WebSocketStream<TcpStream>, Message>>>,
        proof_submitter_addr: Address,
    ) {
        let mut batch_queue_lock = self.batch_queue.lock().await;
        info!("Calculating verification data commitments...");
        let verification_data_comm = verification_data.clone().into();
        info!("Adding verification data to batch...");
        batch_queue_lock.push((
            verification_data,
            verification_data_comm,
            ws_conn_sink,
            proof_submitter_addr,
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

        let batch_verification_data: Vec<VerificationData> = batch_queue_lock
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
        let batch_verification_data: Vec<VerificationData> = finalized_batch
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

        let submitter_addresses = finalized_batch
            .iter()
            .map(|(_, _, _, addr)| *addr)
            .collect();

        {
            let mut last_uploaded_batch_block = self.last_uploaded_batch_block.lock().await;
            // update last uploaded batch block
            *last_uploaded_batch_block = block_number;
            info!(
                "Batch Finalizer: Last uploaded batch block updated to: {}. Lock unlocked",
                block_number
            );
        }
        // Moving this outside the previous scope is a hotfix until we merge https://github.com/yetanotherco/aligned_layer/pull/365
        self.submit_batch(&batch_bytes, &batch_merkle_tree.root, submitter_addresses)
            .await;

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
        submitter_addresses: Vec<Address>,
    ) {
        let s3_client = self.s3_client.clone();
        let batch_merkle_root_hex = hex::encode(batch_merkle_root);
        info!("Batch merkle root: {}", batch_merkle_root_hex);
        let file_name = batch_merkle_root_hex.clone() + ".json";

        info!("Uploading batch to S3...");
        s3::upload_object(&s3_client, S3_BUCKET_NAME, batch_bytes.to_vec(), &file_name)
            .await
            .expect("Failed to upload object to S3");

        info!("Batch sent to S3 with name: {}", file_name);

        info!("Uploading batch to contract");
        let payment_service = &self.payment_service;
        let batch_data_pointer = "https://".to_owned() + S3_BUCKET_NAME + "/" + &file_name;

        let num_proofs_in_batch = submitter_addresses.len();

        // FIXME: This constants should be aggregated into one constants file
        const AGGREGATOR_COST: u128 = 400000;
        const BATCHER_SUBMISSION_BASE_COST: u128 = 100000;
        const ADDITIONAL_SUBMISSION_COST_PER_PROOF: u128 = 1325;
        const CONSTANT_COST: u128 = AGGREGATOR_COST + BATCHER_SUBMISSION_BASE_COST;

        let gas_per_proof = (CONSTANT_COST
            + ADDITIONAL_SUBMISSION_COST_PER_PROOF * num_proofs_in_batch as u128)
            / num_proofs_in_batch as u128;

        match eth::create_new_task(
            payment_service,
            *batch_merkle_root,
            batch_data_pointer,
            submitter_addresses,
            AGGREGATOR_COST.into(), // FIXME(uri): This value should be read from aligned_layer/contracts/script/deploy/config/devnet/batcher-payment-service.devnet.config.json
            gas_per_proof.into(), //FIXME(uri): This value should be read from aligned_layer/contracts/script/deploy/config/devnet/batcher-payment-service.devnet.config.json
        )
        .await
        {
            Ok(_) => info!("Batch verification task created on Aligned contract"),
            Err(e) => error!("Failed to create batch verification task: {}", e),
        }
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
