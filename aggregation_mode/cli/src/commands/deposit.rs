use agg_mode_sdk::types::Network;
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder},
};
use clap::{self, Args};

use crate::commands::helpers::{parse_network, PrivateKeyType};

const PAYMENT_AMOUNT: &str = "0.0035"; // ether

/// Send 1 ether to the aggregation mode payment service to fund proof submissions
#[derive(Debug, Clone, Args)]
pub struct SendPaymentArgs {
    #[command(flatten)]
    private_key_type: PrivateKeyType,
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
}

pub async fn run(args: SendPaymentArgs) {
    tracing::info!(
        "Sending payment to aggregation mode payment service on {:?}",
        args.network
    );

    let signer = match args.private_key_type.into_signer() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("{e}");
            return;
        }
    };

    let wallet = EthereumWallet::from(signer.clone());

    let rpc_url = args.rpc_url.parse().expect("Invalid RPC URL");
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    let payment_service_address_str = args.network.aggregation_mode_payment_service_address();
    let payment_service_address = match payment_service_address_str.parse::<Address>() {
        Ok(addr) => addr,
        Err(e) => {
            tracing::error!("Failed to parse payment service address: {e}");
            return;
        }
    };

    let amount_ether: f64 = PAYMENT_AMOUNT.parse().expect("Invalid payment amount");
    let amount_wei = U256::from((amount_ether * 1e18) as u64);

    let tx = alloy::rpc::types::TransactionRequest::default()
        .with_to(payment_service_address)
        .with_value(amount_wei);

    match provider.send_transaction(tx).await {
        Ok(pending_tx) => {
            tracing::info!("Transaction sent. Hash: {:?}", pending_tx.tx_hash());
            match pending_tx.watch().await {
                Ok(receipt) => {
                    tracing::info!(
                        "Payment of {} ether sent successfully to aggregation mode payment service at {}",
                        PAYMENT_AMOUNT,
                        payment_service_address_str
                    );
                    tracing::info!("Transaction receipt: {:?}", receipt);
                }
                Err(e) => {
                    tracing::error!("Failed to get transaction receipt: {e}");
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to send transaction: {e}");
        }
    }
}
