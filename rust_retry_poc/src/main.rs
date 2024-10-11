use rand::Rng;
use std::time::SystemTime;
use tokio;
use tokio_retry::strategy::ExponentialBackoff;
use tokio_retry::Condition;
use tokio_retry::RetryIf;

/// RetryError will be used to differentiate between recoverable and not recoverable
/// errors.
#[derive(Debug)]
enum RetryError {
    RecoverableError,
    NonRecoverableError,
}

impl std::fmt::Display for RetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Retry error!")
    }
}
impl std::error::Error for RetryError {}

// A condition is defined over `RetryError` so that we can distinguish between the
// recoverable and the not recoverable case in order to define to continue retrying or not
struct MyCondition;
impl Condition<RetryError> for MyCondition {
    fn should_retry(&mut self, error: &RetryError) -> bool {
        match error {
            RetryError::NonRecoverableError => false,
            RetryError::RecoverableError => true,
        }
    }
}

#[tokio::main]
async fn main() {
    // * ---------------------------------------------------------------------------------------- *
    // *                          DEFINE THE ACTION TO BE RETRIED                                 *
    // * ---------------------------------------------------------------------------------------- *

    // In the case of Aligned, this would be whatever messaging function we would like
    // to be retried in case of failure.
    // This function will return two different type of errors: recoverable and not recoverable.
    // For the case of a recoverable error, the `RetryIf` function will keep trying until some of the
    // stop conditions is met.
    // For the not recoverable case, the `RetryIf` function will return without retrying again.
    // This behavior is simulated here with some randomness.

    async fn action() -> Result<u64, RetryError> {
        println!("Doing some operation...");
        println!("Actual time: {:?}", SystemTime::now());

        let mut rng = rand::thread_rng();
        let random_num: f64 = rng.gen(); // generates a float between 0 and 1
        if random_num > 0.5 {
            return Err(RetryError::NonRecoverableError);
        }

        Err(RetryError::RecoverableError)
    }

    // * ---------------------------------------------------------------------------------------- *
    // *                         EXPONENTIAL BACKOFF CONFIGURATION                                *
    // * ---------------------------------------------------------------------------------------- *
    // For the exponential backoff formula `backoff(n) = a * b^n`, we set the following config:
    // *    a = 1000
    // * 	b = 2ms
    // *    1 <= n <= 3
    // There is no randomization factor (jitter)

    let my_condition = MyCondition {};
    let retry_strategy = ExponentialBackoff::from_millis(2).factor(1000).take(3);

    // * ---------------------------------------------------------------------------------------- *
    // *                               RETRY FUNCTION CALL                                        *
    // * ---------------------------------------------------------------------------------------- *

    let result = RetryIf::spawn(retry_strategy, action, my_condition).await;

    println!("RESULT: {:?}", result);
}
