use crate::types::Receipt;
use db::{orchestrator::DbOrchestrator, retry::RetryConfig};
use sqlx::types::{BigDecimal, Uuid};

// Retry/backoff behavior summary (see
// aggregation_mode/db/src/orchestrator.rs:next_back_off_delay for implementation)
//
// NOTE: These retry limits are intentionally lower than in other crates.
// This code runs in an HTTP server; in the worst case the request fails fast
// and the client can retry the request. Prolonged blocking retries here are
// less critical than in background or batch processing jobs.
//
// 1) Max wait time between failures if all retries fail:
//    The sleep between retries is capped at 10 seconds (RETRY_MAX_DELAY_SECONDS).
//
// 2) Wait before each retry attempt with the current config
//    (start = 500ms, factor = 2.0, max retries = 4):
//
//    retry 1: 0.5s
//    retry 2: 1.0s
//    retry 3: 2.0s
//    retry 4: 4.0s
//
//    Worst-case total sleep time across all retries: 7.5 seconds,
//    plus the execution time of each DB attempt.
/// Initial delay before first retry attempt (in milliseconds)
const RETRY_MIN_DELAY_MILLIS: u64 = 500;
/// Exponential backoff multiplier for retry delays
const RETRY_FACTOR: f32 = 2.0;
/// Maximum number of retry attempts
const RETRY_MAX_TIMES: usize = 4;
/// Maximum delay between retry attempts (in seconds)
const RETRY_MAX_DELAY_SECONDS: u64 = 10;

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

    pub async fn count_tasks_by_address(&self, address: &str) -> Result<i64, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                let (count,) =
                    sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE address = $1")
                        .bind(address.to_lowercase())
                        .fetch_one(&pool)
                        .await?;

                Ok(count)
            })
            .await
    }

    pub async fn get_merkle_path_by_task_id(
        &self,
        task_id: Uuid,
    ) -> Result<Option<Vec<u8>>, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_scalar::<_, Option<Vec<u8>>>(
                    "SELECT merkle_path FROM tasks WHERE task_id = $1",
                )
                .bind(task_id)
                .fetch_optional(&pool)
                .await
                .map(|res| res.flatten())
            })
            .await
    }

    pub async fn get_tasks_by_address_and_nonce(
        &self,
        address: &str,
        nonce: i64,
    ) -> Result<Vec<Receipt>, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_as::<_, Receipt>(
                    "SELECT status,merkle_path,nonce,address FROM tasks
                    WHERE address = $1
                    AND nonce = $2
                    ORDER BY nonce DESC",
                )
                .bind(address.to_lowercase())
                .bind(nonce)
                .fetch_all(&pool)
                .await
            })
            .await
    }

    pub async fn get_tasks_by_address_with_limit(
        &self,
        address: &str,
        limit: i64,
    ) -> Result<Vec<Receipt>, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_as::<_, Receipt>(
                    "SELECT status,merkle_path,nonce,address FROM tasks
                    WHERE address = $1
                    ORDER BY nonce DESC
                    LIMIT $2",
                )
                .bind(address.to_lowercase())
                .bind(limit)
                .fetch_all(&pool)
                .await
            })
            .await
    }

    pub async fn get_daily_tasks_by_address(&self, address: &str) -> Result<i64, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*)
                FROM tasks
                WHERE address = $1
                AND inserted_at::date = CURRENT_DATE",
                )
                .bind(address.to_lowercase())
                .fetch_one(&pool)
                .await
            })
            .await
    }

    pub async fn insert_task(
        &self,
        address: &str,
        proving_system_id: i32,
        proof: &[u8],
        program_commitment: &[u8],
        merkle_path: Option<&[u8]>,
        nonce: i64,
    ) -> Result<Uuid, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_scalar::<_, Uuid>(
                    "INSERT INTO tasks (
                        address,
                        proving_system_id,
                        proof,
                        program_commitment,
                        merkle_path,
                        nonce
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING task_id",
                )
                .bind(address.to_lowercase())
                .bind(proving_system_id)
                .bind(proof)
                .bind(program_commitment)
                .bind(merkle_path)
                .bind(nonce)
                .fetch_one(&pool)
                .await
            })
            .await
    }

    pub async fn has_active_payment_event(
        &self,
        address: &str,
        epoch: BigDecimal,
    ) -> Result<bool, sqlx::Error> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS (
                    SELECT 1 FROM payment_events
                    WHERE address = $1 AND started_at < $2 AND $2 < valid_until
                )",
                )
                .bind(address.to_lowercase())
                .bind(&epoch)
                .fetch_one(&pool)
                .await
            })
            .await
    }
}
