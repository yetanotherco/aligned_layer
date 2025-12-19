use db::{orchestrator::DbOrchestartor, retry::RetryConfig};
use sqlx::types::BigDecimal;

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
                factor: 0.0,
                max_delay_seconds: 0,
                max_times: 0,
                min_delay_millis: 0,
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
