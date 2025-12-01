use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

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

    pub async fn count_proofs_by_address(&self, address: &str) -> Result<i64, sqlx::Error> {
        let (count,) =
            sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM proofs WHERE address = $1")
                .bind(address)
                .fetch_one(&self.pool)
                .await?;

        Ok(count)
    }

    pub async fn get_merkle_path_by_proof_id(
        &self,
        proof_id: &str,
    ) -> Result<Option<Vec<u8>>, sqlx::Error> {
        sqlx::query_scalar::<_, Option<Vec<u8>>>(
            "SELECT merkle_path FROM proofs WHERE proof_id = $1",
        )
        .bind(proof_id)
        .fetch_optional(&self.pool)
        .await
        .map(|res| res.flatten())
    }
}
