use db::{orchestrator::DbOrchestartor, retry::RetryConfig};
use sqlx::types::BigDecimal;

// Retry parameters for Db queries
/// Initial delay before first retry attempt (in milliseconds)
const RETRY_MIN_DELAY_MILLIS: u64 = 500;
/// Exponential backoff multiplier for retry delays
const RETRY_FACTOR: f32 = 2.0;
/// Maximum number of retry attempts
const RETRY_MAX_TIMES: usize = 5;
/// Maximum delay between retry attempts (in seconds)
const RETRY_MAX_DELAY_SECONDS: u64 = 30;

#[derive(Clone, Debug)]
pub struct Db {
    orchestartor: DbOrchestartor,
}

#[derive(Debug, Clone)]
pub enum DbError {
    ConnectError(String),
}

impl Db {
    pub async fn try_new(connection_urls: &[String]) -> Result<Self, DbError> {
        let orchestartor = DbOrchestartor::try_new(
            connection_urls,
            RetryConfig {
                min_delay_millis: RETRY_MIN_DELAY_MILLIS,
                factor: RETRY_FACTOR,
                max_times: RETRY_MAX_TIMES,
                max_delay_seconds: RETRY_MAX_DELAY_SECONDS,
            },
        )
        .map_err(|e| DbError::ConnectError(e.to_string()))?;

        Ok(Self { orchestartor })
    }

    pub async fn insert_payment_event(
        &self,
        address: &str,
        started_at: &BigDecimal,
        amount: &BigDecimal,
        valid_until: &BigDecimal,
        tx_hash: &str,
    ) -> Result<(), sqlx::Error> {
        self.orchestartor
            .write(async |pool| {
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
