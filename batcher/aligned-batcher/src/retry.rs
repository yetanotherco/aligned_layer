use backon::ExponentialBuilder;
use backon::Retryable;
use std::{future::Future, time::Duration};

pub const DEFAULT_MIN_DELAY: u64 = 2000;
pub const DEFAULT_MAX_TIMES: usize = 3;
pub const DEFAULT_FACTOR: f32 = 2.0;

#[derive(Debug)]
pub enum RetryError<E> {
    Transient,
    Permanent(E),
}

impl<E> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Retry error!")
    }
}
impl<E> std::error::Error for RetryError<E> where E: std::fmt::Debug {}

pub async fn retry_function<FutureFn, Fut, T, E>(
    function: FutureFn,
    min_delay: u64,
    factor: f32,
    max_times: usize,
) -> Result<T, RetryError<E>>
where
    Fut: Future<Output = Result<T, RetryError<E>>>,
    FutureFn: FnMut() -> Fut,
{
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(min_delay))
        .with_max_times(max_times)
        .with_factor(factor);

    function
        .retry(backoff)
        .sleep(tokio::time::sleep)
        .when(|e| matches!(e, RetryError::Transient))
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::eth;
    use ethers::{providers::Middleware, types::U256};
    use std::time::SystemTime;

    #[tokio::test]
    async fn retry_test() {
        async fn dummy_action(x: u64) -> Result<u64, RetryError<()>> {
            println!("Doing some operation...");
            println!("Actual time: {:?}", SystemTime::now());
            println!("X: {x}");

            Err(RetryError::Permanent(()))
        }

        assert!(retry_function(|| dummy_action(10), 2000, 2.0, 3)
            .await
            .is_err());
    }

    #[tokio::test]
    async fn retry_test_eth() {
        async fn get_gas_price() -> Result<U256, RetryError<()>> {
            let eth_rpc_provider =
                eth::get_provider(String::from("https://ethereum-holesky-rpc.publicnode.com"))
                    .expect("Failed to get provider");

            match eth_rpc_provider.get_gas_price().await {
                Ok(val) => {
                    println!("GAS PRICE IS: {:?}", val);
                    Ok(val)
                }
                Err(_) => Err(RetryError::Transient),
            }
        }

        assert!(retry_function(get_gas_price, 2000, 2.0, 3).await.is_ok());
    }
}
