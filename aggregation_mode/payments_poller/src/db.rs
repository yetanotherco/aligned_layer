use sqlx::{postgres::PgPoolOptions, types::BigDecimal, Pool, Postgres};

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

    pub async fn insert_payment_event(
        &self,
        address: &str,
        started_at: &BigDecimal,
        amount: &BigDecimal,
        valid_until: &BigDecimal,
        tx_hash: &str,
    ) -> Result<(), sqlx::Error> {
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
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
}
