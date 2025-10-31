pub mod batcher_retryables;

use backon::ExponentialBuilder;
use backon::Retryable;
use std::{future::Future, time::Duration};

#[derive(Debug)]
pub enum RetryError<E> {
    Transient(E),
    Permanent(E),
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RetryError::Transient(e) => write!(f, "{}", e),
            RetryError::Permanent(e) => write!(f, "{}", e),
        }
    }
}

impl<E> RetryError<E> {
    pub fn inner(self) -> E {
        match self {
            RetryError::Transient(e) => e,
            RetryError::Permanent(e) => e,
        }
    }
}

impl<E: std::fmt::Display> std::error::Error for RetryError<E> where E: std::fmt::Debug {}

/// Supports retries only on async functions. See: https://docs.rs/backon/latest/backon/#retry-an-async-function
/// Runs with `jitter: false`.
pub async fn retry_function<FutureFn, Fut, T, E>(
    function: FutureFn,
    min_delay: u64,
    factor: f32,
    max_times: usize,
    max_delay: u64,
) -> Result<T, RetryError<E>>
where
    Fut: Future<Output = Result<T, RetryError<E>>>,
    FutureFn: FnMut() -> Fut,
{
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(min_delay))
        .with_max_times(max_times)
        .with_factor(factor)
        .with_max_delay(Duration::from_secs(max_delay));

    function
        .retry(backoff)
        .sleep(tokio::time::sleep)
        .when(|e| matches!(e, RetryError::Transient(_)))
        .await
}
