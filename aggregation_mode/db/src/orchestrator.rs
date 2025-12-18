use std::{future::Future, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::retry::{RetryConfig, RetryError};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Read,
    Write,
}

#[derive(Debug)]
struct DbNode {
    pool: Pool<Postgres>,
    last_read_failed: bool,
    last_write_failed: bool,
}

#[derive(Debug)]
pub struct DbOrchestartor {
    nodes: Vec<DbNode>,
    retry_config: RetryConfig,
}

pub enum DbOrchestartorError {
    InvalidNumberOfConnectionUrls,
    Sqlx(sqlx::Error),
}

impl DbOrchestartor {
    pub fn try_new(
        connection_urls: Vec<String>,
        retry_config: RetryConfig,
    ) -> Result<Self, DbOrchestartorError> {
        if connection_urls.is_empty() {
            return Err(DbOrchestartorError::InvalidNumberOfConnectionUrls);
        }

        let nodes = connection_urls
            .into_iter()
            .map(|url| {
                let pool = PgPoolOptions::new().max_connections(5).connect_lazy(&url)?;

                Ok(DbNode {
                    pool,
                    last_read_failed: false,
                    last_write_failed: false,
                })
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(|e| DbOrchestartorError::Sqlx(e))?;

        Ok(Self {
            nodes,
            retry_config,
        })
    }

    pub async fn write<T, Q, Fut>(&mut self, query: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        self.query::<T, Q, Fut>(query, Operation::Write).await
    }

    pub async fn read<T, Q, Fut>(&mut self, query: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        self.query::<T, Q, Fut>(query, Operation::Read).await
    }

    async fn query<T, Q, Fut>(
        &mut self,
        query_fn: Q,
        operation: Operation,
    ) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let mut attempts = 0;
        let mut delay = Duration::from_millis(self.retry_config.min_delay_millis);

        loop {
            match self.execute_once(&query_fn, operation).await {
                Ok(value) => return Ok(value),
                Err(RetryError::Permanent(err)) => return Err(err),
                Err(RetryError::Transient(err)) => {
                    if attempts >= self.retry_config.max_delay_seconds {
                        return Err(err);
                    }

                    tracing::warn!(attempt = attempts, delay_milis = delay.as_millis(), error = ?err, "retrying after backoff");
                    tokio::time::sleep(delay).await;
                    delay = self.backoff_delay(delay);
                    attempts += 1;
                }
            }
        }
    }

    fn backoff_delay(&self, current: Duration) -> Duration {
        let max = Duration::from_secs(self.retry_config.max_delay_seconds);
        let scaled_secs = current.as_secs_f64() * f64::from(self.retry_config.factor);
        let scaled = Duration::from_secs_f64(scaled_secs);
        if scaled > max {
            max
        } else {
            scaled
        }
    }

    async fn execute_once<T, Q, Fut>(
        &mut self,
        query_fn: &Q,
        operation: Operation,
    ) -> Result<T, RetryError<sqlx::Error>>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let mut last_error = None;

        for idx in self.preferred_order(operation) {
            let pool = self.nodes[idx].pool.clone();

            match query_fn(pool).await {
                Ok(res) => {
                    match operation {
                        Operation::Read => self.nodes[idx].last_read_failed = false,
                        Operation::Write => self.nodes[idx].last_write_failed = false,
                    };
                    return Ok(res);
                }
                Err(err) => {
                    if Self::is_connection_error(&err) {
                        tracing::warn!(node_index = idx, error = ?err, "database query failed");
                        match operation {
                            Operation::Read => self.nodes[idx].last_read_failed = true,
                            Operation::Write => self.nodes[idx].last_write_failed = true,
                        };
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
    }

    fn preferred_order(&self, operation: Operation) -> Vec<usize> {
        let mut preferred = Vec::with_capacity(self.nodes.len());
        let mut fallback = Vec::new();

        for (idx, node) in self.nodes.iter().enumerate() {
            let failed = match operation {
                Operation::Read => node.last_read_failed,
                Operation::Write => node.last_write_failed,
            };

            if failed {
                fallback.push(idx);
            } else {
                preferred.push(idx);
            }
        }

        preferred.extend(fallback);
        preferred
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
                | sqlx::Error::Database(_)
        )
    }
}
