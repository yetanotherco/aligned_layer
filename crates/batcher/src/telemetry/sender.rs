use ethers::types::H256;

#[derive(Debug, serde::Serialize)]
pub enum TraceMessage {
    CreatingTask([u8; 32]),
    TaskCreated(H256),
    TaskCreationFailed(H256),
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageTask {
    merkle_root: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageTaskStarted {
    merkle_root: String,
    fee_per_proof: String,
    num_proofs_in_batch: usize,
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageTaskSentToEthereum {
    merkle_root: String,
    tx_hash: H256,
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageNewBatch {
    merkle_root: String,
    proof_count: usize,
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageTaskError {
    merkle_root: String,
    error: String,
}

pub struct TelemetrySender {
    base_url: String,
    client: reqwest::Client,
}

impl TelemetrySender {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::new();
        Self { base_url, client }
    }

    pub fn get_full_url(&self, path: &str) -> String {
        format!("{}/api/{}", self.base_url, path)
    }

    pub async fn init_task_trace(&self, batch_merkle_root: &str) -> Result<(), reqwest::Error> {
        let url = self.get_full_url("initBatcherTaskTrace");
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageTask {
            merkle_root: formatted_merkle_root,
        };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }

    pub async fn task_sent(
        &self,
        batch_merkle_root: &str,
        tx_hash: H256,
    ) -> Result<(), reqwest::Error> {
        let url = self.get_full_url("batcherTaskSent");
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageTaskSentToEthereum {
            merkle_root: formatted_merkle_root,
            tx_hash,
        };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }

    pub async fn task_created(
        &self,
        batch_merkle_root: &str,
        fee_per_proof: String,
        num_proofs_in_batch: usize,
    ) -> Result<(), reqwest::Error> {
        let url = self.get_full_url("batcherTaskStarted");
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageTaskStarted {
            merkle_root: formatted_merkle_root,
            fee_per_proof,
            num_proofs_in_batch,
        };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }

    pub async fn task_uploaded_to_s3(&self, batch_merkle_root: &str) -> Result<(), reqwest::Error> {
        let url = self.get_full_url("batcherTaskUploadedToS3");
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageTask {
            merkle_root: formatted_merkle_root,
        };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }

    pub async fn task_creation_failed(
        &self,
        batch_merkle_root: &str,
        reason: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self.get_full_url("batcherTaskCreationFailed");
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageTaskError {
            merkle_root: formatted_merkle_root,
            error: reason.to_string(),
        };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }
}
