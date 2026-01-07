use std::{future::Future, sync::Arc, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::retry::{next_backoff_delay, RetryConfig, RetryError};

/// A single DB node: connection pool plus shared health flags (used to prioritize nodes).
#[derive(Debug)]
struct DbNode {
    pool: Pool<Postgres>,
}

/// Database orchestrator for running reads/writes across multiple PostgreSQL nodes with retry/backoff.
///
/// `DbOrchestrator` holds a list of database nodes (connection pools) and will
/// retry transient failures with exponential backoff based on `retry_config`,
///
/// ## Thread-safe `Clone`
/// This type is cheap and thread-safe to clone:
/// - `nodes` is `Vec<Arc<DbNode>>`, so cloning only increments `Arc` ref-counts and shares the same pools/nodes,
/// - `sqlx::Pool<Postgres>` is internally reference-counted and designed to be cloned and used concurrently,
/// - the node health flags are `AtomicBool`, so updates are safe from multiple threads/tasks.
///
/// Clones share health state (the atomics) and the underlying pools, so all clones observe and influence
/// the same “preferred node” ordering decisions.
#[derive(Debug, Clone)]
pub struct DbOrchestrator {
    nodes: Vec<Arc<DbNode>>,
    retry_config: RetryConfig,
}

#[derive(Debug)]
pub enum DbOrchestratorError {
    InvalidNumberOfConnectionUrls,
    Sqlx(sqlx::Error),
}

impl std::fmt::Display for DbOrchestratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumberOfConnectionUrls => {
                write!(f, "invalid number of connection URLs")
            }
            Self::Sqlx(e) => write!(f, "{e}"),
        }
    }
}

impl DbOrchestrator {
    pub fn try_new(
        connection_urls: &[String],
        retry_config: RetryConfig,
    ) -> Result<Self, DbOrchestratorError> {
        if connection_urls.is_empty() {
            return Err(DbOrchestratorError::InvalidNumberOfConnectionUrls);
        }

        let nodes = connection_urls
            .iter()
            .map(|url| {
                let pool = PgPoolOptions::new().max_connections(5).connect_lazy(url)?;

                Ok(Arc::new(DbNode { pool }))
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(DbOrchestratorError::Sqlx)?;

        Ok(Self {
            nodes,
            retry_config,
        })
    }

    pub async fn query<T, Q, Fut>(&self, query_fn: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let mut attempts = 0;
        let mut delay = Duration::from_millis(self.retry_config.min_delay_millis);

        loop {
            match self.execute_once(&query_fn).await {
                Ok(value) => return Ok(value),
                Err(RetryError::Permanent(err)) => return Err(err),
                Err(RetryError::Transient(err)) => {
                    if attempts >= self.retry_config.max_times {
                        return Err(err);
                    }

                    tracing::warn!(attempt = attempts, delay_millis = delay.as_millis(), error = ?err, "retrying after backoff");
                    tokio::time::sleep(delay).await;
                    delay = next_backoff_delay(delay, self.retry_config.clone());
                    attempts += 1;
                }
            }
        }
    }

    async fn execute_once<T, Q, Fut>(&self, query_fn: &Q) -> Result<T, RetryError<sqlx::Error>>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let mut last_error = None;

        for (idx, node) in self.nodes.iter().enumerate() {
            let pool = node.pool.clone();

            match query_fn(pool).await {
                Ok(res) => {
                    return Ok(res);
                }
                Err(err) => {
                    if Self::is_connection_error(&err) {
                        tracing::warn!(node_index = idx, error = ?err, "database query failed");
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
