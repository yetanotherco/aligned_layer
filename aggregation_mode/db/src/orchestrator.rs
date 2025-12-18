use std::{future::Future, time::Duration};

use backon::{ExponentialBuilder, Retryable};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::retry::RetryError;

#[derive(Clone, Debug)]
struct DbNode {
    pool: Pool<Postgres>,
}

pub struct DbOrchestartor {
    nodes: Vec<DbNode>,
}

impl DbOrchestartor {
    pub fn try_new(connection_urls: Vec<String>) -> Result<Self, sqlx::Error> {
        // TODO: validate at least one connection url
        let nodes = connection_urls
            .into_iter()
            .map(|url| {
                let pool = PgPoolOptions::new().max_connections(5).connect_lazy(&url)?;

                Ok(DbNode { pool })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()?;

        Ok(Self { nodes })
    }

    fn backoff_builder(&self) -> ExponentialBuilder {
        ExponentialBuilder::default()
            .with_min_delay(Duration::from_millis(0))
            .with_max_times(0)
            .with_factor(0.0)
            .with_max_delay(Duration::from_secs(0))
    }

    pub async fn query<T, E, Q, Fut>(&self, query_fn: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let func = async || {
            let mut last_error = None;

            for idx in 0..self.nodes.len() {
                let pool = self.nodes[idx].pool.clone();
                match query_fn(pool).await {
                    Ok(res) => return Ok(res),
                    Err(err) => {
                        if Self::is_connection_error(&err) {
                            last_error = Some(err);
                        } else {
                            return Err(RetryError::Permanent(err));
                        }
                    }
                };
            }

            Err(RetryError::Transient(
                last_error.expect("write_op attempted without database nodes"),
            ))
        };

        func.retry(self.backoff_builder())
            .sleep(tokio::time::sleep)
            .when(|e| matches!(e, RetryError::Transient(_)))
            .await
            .map_err(|e| e.inner())
    }

    fn is_connection_error(error: &sqlx::Error) -> bool {
        matches!(
            error,
            sqlx::Error::Io(_)
                | sqlx::Error::Tls(_)
                | sqlx::Error::Protocol(_)
                | sqlx::Error::PoolTimedOut
                | sqlx::Error::PoolClosed
                | sqlx::Error::WorkerCrashed
                | sqlx::Error::BeginFailed
        )
    }
}
