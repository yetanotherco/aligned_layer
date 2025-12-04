use db::types::Task;
use sqlx::{
    postgres::PgPoolOptions,
    types::{BigDecimal, Uuid},
    Pool, Postgres,
};

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

    pub async fn get_pending_tasks_and_mark_them_as_processed(
        &self,
        limit: i64,
    ) -> Result<Vec<Task>, DbError> {
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE status = 'pending' LIMIT $1")
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DbError::Query(e.to_string()))
    }

    pub async fn mark_tasks_as_pending(&self) {}

    pub async fn mark_tasks_as_processing(&self) {}

    pub async fn mark_tasks_as_verified(&self) {}

    pub async fn mark_tasks_as_submitted(&self) {}
}
