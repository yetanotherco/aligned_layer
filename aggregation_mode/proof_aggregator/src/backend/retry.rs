use std::future::Future;
use std::time::Duration;

/// Retry/backoff behavior summary:
///
/// This module retries transient failures using exponential backoff, but permanent failures stop immediately.
///
/// Backoff algorithm:
/// - Starts with `delay = ETHEREUM_CALL_MIN_RETRY_DELAY`
/// - After each transient failure, sleeps for `delay` and then updates it following the next formula:
///   delay = min(delay * ETHEREUM_CALL_BACKOFF_FACTOR, ETHEREUM_CALL_MAX_RETRY_DELAY)
/// - Stops retrying when the number of attempts exceed the `ETHEREUM_CALL_MAX_RETRIES` constant
///
/// About the retries limit: In the current implementation `attempt` starts at 0 and we stop when
///   `attempt >= max_times`, incrementing `attempt` after sleeping. That means the code can perform
/// max_times + 1 sleeps/retries. With the current constant value (10), that is 11 backoff intervals.
///
/// Delay schedule with current config
/// (start = 500ms, factor = 2.0, max delay = 60s, max_times = 10):
///
///   retry  1: 0.5s
///   retry  2: 1.0s
///   retry  3: 2.0s
///   retry  4: 4.0s
///   retry  5: 8.0s
///   retry  6: 16.0s
///   retry  7: 32.0s
///   retry  8: 60.0s  (capped)
///   retry  9: 60.0s
///   retry 10: 60.0s
///   retry 11: 60.0s  (due to the max_times + 1 behavior described above)
///
/// Worst-case total sleep time across all retries:
///   0.5 + 1 + 2 + 4 + 8 + 16 + 32 + 60 + 60 + 60 + 60
/// = 303.5 seconds (~5m 3.5s),
/// plus the execution time of each Ethereum call attempt.
///
/// Minimum delay value (the one on first iteration)
pub const ETHEREUM_CALL_MIN_RETRY_DELAY: u64 = 500; // milliseconds

/// Maximum number of retry attempts.
///
/// Note: With the current retry loop logic this behaves as "max_times + 1"
/// backoff intervals.
pub const ETHEREUM_CALL_MAX_RETRIES: usize = 10;

/// Exponential backoff multiplier applied to the delay after each transient failure.
///
/// Note: This value should be at least 1.0, otherwise will be clamped so the backoff never shrinks.
pub const ETHEREUM_CALL_BACKOFF_FACTOR: f32 = 2.0;

/// Maximum delay between retries (seconds). Delays are capped to this value.
pub const ETHEREUM_CALL_MAX_RETRY_DELAY: u64 = 60; // seconds

#[derive(Debug)]
pub enum RetryError<E> {
    Transient(E),
    Permanent(E),
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RetryError::Transient(e) => write!(f, "{e}"),
            RetryError::Permanent(e) => write!(f, "{e}"),
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

pub async fn retry_function<FutureFn, Fut, T, E>(
    mut function: FutureFn,
    min_delay_ms: u64,
    factor: f32,
    max_times: usize,
    max_delay_seconds: u64,
) -> Result<T, RetryError<E>>
where
    Fut: Future<Output = Result<T, RetryError<E>>>,
    FutureFn: FnMut() -> Fut,
{
    let mut delay = Duration::from_millis(min_delay_ms);

    // Defensive: ensure that factor is above 1.0 so backoff never shrinks or becomes invalid.
    let factor = (factor as f64).max(1.0);

    let mut attempt: usize = 0;

    loop {
        match function().await {
            Ok(v) => return Ok(v),
            Err(RetryError::Permanent(e)) => return Err(RetryError::Permanent(e)),
            Err(RetryError::Transient(e)) => {
                if attempt >= max_times {
                    return Err(RetryError::Transient(e));
                }

                tracing::warn!(
                    "Retryable function failed, retrying in {} seconds",
                    delay.as_secs()
                );

                tokio::time::sleep(delay).await;

                delay = next_backoff_delay(delay, max_delay_seconds, factor);

                attempt += 1;
            }
        }
    }
}

/// TODO: Replace with the one in aggregation_mode/db/src/orchestrator.rs, or use a common method.
fn next_backoff_delay(current_delay: Duration, max_delay_seconds: u64, factor: f64) -> Duration {
    let max: Duration = Duration::from_secs(max_delay_seconds);

    let scaled_secs = current_delay.as_secs_f64() * factor;
    let scaled_secs = if scaled_secs.is_finite() {
        scaled_secs
    } else {
        max.as_secs_f64()
    };

    let scaled = Duration::from_secs_f64(scaled_secs);
    scaled.min(max)
}
