use db::types::Task;
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct Db {
    pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
pub enum DbError {
    ConnectError(String),
    Query(String),
}

impl Db {
    pub async fn try_new(connection_url: &str) -> Result<Self, DbError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_url)
            .await
            .map_err(|e| DbError::ConnectError(e.to_string()))?;

        Ok(Self { pool })
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
        .fetch_all(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))
    }

    pub async fn insert_tasks_merkle_path_and_mark_them_as_verified(
        &self,
        updates: Vec<(Uuid, Vec<u8>)>,
    ) -> Result<(), DbError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

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
                    .await
                    .map_err(|e| DbError::Query(e.to_string()))?;
                tracing::error!("Error while updating task merkle path and status {}", e);
                return Err(DbError::Query(e.to_string()));
            }
        }

        tx.commit()
            .await
            .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }

    pub async fn mark_tasks_as_pending(&self, tasks_id: &[Uuid]) -> Result<(), DbError> {
        if tasks_id.is_empty() {
            return Ok(());
        }

        sqlx::query(
            "UPDATE tasks SET status = 'pending', status_updated_at = now()
             WHERE task_id = ANY($1) AND status = 'processing'",
        )
        .bind(tasks_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::Query(e.to_string()))?;

        Ok(())
    }
}
