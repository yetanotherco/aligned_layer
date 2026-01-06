use db::{orchestrator::DbOrchestrator, retry::RetryConfig, types::Task};
use sqlx::types::Uuid;

// Retry/backoff behavior summary (see
// aggregation_mode/db/src/orchestrator.rs:next_back_off_delay for implementation)
//
// 1) Max wait time between failures if all retries fail:
//    The sleep between retries is capped at 30 seconds (RETRY_MAX_DELAY_SECONDS).
//
// 2) Wait before each retry attempt with the current config
//    (start = 500ms, factor = 5.0, max retries = 10):
//
//    retry 1: 0.5s
//    retry 2: 2.5s
//    retry 3: 12.5s
//    retry 4: 30s (capped)
//    retry 5–10: 30s each
//
//    Worst-case total sleep time across all retries: ~3m 48s,
//    plus the execution time of each DB attempt.
/// Initial delay before first retry attempt (in milliseconds)
const RETRY_MIN_DELAY_MILLIS: u64 = 500;
/// Exponential backoff multiplier for retry delays
const RETRY_FACTOR: f32 = 5.0;
/// Maximum number of retry attempts
const RETRY_MAX_TIMES: usize = 10;
/// Maximum delay between retry attempts (in seconds)
const RETRY_MAX_DELAY_SECONDS: u64 = 30;

#[derive(Debug, Clone)]
pub struct Db {
    orchestrator: DbOrchestrator,
}

#[derive(Debug, Clone)]
pub enum DbError {
    Creation(String),
    Query(String),
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
        .map_err(|e| DbError::Creation(e.to_string()))?;

        Ok(Self { orchestrator })
    }

    /// Fetches tasks that are ready to be processed and atomically updates their status.
    ///
    /// This function selects up to `limit` tasks for the given `proving_system_id` that are
    /// either:
    /// - in `pending` status, or
    /// - in `processing` status but whose `status_updated_at` timestamp is older than 12 hours
    ///   (to recover tasks that may have been abandoned or stalled).
    ///
    /// The selected rows are locked using `FOR UPDATE SKIP LOCKED` to ensure safe concurrent
    /// processing by multiple workers. All selected tasks have their status set to
    /// `processing` and their `status_updated_at` updated to `now()` before being returned.
    pub async fn get_tasks_to_process_and_update_their_status(
        &self,
        proving_system_id: i32,
        limit: i64,
    ) -> Result<Vec<Task>, DbError> {
        self.orchestrator
            .query(async |pool| {
                sqlx::query_as::<_, Task>(
                    "WITH selected AS (
                    SELECT task_id
                    FROM tasks
                    WHERE proving_system_id = $1
                      AND (
                        status = 'pending'
                        OR (
                            status = 'processing'
                            AND status_updated_at <= now() - interval '12 hours'
                        )
                      )
                    LIMIT $2
                    FOR UPDATE SKIP LOCKED
                )
                UPDATE tasks t
                SET status = 'processing', status_updated_at = now()
                FROM selected s
                WHERE t.task_id = s.task_id
                RETURNING t.*;",
                )
                .bind(proving_system_id)
                .bind(limit)
                .fetch_all(&pool)
                .await
            })
            .await
            .map_err(|e| DbError::Query(e.to_string()))
    }

    pub async fn insert_tasks_merkle_path_and_mark_them_as_verified(
        &mut self,
        updates: Vec<(Uuid, Vec<u8>)>,
    ) -> Result<(), DbError> {
        let updates_ref = &updates;

        self.orchestrator
            .query(async |pool| {
                let updates = updates_ref;
                let mut tx = pool.begin().await?;

                for (task_id, merkle_path) in updates {
                    if let Err(e) = sqlx::query(
                        "UPDATE tasks SET merkle_path = $1, status = 'verified', status_updated_at = now(), proof = NULL WHERE task_id = $2",
                    )
                    .bind(merkle_path)
                    .bind(task_id)
                    .execute(&mut *tx)
                    .await
                    {
                        tx.rollback()
                            .await?;
                        tracing::error!("Error while updating task merkle path and status {}", e);
                        return Err(e);
                    }
                }

                tx.commit().await
            })
            .await
            .map_err(|e| DbError::Query(e.to_string()))
    }

    pub async fn mark_tasks_as_pending(&self, tasks_id: &[Uuid]) -> Result<(), DbError> {
        if tasks_id.is_empty() {
            return Ok(());
        }

        self.orchestrator
            .query(async |pool| {
                sqlx::query(
                    "UPDATE tasks SET status = 'pending', status_updated_at = now()
                 WHERE task_id = ANY($1) AND status = 'processing'",
                )
                .bind(tasks_id)
                .execute(&pool)
                .await
            })
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }
}
