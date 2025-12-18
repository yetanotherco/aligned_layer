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
