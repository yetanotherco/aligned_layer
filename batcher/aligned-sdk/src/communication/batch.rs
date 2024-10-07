use log::debug;

use crate::{
    core::{
        errors,
        types::{
            AlignedVerificationData, BatchInclusionData, Network, VerificationCommitmentBatch,
            VerificationDataCommitment,
        },
    },
    sdk::is_proof_verified,
};

const RETRIES: u64 = 10;
const TIME_BETWEEN_RETRIES: u64 = 10;

pub fn handle_batch_inclusion_data(
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

    let verification_data_commitment = verification_data_commitments_rev
        .pop()
        .ok_or_else(|| errors::SubmitError::EmptyVerificationDataCommitments)?;

    if verify_response(&verification_data_commitment, &batch_inclusion_data) {
        aligned_verification_data.push(AlignedVerificationData::new(
            &verification_data_commitment,
            &batch_inclusion_data,
        ));
    }

    Ok(())
}

pub async fn await_batch_verification(
    aligned_verification_data: &AlignedVerificationData,
    rpc_url: &str,
    network: Network,
) -> Result<(), errors::SubmitError> {
    for _ in 0..RETRIES {
        if is_proof_verified(aligned_verification_data, network, rpc_url)
            .await
            .is_ok_and(|r| r)
        {
            return Ok(());
        }

        debug!(
            "Proof not verified yet. Waiting {} seconds before checking again...",
            TIME_BETWEEN_RETRIES
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(TIME_BETWEEN_RETRIES)).await;
    }
    Err(errors::SubmitError::BatchVerificationTimeout {
        timeout_seconds: (TIME_BETWEEN_RETRIES * RETRIES),
    })
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

    debug!("Verification data commitments and batcher response with merkle root {} and index in batch {} don't match", hex::encode(batch_inclusion_data.batch_merkle_root), batch_inclusion_data.index_in_batch);
    false
}
