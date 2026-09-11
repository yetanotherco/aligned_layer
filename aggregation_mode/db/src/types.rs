use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        BigDecimal, Uuid,
    },
    Type,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, serde::Serialize)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Processing,
    Verified,
}

#[derive(Debug, Clone, FromRow)]
pub struct Task {
    pub task_id: Uuid,
    pub address: String,
    pub proving_system_id: i32,
    pub proof: Vec<u8>,
    pub program_commitment: Vec<u8>,
    pub merkle_path: Option<Vec<u8>>,
    pub status: TaskStatus,
    pub status_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Payment {
    pub payment_event_id: Uuid,
    pub address: String,
    pub amount: i32,
    pub started_at: BigDecimal,
    pub valid_until: BigDecimal,
    pub tx_hash: String,
}
