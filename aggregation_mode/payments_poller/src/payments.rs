use std::{str::FromStr, sync::Arc};

use crate::{
    config::Config,
    db::Db,
    metrics::PaymentsPollerMetrics,
    types::{AggregationModePaymentService, AggregationModePaymentServiceContract, RpcProvider},
};
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use sqlx::types::BigDecimal;

#[derive(Debug, Clone)]
pub enum PaymentsPollerError {
    ReadLastBlockError(String),
}

pub struct PaymentsPoller {
    db: Db,
    proof_aggregation_service: AggregationModePaymentServiceContract,
    rpc_provider: RpcProvider,
    config: Config,
    metrics: Arc<PaymentsPollerMetrics>,
}

impl PaymentsPoller {
    pub fn new(db: Db, config: Config) -> Result<Self, PaymentsPollerError> {
        let rpc_url = config.eth_rpc_url.parse().expect("RPC URL should be valid");
        let rpc_provider = ProviderBuilder::new().connect_http(rpc_url);
        let proof_aggregation_service = AggregationModePaymentService::new(
            Address::from_str(&config.payment_service_address)
                .expect("AggregationModePaymentService address should be valid"),
            rpc_provider.clone(),
        );

        // This check is here to catch early failures on last block fetching
        let _ = config
            .get_last_block_fetched()
            .map_err(|err| PaymentsPollerError::ReadLastBlockError(err.to_string()));

        tracing::info!(
            "Starting metrics server on port {}",
            config.poller_metrics_port
        );
        let metrics = PaymentsPollerMetrics::start(config.poller_metrics_port)
            .expect("Failed to start metrics server");

        Ok(Self {
            db,
            proof_aggregation_service,
            rpc_provider,
            config,
            metrics,
        })
    }

    pub async fn start(&self) {
        let seconds_to_wait_between_polls = 12;

        loop {
            let Ok(last_block_fetched) = self.config.get_last_block_fetched() else {
                tracing::warn!("Could not get last block fetched, skipping polling iteration...");
                tokio::time::sleep(std::time::Duration::from_secs(
                    seconds_to_wait_between_polls,
                ))
                .await;
                continue;
            };

            let Ok(current_block) = self.rpc_provider.get_block_number().await else {
                tracing::warn!("Could not get current block skipping polling iteration...");
                tokio::time::sleep(std::time::Duration::from_secs(
                    seconds_to_wait_between_polls,
                ))
                .await;
                continue;
            };

            let start_block = last_block_fetched.saturating_sub(5);
            tracing::info!("Fetching logs from block {start_block} to {current_block}");

            let Ok(logs) = self
                .proof_aggregation_service
                .UserPayment_filter()
                .from_block(start_block)
                .to_block(current_block)
                .query()
                .await
            else {
                tracing::warn!("Could not get payment log events skipping polling iteration...");
                tokio::time::sleep(std::time::Duration::from_secs(
                    seconds_to_wait_between_polls,
                ))
                .await;
                continue;
            };

            tracing::info!("Logs collected {}", logs.len());

            for (payment_event, log) in logs {
                let address = format!("{:#x}", payment_event.user);
                let Some(tx_hash) = log.transaction_hash else {
                    tracing::warn!("Skipping payment event for {address}: missing tx hash");
                    continue;
                };
                let tx_hash = format!("{tx_hash:#x}");

                let Ok(amount) = BigDecimal::from_str(&payment_event.amount.to_string()) else {
                    continue;
                };
                let Ok(started_at) = BigDecimal::from_str(&payment_event.from.to_string()) else {
                    continue;
                };
                let Ok(valid_until) = BigDecimal::from_str(&payment_event.until.to_string()) else {
                    continue;
                };

                if let Err(err) = self
                    .db
                    .insert_payment_event(&address, &started_at, &amount, &valid_until, &tx_hash)
                    .await
                {
                    tracing::error!("Failed to insert payment event for {address}: {err}");
                }
            }

            if let Err(err) = self.config.update_last_block_fetched(current_block) {
                tracing::error!("Failed to update the last aggregated block: {err}");
                continue;
            };

            self.metrics.register_last_processed_block(current_block);

            tokio::time::sleep(std::time::Duration::from_secs(
                seconds_to_wait_between_polls,
            ))
            .await;
        }
    }
}
