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
    use aligned_sdk::eth::batcher_payment_service::BatcherPaymentService;
    use ethers::{
        contract::abigen,
        providers::{Http, Middleware, Provider},
        types::{Address, U256},
        utils::Anvil,
    };
    use std::str::FromStr;
    use std::{sync::Arc, time::SystemTime};

    abigen!(
        BatcherPaymentServiceContract,
        "../aligned-sdk/abi/BatcherPaymentService.json"
    );

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

    #[tokio::test]
    async fn test_anvil() {
        let _anvil = Anvil::new()
            .port(8545u16)
            .arg("--load-state")
            .arg("../../contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json")
            .spawn();

        let eth_rpc_provider: Provider<Http> =
            eth::get_provider(String::from("http://localhost:8545"))
                .expect("Failed to get provider");

        let payment_service_addr =
            Address::from_str("0x7969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap();

        let payment_service =
            BatcherPaymentService::new(payment_service_addr, Arc::new(eth_rpc_provider));

        let dummy_user_addr =
            Address::from_str("0x8969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap();

        if let Ok(balance) = payment_service.user_balances(dummy_user_addr).call().await {
            println!("ALIGNED USER BALANCE: {:?}", balance)
        };
    }
}
