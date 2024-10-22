use backon::ExponentialBuilder;
use backon::Retryable;
use std::{future::Future, time::Duration};

pub const DEFAULT_MIN_DELAY: u64 = 2000;
pub const DEFAULT_MAX_TIMES: usize = 3;
pub const DEFAULT_FACTOR: f32 = 2.0;

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
        .when(|e| matches!(e, RetryError::Transient(_)))
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        config::ECDSAConfig,
        connection,
        eth::{
            self, get_provider,
            payment_service::{
                get_user_balance_retryable, get_user_nonce_from_ethereum_retryable,
                user_balance_is_unlocked_retryable, BatcherPaymentService,
            },
            utils::get_gas_price_retryable,
        },
    };
    use ethers::{
        contract::abigen,
        types::{Address, U256},
        utils::{Anvil, AnvilInstance},
    };
    use futures_util::StreamExt;
    use std::{str::FromStr, sync::Arc};
    use tokio::{
        net::{TcpListener, TcpStream},
        sync::RwLock,
    };

    abigen!(
        BatcherPaymentServiceContract,
        "../aligned-sdk/abi/BatcherPaymentService.json"
    );

    async fn setup_anvil(port: u16) -> (AnvilInstance, BatcherPaymentService) {
        let anvil = Anvil::new()
            .port(port)
            .arg("--load-state")
            .arg("../../contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json")
            .spawn();

        let eth_rpc_provider = eth::get_provider(format!("http://localhost:{}", port))
            .expect("Failed to get provider");

        let payment_service_addr = String::from("0x7969c5eD335650692Bc04293B07F5BF2e7A673C0");

        let payment_service = eth::payment_service::get_batcher_payment_service(
            eth_rpc_provider,
            ECDSAConfig {
                private_key_store_path: "../../config-files/anvil.batcher.ecdsa.key.json"
                    .to_string(),
                private_key_store_password: "".to_string(),
            },
            payment_service_addr,
        )
        .await
        .expect("Failed to get Batcher Payment Service contract");
        (anvil, payment_service)
    }

    #[tokio::test]
    async fn test_get_user_balance_retryable() {
        let payment_service;
        let dummy_user_addr =
            Address::from_str("0x8969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap();
        {
            let _anvil;
            (_anvil, payment_service) = setup_anvil(8545u16).await;

            let balance =
                get_user_balance_retryable(&payment_service, &payment_service, &dummy_user_addr)
                    .await
                    .unwrap();

            assert_eq!(balance, U256::zero());
            // Kill anvil
        }

        let result =
            get_user_balance_retryable(&payment_service, &payment_service, &dummy_user_addr).await;
        assert!(matches!(result, Err(RetryError::Transient(_))));

        // restart anvil
        let (_anvil, _) = setup_anvil(8545u16).await;
        let balance =
            get_user_balance_retryable(&payment_service, &payment_service, &dummy_user_addr)
                .await
                .unwrap();

        assert_eq!(balance, U256::zero());
    }

    #[tokio::test]
    async fn test_user_balance_is_unlocked_retryable_kill_anvil() {
        let payment_service;
        let dummy_user_addr =
            Address::from_str("0x8969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap();

        {
            let _anvil;
            (_anvil, payment_service) = setup_anvil(8546u16).await;
            let unlocked = user_balance_is_unlocked_retryable(
                &payment_service,
                &payment_service,
                &dummy_user_addr,
            )
            .await
            .unwrap();

            assert_eq!(unlocked, false);
            // Kill Anvil
        }

        let result = user_balance_is_unlocked_retryable(
            &payment_service,
            &payment_service,
            &dummy_user_addr,
        )
        .await;
        assert!(matches!(result, Err(RetryError::Transient(_))));

        // restart Anvil
        let (_anvil, payment_service) = setup_anvil(8546u16).await;
        let unlocked = user_balance_is_unlocked_retryable(
            &payment_service,
            &payment_service,
            &dummy_user_addr,
        )
        .await
        .unwrap();

        assert_eq!(unlocked, false);
    }

    #[tokio::test]
    async fn test_get_user_nonce_retryable_kill_anvil() {
        let payment_service;
        let dummy_user_addr =
            Address::from_str("0x8969c5eD335650692Bc04293B07F5BF2e7A673C0").unwrap();
        {
            let _anvil;
            (_anvil, payment_service) = setup_anvil(8547u16).await;
            let nonce = get_user_nonce_from_ethereum_retryable(
                &payment_service,
                &payment_service,
                dummy_user_addr,
            )
            .await
            .unwrap();

            assert_eq!(nonce, U256::zero());
            // Kill Anvil
        }

        let result = get_user_nonce_from_ethereum_retryable(
            &payment_service,
            &payment_service,
            dummy_user_addr,
        )
        .await;
        assert!(matches!(result, Err(RetryError::Transient(_))));

        // restart Anvil
        let (_anvil, payment_service) = setup_anvil(8547u16).await;

        let nonce = get_user_nonce_from_ethereum_retryable(
            &payment_service,
            &payment_service,
            dummy_user_addr,
        )
        .await
        .unwrap();

        assert_eq!(nonce, U256::zero());
    }

    #[tokio::test]
    async fn test_get_gas_price_retryable_kill_anvil() {
        let eth_rpc_provider;
        {
            let (_anvil, _payment_service) = setup_anvil(8548u16).await;
            eth_rpc_provider = get_provider("http://localhost:8548".to_string())
                .expect("Failed to get ethereum websocket provider");
            let result = get_gas_price_retryable(&eth_rpc_provider, &eth_rpc_provider).await;

            assert!(result.is_ok());
            // kill Anvil
        }
        let result = get_gas_price_retryable(&eth_rpc_provider, &eth_rpc_provider).await;
        assert!(matches!(result, Err(RetryError::Transient(_))));

        // restart Anvil
        let (_anvil, _payment_service) = setup_anvil(8548u16).await;
        let result = get_gas_price_retryable(&eth_rpc_provider, &eth_rpc_provider).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_response_retryable() {
        let listener = TcpListener::bind("localhost:8553").await.unwrap();

        let client_handle = tokio::spawn(async move {
            let stream = TcpStream::connect("localhost:8553")
                .await
                .expect("Failed to connect");

            let (mut ws_stream, _) = tokio_tungstenite::client_async("ws://localhost:8553", stream)
                .await
                .expect("WebSocket handshake failed");

            // Read the response from the server
            if let None = ws_stream.next().await {
                panic!("Failed to receive valid WebSocket response");
            }
        });

        let (raw_stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let ws_stream = tokio_tungstenite::accept_async(raw_stream).await.unwrap();
        let (outgoing, _incoming) = ws_stream.split();
        let outgoing = Arc::new(RwLock::new(outgoing));
        let message = "Some message".to_string();

        let result =
            connection::send_response_retryable(&outgoing, message.clone().into_bytes()).await;
        assert!(result.is_ok());
        client_handle.await.unwrap()
    }
}
