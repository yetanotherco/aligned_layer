use crate::zk::{Proof, VerificationError};

#[derive(Debug)]
pub enum ProofQueueError {
    QueueMaxCapacity,
    InvalidProof(VerificationError),
}

pub struct ProofsQueue {
    max_proofs_in_queue: u16,
    proofs: Vec<Proof>,
}

impl ProofsQueue {
    pub fn new(max_proofs_in_queue: u16) -> Self {
        Self {
            max_proofs_in_queue,
            proofs: vec![],
        }
    }

    pub fn proofs(&self) -> &[Proof] {
        &self.proofs
    }

    /// Clears the queue and returns all the current proofs in it
    pub fn clear(&mut self) -> Vec<Proof> {
        self.proofs.drain(0..self.proofs.len()).collect()
    }

    pub fn add_proof(&mut self, proof: Proof) -> Result<(), ProofQueueError> {
        if let Err(err) = proof.verify() {
            return Err(ProofQueueError::InvalidProof(err));
        };

        if self.proofs.len() as u16 >= self.max_proofs_in_queue {
            return Err(ProofQueueError::QueueMaxCapacity);
        }

        self.proofs.push(proof);

        Ok(())
    }
}
