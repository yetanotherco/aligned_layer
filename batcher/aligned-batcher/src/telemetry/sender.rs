use ethers::types::H256;

#[derive(Debug, serde::Serialize)]
pub enum TraceMessage {
    CreatingTask([u8; 32]),
    TaskCreated(H256),
    TaskCreationFailed(H256),
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageCreatingTask {
    merkle_root: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TraceMessageNewBatch {
    merkle_root: String,
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

    pub async fn send_new_batch(&self, batch_merkle_root: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/batcherNewBatch", self.base_url);
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageNewBatch { merkle_root: formatted_merkle_root };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }
    
    pub async fn send_creating_task(&self, batch_merkle_root: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/batcherTaskSent", self.base_url);
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageCreatingTask { merkle_root: formatted_merkle_root };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }

    pub async fn start_task_creation(&self, batch_merkle_root: String) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/batcherTaskStarted", self.base_url);
        let formatted_merkle_root = format!("0x{}", batch_merkle_root);
        let task = TraceMessageCreatingTask { merkle_root: formatted_merkle_root };
        self.client.post(&url).json(&task).send().await?;
        Ok(())
    }
}
