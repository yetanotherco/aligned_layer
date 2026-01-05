use std::time::Duration;

#[derive(Debug)]
pub(super) enum RetryError<E> {
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

impl<E: std::fmt::Display> std::error::Error for RetryError<E> where E: std::fmt::Debug {}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// * `min_delay_millis` - Initial delay before first retry attempt (in milliseconds)
    pub min_delay_millis: u64,
    /// * `factor` - Exponential backoff multiplier for retry delays
    pub factor: f32,
    /// * `max_times` - Maximum number of retry attempts
    pub max_times: usize,
    /// * `max_delay_seconds` - Maximum delay between retry attempts (in seconds)
    pub max_delay_seconds: u64,
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
pub fn next_backoff_delay(current_delay: Duration, retry_config: RetryConfig) -> Duration {
    let max: Duration = Duration::from_secs(retry_config.max_delay_seconds);
    // Defensive: factor should be >= 1.0 for backoff, we clamp it to avoid shrinking/NaN.
    let factor = f64::from(retry_config.factor).max(1.0);

    let scaled_secs = current_delay.as_secs_f64() * factor;
    let scaled_secs = if scaled_secs.is_finite() {
        scaled_secs
    } else {
        max.as_secs_f64()
    };

    let scaled = Duration::from_secs_f64(scaled_secs);
    scaled.min(max)
}
