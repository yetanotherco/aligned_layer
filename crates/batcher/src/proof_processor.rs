use std::{collections::HashMap, sync::Arc};

use aligned_sdk::common::types::SubmitProofResponseMessage;
use ethers::abi::Address;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    oneshot, Mutex,
};

use crate::types::batch_state::BatchState;

pub enum ProofProcessorMessage {
    ValidateProof {
        response_tx: oneshot::Sender<SubmitProofResponseMessage>,
    },
    StopProcessing {
        response_tx: oneshot::Sender<bool>,
    },
    RestartProcessing,
}

pub struct ProofProcessorForwarder {
    tx: Sender<ProofProcessorMessage>,
}

impl ProofProcessorForwarder {
    /// Validates a proof for the given proof
    pub fn validate_proof(&self) {}
    /// Stops the processing of proofs and returns true when all the ongoing messages have been processed
    pub fn stop_processing(&self) {}
    /// Restarts the processing of proofs
    pub fn restart_processing(&self) {}
}

pub struct ProofProcessor {
    user_mutexes: HashMap<Address, Mutex<bool>>,
    batch_state: Arc<Mutex<BatchState>>,
    rx: Receiver<ProofProcessorMessage>,
    processing_stopped: bool,
    current_messages: Arc<Mutex<usize>>,
}

impl ProofProcessor {
    pub fn new(batch_state: Arc<Mutex<BatchState>>) -> (Self, Sender<ProofProcessorMessage>) {
        let (tx, rx) = mpsc::channel::<ProofProcessorMessage>(100);
        let processor = Self {
            batch_state,
            user_mutexes: HashMap::new(),
            rx,
            processing_stopped: false,
            current_messages: Arc::new(Mutex::new(0)),
        };
        (processor, tx)
    }

    async fn start(&mut self) {
        self.recv().await;
    }

    async fn recv(&mut self) {
        while let Some(message) = self.rx.recv().await {
            match message {
                ProofProcessorMessage::ValidateProof { response_tx } => {
                    // check if processing should stop because the batcher requested for it
                    if self.processing_stopped {
                        continue;
                    }

                    *self.current_messages.lock().await += 1;
                    let current_messages_clone = self.current_messages.clone();
                    let batch_state_clone = self.batch_state.clone();
                    tokio::task::spawn(async move {
                        // TODO: lock user mutex check, if is being attended stop
                        let result = Self::handle_validate_proof_message(batch_state_clone).await;
                        let _ = response_tx.send(result);
                        *current_messages_clone.lock().await -= 1;
                    });
                }
                ProofProcessorMessage::StopProcessing { response_tx } => {
                    self.processing_stopped = true;
                    // wait until it can lock all the mutex of the users
                    // we know that no new mutex would appear since we have set processing_stopped = true
                    for mutex in self.user_mutexes.values() {
                        let _ = mutex.lock().await;
                    }

                    let _ = response_tx.send(true);
                }
                ProofProcessorMessage::RestartProcessing => self.processing_stopped = false,
            }
        }
    }

    async fn handle_validate_proof_message(
        batch_state: Arc<Mutex<BatchState>>,
    ) -> SubmitProofResponseMessage {
        SubmitProofResponseMessage::InvalidMaxFee
    }
}
