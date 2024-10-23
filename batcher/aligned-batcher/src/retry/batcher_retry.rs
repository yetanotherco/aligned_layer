use ethers::prelude::*;
use log::warn;

use crate::{eth::payment_service::BatcherPaymentService, retry::RetryError};

pub async fn get_user_balance_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: &Address,
) -> Result<U256, RetryError<String>> {
    if let Ok(balance) = payment_service.user_balances(*addr).call().await {
        return Ok(balance);
    };

    payment_service_fallback
        .user_balances(*addr)
        .call()
        .await
        .map_err(|e| {
            warn!("Failed to get balance for address {:?}. Error: {e}", addr);
            RetryError::Transient(e.to_string())
        })
}

pub async fn get_user_nonce_from_ethereum_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: Address,
) -> Result<U256, RetryError<String>> {
    if let Ok(nonce) = payment_service.user_nonces(addr).call().await {
        return Ok(nonce);
    }
    payment_service_fallback
        .user_nonces(addr)
        .call()
        .await
        .map_err(|e| {
            warn!("Error getting user nonce: {e}");
            RetryError::Transient(e.to_string())
        })
}

pub async fn user_balance_is_unlocked_retryable(
    payment_service: &BatcherPaymentService,
    payment_service_fallback: &BatcherPaymentService,
    addr: &Address,
) -> Result<bool, RetryError<()>> {
    if let Ok(unlock_block) = payment_service.user_unlock_block(*addr).call().await {
        return Ok(unlock_block != U256::zero());
    }
    if let Ok(unlock_block) = payment_service_fallback
        .user_unlock_block(*addr)
        .call()
        .await
    {
        return Ok(unlock_block != U256::zero());
    }
    warn!("Failed to get user locking state {:?}", addr);
    Err(RetryError::Transient(()))
}

pub async fn get_gas_price_retryable(
    eth_ws_provider: &Provider<Http>,
    eth_ws_provider_fallback: &Provider<Http>,
) -> Result<U256, RetryError<String>> {
    if let Ok(gas_price) = eth_ws_provider
        .get_gas_price()
        .await
        .inspect_err(|e| warn!("Failed to get gas price. Trying with fallback: {e:?}"))
    {
        return Ok(gas_price);
    }

    eth_ws_provider_fallback.get_gas_price().await.map_err(|e| {
        warn!("Failed to get fallback gas price: {e:?}");
        RetryError::Transient(e.to_string())
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        config::{ContractDeploymentOutput, ECDSAConfig},
        eth::{self, get_provider, payment_service::BatcherPaymentService},
    };
    use ethers::{
        contract::abigen,
        types::{Address, U256},
        utils::{Anvil, AnvilInstance},
    };
    use std::str::FromStr;

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

        let deployment_output = ContractDeploymentOutput::new(
            "../../contracts/script/output/devnet/alignedlayer_deployment_output.json".to_string(),
        );

        let payment_service_addr = deployment_output.addresses.batcher_payment_service.clone();

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
    async fn test_user_balance_is_unlocked_retryable() {
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
        let (_anvil, _) = setup_anvil(8546u16).await;
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
    async fn test_get_user_nonce_retryable() {
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
        let (_anvil, _) = setup_anvil(8547u16).await;

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
    async fn test_get_gas_price_retryable() {
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
        let (_anvil, _) = setup_anvil(8548u16).await;
        let result = get_gas_price_retryable(&eth_rpc_provider, &eth_rpc_provider).await;

        assert!(result.is_ok());
    }
}
