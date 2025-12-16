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

    pub async fn get_pending_tasks_and_mark_them_as_processing(
        &self,
        proving_system_id: i32,
        limit: i64,
    ) -> Result<Vec<Task>, DbError> {
        sqlx::query_as::<_, Task>(
            "WITH selected AS (
                    SELECT task_id
                    FROM tasks
                    WHERE proving_system_id = $1 AND status = 'pending'
                    LIMIT $2
                    FOR UPDATE SKIP LOCKED
                )
                UPDATE tasks t
                SET status = 'processing'
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
                "UPDATE tasks SET merkle_path = $1, status = 'verified', proof = NULL WHERE task_id = $2",
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

    // TODO: this should be used when rolling back processing proofs on unexpected errors
    pub async fn mark_tasks_as_pending(&self) {}
}
