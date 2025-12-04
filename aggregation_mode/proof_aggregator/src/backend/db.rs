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

    pub async fn get_tasks_and_mark_them_as_processed() {}

    pub async fn mark_tasks_as_pending() {}

    pub async fn mark_tasks_as_processing() {}

    pub async fn mark_tasks_as_verified() {}

    pub async fn mark_tasks_as_submitted() {}
}
