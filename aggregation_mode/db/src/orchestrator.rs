use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::retry::{RetryConfig, RetryError};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Read,
    Write,
}

/// A single DB node: connection pool plus shared health flags (used to prioritize nodes).
#[derive(Debug)]
struct DbNode {
    pool: Pool<Postgres>,
    last_read_failed: AtomicBool,
    last_write_failed: AtomicBool,
}

/// Database orchestrator for running reads/writes across multiple PostgreSQL nodes with retry/backoff.
///
/// `DbOrchestrator` holds a list of database nodes (connection pools) and will:
/// - try nodes in a preferred order (healthy nodes first, then recently-failed nodes),
/// - mark nodes as failed on connection-type errors,
/// - retry transient failures with exponential backoff based on `retry_config`,
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

                Ok(Arc::new(DbNode {
                    pool,
                    last_read_failed: AtomicBool::new(false),
                    last_write_failed: AtomicBool::new(false),
                }))
            })
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(DbOrchestratorError::Sqlx)?;

        Ok(Self {
            nodes,
            retry_config,
        })
    }

    pub async fn write<T, Q, Fut>(&self, query: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        self.query::<T, Q, Fut>(query, Operation::Write).await
    }

    pub async fn read<T, Q, Fut>(&self, query: Q) -> Result<T, sqlx::Error>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        self.query::<T, Q, Fut>(query, Operation::Read).await
    }

    async fn query<T, Q, Fut>(&self, query_fn: Q, operation: Operation) -> Result<T, sqlx::Error>
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

                    tracing::warn!(attempt = attempts, delay_millis = delay.as_millis(), error = ?err, "retrying after backoff");
                    tokio::time::sleep(delay).await;
                    delay = self.next_backoff_delay(delay);
                    attempts += 1;
                }
            }
        }
    }

    // Exponential backoff with a hard cap.
    //
    // Each retry multiplies the previous delay by `retry_config.factor`,
    // then clamps it to `max_delay_seconds`. This yields:
    //
    //   d_{n+1} = min(max, d_n * factor) => d_n = min(max, d_initial * factor^n)
    //
    // Example starting at 500ms with factor = 2.0 (no jitter):
    //   retry 0: 0.5s
    //   retry 1: 1.0s
    //   retry 2: 2.0s
    //   retry 3: 4.0s
    //   retry 4: 8.0s
    //   ...
    // until the delay reaches `max_delay_seconds`, after which it stays at that max.
    // see reference: https://en.wikipedia.org/wiki/Exponential_backoff
    // and here: https://docs.aws.amazon.com/prescriptive-guidance/latest/cloud-design-patterns/retry-backoff.html
    fn next_backoff_delay(&self, current_delay: Duration) -> Duration {
        let max: Duration = Duration::from_secs(self.retry_config.max_delay_seconds);
        // Defensive: factor should be >= 1.0 for backoff, we clamp it to avoid shrinking/NaN.
        let factor = f64::from(self.retry_config.factor).max(1.0);

        let scaled_secs = current_delay.as_secs_f64() * factor;
        let scaled_secs = if scaled_secs.is_finite() {
            scaled_secs
        } else {
            max.as_secs_f64()
        };

        let scaled = Duration::from_secs_f64(scaled_secs);
        if scaled > max {
            max
        } else {
            scaled
        }
    }

    async fn execute_once<T, Q, Fut>(
        &self,
        query_fn: &Q,
        operation: Operation,
    ) -> Result<T, RetryError<sqlx::Error>>
    where
        Q: Fn(Pool<Postgres>) -> Fut,
        Fut: Future<Output = Result<T, sqlx::Error>>,
    {
        let mut last_error = None;

        for idx in self.preferred_order(operation) {
            let node = &self.nodes[idx];
            let pool = node.pool.clone();

            match query_fn(pool).await {
                Ok(res) => {
                    match operation {
                        Operation::Read => node.last_read_failed.store(false, Ordering::Relaxed),
                        Operation::Write => node.last_write_failed.store(false, Ordering::Relaxed),
                    };
                    return Ok(res);
                }
                Err(err) => {
                    if Self::is_connection_error(&err) {
                        tracing::warn!(node_index = idx, error = ?err, "database query failed");
                        match operation {
                            Operation::Read => node.last_read_failed.store(true, Ordering::Relaxed),
                            Operation::Write => {
                                node.last_write_failed.store(true, Ordering::Relaxed)
                            }
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
                Operation::Read => node.last_read_failed.load(Ordering::Relaxed),
                Operation::Write => node.last_write_failed.load(Ordering::Relaxed),
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
