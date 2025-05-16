// Modules

mod helpers;
mod types;

// Makes only the two types on this use public
pub use types::{AggregationModeVerificationData, ProofVerificationAggModeError};

use helpers::{fetch_verified_proofs_events, get_blob_data_from_verified_proof_event};
use types::Hash32;

//
use crate::{
    common::types::Network, eth::aligned_proof_agg_service::aligned_proof_aggregation_service,
};
use ethers::{
    providers::{Http, Provider},
    types::Bytes,
};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;

/// Given the [`AggregationModeVerificationData`], this function checks whether the proof was included in a
/// in a recent aggregated proof and verifies the corresponding Merkle root commitment.
///
/// Note: This functionality is currently in Beta. As a result, we cannot determine with certainty
/// which specific aggregation a proof belongs to. Instead, we check the events from the specified `from_block`.
///
/// Note: The `from_block`  must not be older than 18 days,
/// as blobs expire after that period and will no longer be retrievable.
/// If not provided, it  defaults to fetch logs from [`FROM_BLOCKS_AGO_DEFAULT`]
///
/// The step-by-step verification process includes:
/// 1. Querying the blob versioned hash from the events emitted by the aligned proof aggregation service contract since `from_block`
/// 2. Retrieving the corresponding beacon block using the block's parent beacon root
/// 3. Fetching the blobs associated with that slot
/// 4. Filtering the blob that matches the queried blob versioned hash
/// 5. Decoding the blob to extract the proofs commitments
/// 6. Checking if the given proof commitment exists within the blob's proofs
/// 7. Reconstructing the Merkle root and verifying it against the root stored in the contract
pub async fn is_proof_verified(
    verification_data: AggregationModeVerificationData,
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: Option<u64>,
) -> Result<[u8; 32], ProofVerificationAggModeError> {
    let logs = fetch_verified_proofs_events(network, eth_rpc_url.clone(), from_block).await?;

    for log in logs {
        let Ok((merkle_root, leaves)) = get_blob_data_from_verified_proof_event(
            eth_rpc_url.clone(),
            beacon_client_url.clone(),
            log,
        )
        .await
        else {
            continue;
        };

        let leaves: Vec<Hash32> = leaves.iter().map(|leaf| Hash32(*leaf)).collect();
        let Some(merkle_tree) = MerkleTree::<Hash32>::build(&leaves) else {
            continue;
        };

        if leaves.contains(&Hash32(verification_data.commitment())) {
            return if merkle_tree.root == merkle_root {
                Ok(merkle_root)
            } else {
                Err(ProofVerificationAggModeError::MerkleTreeProofVerification)
            };
        }
    }

    Err(ProofVerificationAggModeError::ProofNotFoundInLogs)
}

/// Performs the same verification as [`is_proof_verified`], but instead of verifying locally, it simulates
/// an on-chain verification by calling the `verifyProofInclusion` function on the `ProofAggregationService` contract.
///
/// This function:
/// 1. Fetches the aggregated proof blob from the blockchain.
/// 2. Constructs the corresponding Merkle tree from the proof commitments.
/// 3. Calls the contract's `verifyProofInclusion` function with:
///     - The Merkle path corresponding to the given [`AggregationModeVerificationData`].
///     - The proof commitment, computed from the program ID and public inputs.
///
/// This is mainly useful for testing and simulation purposes to ensure that a given proof commitment
/// would be accepted by the contract on-chain. For typical off-chain verification (e.g., in services or indexers),
/// prefer using [`is_proof_verified`].
///
/// Note: This function does not perform the actual on-chain transaction but simulates the contract call.
pub async fn is_proof_verified_on_chain(
    verification_data: AggregationModeVerificationData,
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: Option<u64>,
) -> Result<bool, ProofVerificationAggModeError> {
    let Some(merkle_path) = get_merkle_path_for_proof(
        network.clone(),
        eth_rpc_url.clone(),
        beacon_client_url,
        from_block,
        &verification_data,
    )
    .await?
    else {
        return Ok(false);
    };

    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;
    let contract_provider = aligned_proof_aggregation_service(
        eth_rpc_provider,
        network.get_aligned_proof_agg_service_address(),
    )
    .await
    .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    let res = contract_provider
        .verify_proof_inclusion(
            merkle_path,
            verification_data.program_id(),
            Bytes::from(verification_data.public_inputs().clone()),
        )
        .call()
        .await
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    Ok(res)
}

/// Given the [`AggregationModeVerificationData`], this function queries the blockchain logs starting from the
/// specified `from_block` until it founds the proof.
///
/// Once the proof is found:
/// 1. It retrieves the corresponding proof blob.
/// 2. Constructs the Merkle tree based on the proof blob.
/// 3. Returns the Merkle proof needed for verifying the proof.
///
/// Note: This function prepares the Merkle path for on-chain verification, and is typically used in combination with
/// `verifyProofInclusion` to confirm proof validity within the ProofAggregationService contract.
pub async fn get_merkle_path_for_proof(
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: Option<u64>,
    verification_data: &AggregationModeVerificationData,
) -> Result<Option<Vec<[u8; 32]>>, ProofVerificationAggModeError> {
    let logs = fetch_verified_proofs_events(network, eth_rpc_url.clone(), from_block).await?;
    let proof_commitment = verification_data.commitment();

    for log in logs {
        let (merkle_root, leaves) = get_blob_data_from_verified_proof_event(
            eth_rpc_url.clone(),
            beacon_client_url.clone(),
            log,
        )
        .await?;

        let leaves: Vec<Hash32> = leaves.iter().map(|leaf| Hash32(*leaf)).collect();
        let Some(merkle_tree) = MerkleTree::<Hash32>::build(&leaves) else {
            continue;
        };

        let Some(pos) = leaves.iter().position(|p| p.0 == proof_commitment) else {
            continue;
        };
        let Some(proof) = merkle_tree.get_proof_by_pos(pos) else {
            continue;
        };

        let result = proof.verify::<Hash32>(&merkle_root, pos, &Hash32(proof_commitment));
        if !result {
            return Err(ProofVerificationAggModeError::MerkleTreeProofVerification);
        }

        return Ok(Some(proof.merkle_path));
    }

    Ok(None)
}
