use db::{orchestrator::DbOrchestrator, retry::RetryConfig};
use sqlx::types::BigDecimal;

// Retry/backoff behavior summary for DB queries (see
// aggregation_mode/db/src/orchestrator.rs:next_back_off_delay for implementation)
//
// 1) Max wait time between failures if all retries fail:
//    The sleep between retries is capped at 30 seconds (RETRY_MAX_DELAY_SECONDS).
//
// 2) Wait before each retry attempt with the current config
//    (start = 500ms, factor = 4.0, max retries = 5):
//
//    retry 1: 0.5s
//    retry 2: 2.0s
//    retry 3: 8.0s
//    retry 4: 30s (capped; 32s would have been next)
//    retry 5: 30s
//
//    Worst-case total sleep time across all retries: 70.5 seconds -> 5 blocks of ethereum waiting,
//    plus the execution time of each DB attempt.
/// Initial delay before first retry attempt (in milliseconds)
const RETRY_MIN_DELAY_MILLIS: u64 = 500;
/// Exponential backoff multiplier for retry delays
const RETRY_FACTOR: f32 = 4.0;
/// Maximum number of retry attempts
const RETRY_MAX_TIMES: usize = 5;
/// Maximum delay between retry attempts (in seconds)
const RETRY_MAX_DELAY_SECONDS: u64 = 30;

#[derive(Clone, Debug)]
pub struct Db {
    orchestrator: DbOrchestrator,
}

#[derive(Debug, Clone)]
pub enum DbError {
    ConnectError(String),
}

impl Db {
    pub async fn try_new(connection_urls: &[String]) -> Result<Self, DbError> {
        let orchestrator = DbOrchestrator::try_new(
            connection_urls,
            RetryConfig {
                min_delay_millis: RETRY_MIN_DELAY_MILLIS,
                factor: RETRY_FACTOR,
                max_times: RETRY_MAX_TIMES,
                max_delay_seconds: RETRY_MAX_DELAY_SECONDS,
            },
        )
        .map_err(|e| DbError::ConnectError(e.to_string()))?;

        Ok(Self { orchestrator })
    }

    pub async fn insert_payment_event(
        &self,
        address: &str,
        started_at: &BigDecimal,
        amount: &BigDecimal,
        valid_until: &BigDecimal,
        tx_hash: &str,
    ) -> Result<(), sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query(
                    "INSERT INTO payment_events (address, started_at, amount, valid_until, tx_hash)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (tx_hash) DO NOTHING",
                )
                .bind(address.to_lowercase())
                .bind(started_at)
                .bind(amount)
                .bind(valid_until)
                .bind(tx_hash)
                .execute(&pool)
                .await?;

                Ok(())
            })
            .await
    }
}
