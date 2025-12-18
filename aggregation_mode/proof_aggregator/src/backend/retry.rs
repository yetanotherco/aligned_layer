use backon::ExponentialBuilder;
use backon::Retryable;
use std::future::Future;
use std::time::Duration;
use tracing::info;

use crate::aggregators::AlignedProof;
use crate::backend::types::AlignedProofAggregationServiceContract;
use crate::backend::AggregatedProofSubmissionError;

use crate::backend::helpers;
use alloy::{
    consensus::{BlobTransactionSidecar, EnvKzgSettings, EthereumTxEnvelope, TxEip4844WithSidecar},
    eips::{eip7594::BlobTransactionSidecarEip7594, Encodable2718},
    primitives::U256,
    providers::Provider,
    rpc::types::TransactionReceipt,
};
use risc0_ethereum_contracts::encode_seal;
use std::thread::sleep;

#[derive(Debug)]
pub enum RetryError<E> {
    Transient(E),
    //    Permanent(E),
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RetryError::Transient(e) => write!(f, "{e}"),
            //RetryError::Permanent(e) => write!(f, "{e}"),
        }
    }
}

impl<E> RetryError<E> {
    pub fn inner(self) -> E {
        match self {
            RetryError::Transient(e) => e,
            //RetryError::Permanent(e) => e,
        }
    }
}

impl<E: std::fmt::Display> std::error::Error for RetryError<E> where E: std::fmt::Debug {}

/// Supports retries only on async functions. See: https://docs.rs/backon/latest/backon/#retry-an-async-function
/// Runs with `jitter: false`.
pub async fn retry_function<FutureFn, Fut, T, E>(
    function: FutureFn,
    min_delay: u64,
    factor: f32,
    max_times: usize,
    max_delay: u64,
) -> Result<T, RetryError<E>>
where
    Fut: Future<Output = Result<T, RetryError<E>>>,
    FutureFn: FnMut() -> Fut,
{
    let backoff = ExponentialBuilder::default()
        .with_min_delay(Duration::from_millis(min_delay))
        .with_max_times(max_times)
        .with_factor(factor)
        .with_max_delay(Duration::from_secs(max_delay));

    function
        .retry(backoff)
        .sleep(tokio::time::sleep)
        .when(|e| matches!(e, RetryError::Transient(_)))
        .await
}

async fn wait_until_can_submit_aggregated_proof(
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    monthly_budget_eth: f64,
) -> Result<(), RetryError<AggregatedProofSubmissionError>> {
    // We start on 24 hours because the proof aggregator runs once a day, so the time elapsed
    // should be considered over a 24h period.
    let mut time_elapsed = Duration::from_secs(24 * 3600);

    // Iterate until we can send the proof on-chain
    loop {
        // Fetch gas price from network
        let gas_price = proof_aggregation_service
            .provider()
            .get_gas_price()
            .await
            .map_err(|e| {
                RetryError::Transient(AggregatedProofSubmissionError::GasPriceError(e.to_string()))
            })?;

        if helpers::should_send_proof_to_verify_on_chain(
            time_elapsed,
            monthly_budget_eth,
            U256::from(gas_price),
        ) {
            break;
        } else {
            info!("Skipping sending proof to ProofAggregationService contract due to budget/time constraints.");
        }

        // Sleep for 3 minutes (15 blocks) before re-evaluating
        let time_to_sleep = Duration::from_secs(180);
        time_elapsed += time_to_sleep;
        sleep(time_to_sleep);
    }

    Ok(())
}

pub async fn wait_and_send_proof_to_verify_on_chain(
    blob: BlobTransactionSidecar,
    blob_versioned_hash: [u8; 32],
    aggregated_proof: &AlignedProof,
    proof_aggregation_service: AlignedProofAggregationServiceContract,
    sp1_chunk_aggregator_vk_hash_bytes: [u8; 32],
    risc0_chunk_aggregator_image_id_bytes: [u8; 32],
    monthly_budget_eth: f64,
) -> Result<TransactionReceipt, RetryError<AggregatedProofSubmissionError>> {
    wait_until_can_submit_aggregated_proof(proof_aggregation_service.clone(), monthly_budget_eth)
        .await?;

    info!("Sending proof to ProofAggregationService contract...");

    let tx_req = match aggregated_proof {
        AlignedProof::SP1(proof) => proof_aggregation_service
            .verifyAggregationSP1(
                blob_versioned_hash.into(),
                proof.proof_with_pub_values.public_values.to_vec().into(),
                proof.proof_with_pub_values.bytes().into(),
                sp1_chunk_aggregator_vk_hash_bytes.into(),
            )
            .sidecar(blob)
            .into_transaction_request(),
        AlignedProof::Risc0(proof) => {
            let encoded_seal = encode_seal(&proof.receipt)
                .map_err(|e| AggregatedProofSubmissionError::Risc0EncodingSeal(e.to_string()))
                .map_err(RetryError::Transient)?;
            proof_aggregation_service
                .verifyAggregationRisc0(
                    blob_versioned_hash.into(),
                    encoded_seal.into(),
                    proof.receipt.journal.bytes.clone().into(),
                    risc0_chunk_aggregator_image_id_bytes.into(),
                )
                .sidecar(blob)
                .into_transaction_request()
        }
    };

    let provider = proof_aggregation_service.provider();
    let envelope = provider
        .fill(tx_req)
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?
        .try_into_envelope()
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;
    let tx: EthereumTxEnvelope<TxEip4844WithSidecar<BlobTransactionSidecarEip7594>> = envelope
        .try_into_pooled()
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?
        .try_map_eip4844(|tx| {
            tx.try_map_sidecar(|sidecar| sidecar.try_into_7594(EnvKzgSettings::Default.get()))
        })
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    let encoded_tx = tx.encoded_2718();
    let pending_tx = provider
        .send_raw_transaction(&encoded_tx)
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    let receipt = pending_tx
        .get_receipt()
        .await
        .map_err(|err| {
            AggregatedProofSubmissionError::SendVerifyAggregatedProofTransaction(err.to_string())
        })
        .map_err(RetryError::Transient)?;

    Ok(receipt)
}
