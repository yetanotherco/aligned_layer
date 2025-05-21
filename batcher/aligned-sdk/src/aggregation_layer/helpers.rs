use crate::{beacon::BeaconClient, common::types::Network};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Filter, Log},
};

use super::ProofVerificationAggModeError;

/// How much to go back from current block if from_block is not provided
/// 7500 blocks = 25hr
const FROM_BLOCKS_AGO_DEFAULT: u64 = 7500;

pub async fn fetch_verified_proofs_events(
    network: Network,
    eth_rpc_url: String,
    from_block: Option<u64>,
) -> Result<Vec<Log>, ProofVerificationAggModeError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    let from_block = match from_block {
        Some(from_block) => from_block,
        None => {
            let block_number = eth_rpc_provider
                .get_block_number()
                .await
                .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

            block_number
                .as_u64()
                .saturating_sub(FROM_BLOCKS_AGO_DEFAULT)
        }
    };

    let filter = Filter::new()
        .address(network.get_aligned_proof_agg_service_address())
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(from_block);

    let logs = eth_rpc_provider
        .get_logs(&filter)
        .await
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    Ok(logs)
}

pub async fn get_blob_data_from_verified_proof_event(
    eth_rpc_url: String,
    beacon_client_url: String,
    log: Log,
) -> Result<([u8; 32], Vec<[u8; 32]>), ProofVerificationAggModeError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;
    let beacon_client = BeaconClient::new(beacon_client_url);

    // First 32 bytes of the data are the bytes of the blob versioned hash
    let blob_versioned_hash: [u8; 32] = log.data[0..32]
        .try_into()
        .map_err(|_| ProofVerificationAggModeError::EventDecoding)?;

    // Event is indexed by merkle root
    let merkle_root = log.topics[1].0;

    // Block Number shouldn't be empty, in case it is,
    // there is a problem with this log, and we skip it
    // This same logic is replicated for other checks.
    let Some(block_number) = log.block_number else {
        return Err(ProofVerificationAggModeError::EventDecoding);
    };

    let Some(block) = eth_rpc_provider
        .get_block(block_number.as_u64())
        .await
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?
    else {
        return Err(ProofVerificationAggModeError::EventDecoding);
    };

    let Some(beacon_parent_root) = block.parent_beacon_block_root else {
        return Err(ProofVerificationAggModeError::EventDecoding);
    };

    let Some(beacon_block) = beacon_client
        .get_block_header_from_parent_hash(beacon_parent_root.0)
        .await
        .map_err(ProofVerificationAggModeError::BeaconClient)?
    else {
        return Err(ProofVerificationAggModeError::EventDecoding);
    };

    let slot: u64 = beacon_block
        .header
        .message
        .slot
        .parse()
        .expect("Slot to be parsable number");

    let Some(blob_data) = beacon_client
        .get_blob_by_versioned_hash(slot, blob_versioned_hash)
        .await
        .map_err(ProofVerificationAggModeError::BeaconClient)?
    else {
        return Err(ProofVerificationAggModeError::EventDecoding);
    };

    let blob_bytes =
        hex::decode(blob_data.blob.replace("0x", "")).expect("A valid hex encoded data");
    let proof_commitments = decoded_blob(&blob_bytes);

    Ok((merkle_root, proof_commitments))
}

fn decoded_blob(blob_data: &[u8]) -> Vec<[u8; 32]> {
    let mut proof_hashes = vec![];

    let mut current_hash = [0u8; 32];
    let mut current_hash_count = 0;
    let mut total_bytes_count = 0;

    while total_bytes_count < blob_data.len() {
        // Every 32 bytes there is a 0x0 acting as padding, so we need to skip the byte
        let is_pad = total_bytes_count % 32 == 0;
        if is_pad {
            total_bytes_count += 1;
            continue;
        }

        current_hash[current_hash_count] = blob_data[total_bytes_count];

        if current_hash_count + 1 == 32 {
            // if the current_hash is the zero hash, then there are no more proofs in the blob
            if current_hash == [0u8; 32] {
                break;
            }
            proof_hashes.push(current_hash);
            current_hash = [0u8; 32];
            current_hash_count = 0;
        } else {
            current_hash_count += 1;
        }

        total_bytes_count += 1;
    }

    proof_hashes
}
