use std::future::Future;
use std::time::Duration;

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

                tracing::warn!("Retryable function failed: {e}");

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
    if scaled > max {
        max
    } else {
        scaled
    }
}
