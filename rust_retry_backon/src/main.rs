use anyhow::Result;
use backon::ExponentialBuilder;
use backon::Retryable;
use tokio;
use tokio::time::Duration;
use std::time::SystemTime;
use rand::Rng;

/// RetryError will be used to differentiate between recoverable and not recoverable
/// errors.
#[derive(Clone, Debug, Eq, PartialEq)]
enum RetryError {
    RecoverableError,
    NonRecoverableError,
}

impl std::fmt::Display for RetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RetryError::NonRecoverableError => write!(f, "Non recoverable"),
            RetryError::RecoverableError => write!(f, "Recoverable"),
        }
    }
}
impl std::error::Error for RetryError {}

#[tokio::main]
async fn main() -> Result<()> {

    // * ---------------------------------------------------------------------------------------- *
    // *                          DEFINE THE ACTION TO BE RETRIED                                 *
    // * ---------------------------------------------------------------------------------------- *

    // In the case of Aligned, this would be whatever messaging function we would like
    // to be retried in case of failure.
    // This function will return two different type of errors: recoverable and not recoverable.
    // For the case of a recoverable error, the retry mechanism will keep trying until some of the
    // stop conditions is met.
    // For the not recoverable case, it will return without retrying again.
    // This behavior is simulated here with some randomness.
    async fn action() -> anyhow::Result<u64> {
            println!("Doing some operation...");
            println!("Actual time: {:?}", SystemTime::now());

            let mut rng = rand::thread_rng();
            let random_num: f64 = rng.gen(); // generates a float between 0 and 1
            if random_num > 0.5 {
                return anyhow::bail!(RetryError::NonRecoverableError);
            };

            anyhow::bail!(RetryError::RecoverableError)
        }

    // jitter: false
    // factor: 2
    // min_delay: 2s
    // max_delay: 60s
    // max_times: 3
    let backoff = ExponentialBuilder::default().with_min_delay(Duration::from_millis(2000)).with_max_times(3);

    let content = action
        // Retry with exponential backoff
        .retry(backoff)
        // Sleep implementation, required if no feature has been enabled
        .sleep(tokio::time::sleep)
        // When to retry
        .when(|e| e.to_string() == "Recoverable")
        // Notify when retrying
        .notify(|err: &anyhow::Error, dur: Duration| {
            println!("retrying {:?} after {:?}", err, dur);
        })
        .await?;
    println!("action succeeded: {}", content);

    Ok(())
}