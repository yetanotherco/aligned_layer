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

    pub async fn count_tasks_by_address(&self, address: &str) -> Result<i64, sqlx::Error> {
        let (count,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM tasks WHERE address = $1")
            .bind(address.to_lowercase())
            .fetch_one(&self.pool)
            .await?;

        Ok(count)
    }

    pub async fn get_merkle_path_by_task_id(
        &self,
        task_id: Uuid,
    ) -> Result<Option<Vec<u8>>, sqlx::Error> {
        sqlx::query_scalar::<_, Option<Vec<u8>>>("SELECT merkle_path FROM tasks WHERE task_id = $1")
            .bind(task_id)
            .fetch_optional(&self.pool)
            .await
            .map(|res| res.flatten())
    }

    pub async fn insert_task(
        &self,
        address: &str,
        proving_system_id: i32,
        proof: &[u8],
        program_commitment: &[u8],
        merkle_path: Option<&[u8]>,
    ) -> Result<Uuid, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO tasks (
                address,
                proving_system_id,
                proof,
                program_commitment,
                merkle_path
            ) VALUES ($1, $2, $3, $4, $5)
            RETURNING task_id",
        )
        .bind(address.to_lowercase())
        .bind(proving_system_id)
        .bind(proof)
        .bind(program_commitment)
        .bind(merkle_path)
        .fetch_one(&self.pool)
        .await
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

    pub async fn has_active_payment_event(
        &self,
        address: &str,
        epoch: BigDecimal,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT 1 FROM payment_events
                WHERE address = $1 AND started_at < $2 AND $2 < valid_until
            )",
        )
        .bind(address.to_lowercase())
        .bind(epoch)
        .fetch_one(&self.pool)
        .await
    }
}
