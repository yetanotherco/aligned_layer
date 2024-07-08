use std::{collections::HashSet, time::Duration};

use futures_util::StreamExt;
use log::{debug, error};
use tokio::time::timeout;

use crate::{
    core::errors,
    core::types::{
        AlignedVerificationData, BatchInclusionData, VerificationCommitmentBatch,
        VerificationDataCommitment,
    },
    eth::BatchVerifiedEventStream,
};

const AWAIT_BATCH_VERIFICATION_TIMEOUT: u64 = 60;

pub async fn handle_batch_inclusion_data<'s>(
    batch_inclusion_data: BatchInclusionData,
    aligned_verification_data: &mut Vec<AlignedVerificationData>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
    stream: &mut BatchVerifiedEventStream<'s>,
    verified_batch_merkle_roots: &mut HashSet<Vec<u8>>,
) -> Result<(), errors::SubmitError> {
    handle_batch_inclusion_data_without_await(
        batch_inclusion_data.clone(),
        aligned_verification_data,
        verification_data_commitments_rev,
    )
    .await?;

    let batch_merkle_root = batch_inclusion_data.batch_merkle_root.to_vec();

    if !verified_batch_merkle_roots.contains(&batch_merkle_root) {
        await_batch_verification(stream, &batch_inclusion_data.batch_merkle_root).await?;
        verified_batch_merkle_roots.insert(batch_merkle_root);
    }

    Ok(())
}

pub async fn handle_batch_inclusion_data_without_await<'s>(
    batch_inclusion_data: BatchInclusionData,
    aligned_verification_data: &mut Vec<AlignedVerificationData>,
    verification_data_commitments_rev: &mut Vec<VerificationDataCommitment>,
) -> Result<(), errors::SubmitError> {
    debug!("Received response from batcher");
    debug!(
        "Batch merkle root: {}",
        hex::encode(batch_inclusion_data.batch_merkle_root)
    );
    debug!("Index in batch: {}", batch_inclusion_data.index_in_batch);

    let verification_data_commitment =
        verification_data_commitments_rev.pop().ok_or_else(|| {
            errors::SubmitError::GenericError("Verification data commitments are empty".to_string())
        })?;

    if verify_response(&verification_data_commitment, &batch_inclusion_data) {
        aligned_verification_data.push(AlignedVerificationData::new(
            &verification_data_commitment,
            &batch_inclusion_data,
        ));
    }

    Ok(())
}

async fn await_batch_verification<'s>(
    stream: &mut BatchVerifiedEventStream<'s>,
    batch_merkle_root: &[u8; 32],
) -> Result<(), errors::SubmitError> {
    let await_batch_verified_fut = await_batch_verified_event(stream, batch_merkle_root);

    match timeout(
        Duration::from_secs(AWAIT_BATCH_VERIFICATION_TIMEOUT),
        await_batch_verified_fut,
    )
    .await
    {
        Ok(Ok(_)) => {
            debug!("Batch operator signatures verified on Ethereum");
        }
        Ok(Err(e)) => {
            error!(
                "Error awaiting for batch signature verification event: {}",
                e
            );
        }
        Err(_) => {
            debug!("Batch operator signatures were not verified yet on Ethereum");
        }
    }

    Ok(())
}

// Await for the `BatchVerified` event emitted by the Aligned contract and then send responses.
async fn await_batch_verified_event<'s>(
    events_stream: &mut BatchVerifiedEventStream<'s>,
    batch_merkle_root: &[u8; 32],
) -> Result<(), errors::SubmitError> {
    while let Some(event_result) = events_stream.next().await {
        if let Ok(event) = event_result {
            if &event.batch_merkle_root == batch_merkle_root {
                debug!("Batch operator signatures verified on Ethereum");
                break;
            }
        } else {
            error!("Error awaiting for batch signature verification event");
            return Err(errors::SubmitError::BatchVerifiedEventStreamError(
                "Error awaiting for batch signature verification event".to_string(),
            ));
        }
    }
    Ok(())
}

fn verify_response(
    verification_data_commitment: &VerificationDataCommitment,
    batch_inclusion_data: &BatchInclusionData,
) -> bool {
    debug!("Verifying response data matches sent proof data ...");
    let batch_inclusion_proof = batch_inclusion_data.batch_inclusion_proof.clone();

    if batch_inclusion_proof.verify::<VerificationCommitmentBatch>(
        &batch_inclusion_data.batch_merkle_root,
        batch_inclusion_data.index_in_batch,
        verification_data_commitment,
    ) {
        debug!("Done. Data sent matches batcher answer");
        return true;
    }

    error!("Verification data commitments and batcher response with merkle root {} and index in batch {} don't match", hex::encode(batch_inclusion_data.batch_merkle_root), batch_inclusion_data.index_in_batch);
    false
}
