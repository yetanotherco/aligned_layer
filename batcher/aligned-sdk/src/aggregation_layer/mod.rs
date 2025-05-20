// Modules
mod helpers;
mod types;

// Makes only the two types on this use public
pub use types::{AggregationModeVerificationData, ProofVerificationAggModeError};

use crate::{
    common::types::Network, eth::aligned_proof_agg_service::aligned_proof_aggregation_service,
};
use ethers::{
    providers::{Http, Provider},
    types::Bytes,
};
use helpers::{fetch_verified_proofs_events, get_blob_data_from_verified_proof_event};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use types::Hash32;

pub enum ProofStatus {
    Verified {
        merkle_root: [u8; 32],
        merkle_path: Vec<[u8; 32]>,
    },
    Invalid,
    NotFound,
}

/// Given the [`AggregationModeVerificationData`], this function checks whether the proof was included
/// in a recent aggregated proof and verifies the corresponding Merkle root commitment.
///
/// ### Notes
/// - This functionality is currently in Beta. As a result, we cannot determine with certainty.
///   which specific aggregation a proof belongs to. Instead, we check the events from the specified `from_block`.
/// - The `from_block`  must not be older than 18 days, as blobs expire after that period and will no longer be retrievable.
/// - If not provided, it  defaults to fetch logs from [`FROM_BLOCKS_AGO_DEFAULT`]
///
/// ### The verification process includes:
/// 1. Querying the blob versioned hash from the events emitted by the aligned proof aggregation service contract since `from_block`
/// 2. Retrieving the corresponding beacon block using the block's parent beacon root
/// 3. Fetching the blobs associated with that slot
/// 4. Filtering the blob that matches the queried blob versioned hash
/// 5. Decoding the blob to extract the proofs commitments
/// 6. Checking if the given proof commitment exists within the blob's proofs
/// 7. Reconstructing the Merkle root and verifying it against the root stored in the contract
///
/// This function is typically used in conjunction with `verifyProofInclusion` for complete on-chain verification.
pub async fn check_proof_verification(
    verification_data: &AggregationModeVerificationData,
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: Option<u64>,
) -> Result<ProofStatus, ProofVerificationAggModeError> {
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

        let Some(pos) = leaves.iter().position(|p| p.0 == proof_commitment) else {
            continue;
        };

        let Some(merkle_tree) = MerkleTree::<Hash32>::build(&leaves) else {
            continue;
        };

        let Some(proof) = merkle_tree.get_proof_by_pos(pos) else {
            continue;
        };

        let result = proof.verify::<Hash32>(&merkle_root, pos, &Hash32(proof_commitment));
        if !result {
            return Ok(ProofStatus::Invalid);
        }

        return Ok(ProofStatus::Verified {
            merkle_path: proof.merkle_path,
            merkle_root,
        });
    }

    Ok(ProofStatus::NotFound)
}

/// Simulates an on-chain verification of the proof by calling the `verifyProofInclusion` function
/// on the `ProofAggregationService` contract.
///
/// This function is intended to complement [`check_proof_verification`], which performs off-chain verification.
/// After calling `check_proof_verification` to confirm the proof's inclusion and obtain the Merkle path,
/// this function can be used to simulate the corresponding contract call.
///
/// ### How it works:
/// 1. Uses the provided Merkle path (as returned by [`check_proof_verification`]).
/// 2. Calls the `verifyProofInclusion` function on the contract with:
///     - The Merkle path,
///     - The proof program id.
///     - The proof public inputs bytes
///
/// ### Purpose:
/// This is mainly useful for **testing or simulation**, to confirm that the on-chain contract would
/// accept a given proof commitment and Merkle path. It does **not** perform an actual transaction on-chain,
/// but instead simulates the call via `eth_call`.
///
/// For off-chain verification use cases, prefer using [`check_proof_verification`].
pub async fn is_proof_verified_on_chain(
    verification_data: AggregationModeVerificationData,
    merkle_path: Vec<[u8; 32]>,
    network: Network,
    eth_rpc_url: String,
) -> Result<bool, ProofVerificationAggModeError> {
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
