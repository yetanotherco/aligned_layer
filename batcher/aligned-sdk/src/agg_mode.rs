use crate::{
    beacon::{BeaconClient, BeaconClientError},
    core::types::Network,
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Filter,
};
use sha3::{Digest, Keccak256};

/// How much to go back from current block if from_block is not provided
/// 7500 blocks = 25hr
const FROM_BLOCKS_AGO_DEFAULT: u64 = 7500;

#[derive(Debug)]
pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
    UnmatchedBlobAndEventMerkleRoot,
    ProofNotFoundInLogs,
    EventDecoding,
}

/// Given aligned verification data, this function checks whether a proof was included
/// in the most recent aggregated proof and verifies the corresponding Merkle root commitment.
///
/// Note: This functionality is currently in Beta. As a result, we cannot determine with certainty
/// which specific aggregation a proof belongs to. Instead, we optimistically check the latest one.
///
/// ⚠️ The `from` block used in the verification process must not be older than 18 days,
/// as blobs expire after that period and will no longer be retrievable.
/// If not provided, it  defaults to fetch logs from the past 25hs
///
/// The step-by-step verification process includes:
/// 1. Querying the blob versioned hash from the latest event emitted by the aligned proof aggregation service contract
/// 2. Retrieving the corresponding beacon block using the block’s parent beacon root
/// 3. Fetching the blobs associated with that slot
/// 4. Filtering the blob that matches the queried blob versioned hash
/// 5. Decoding the blob to extract the proofs
/// 6. Checking if the given proof hash exists within the blob’s proofs
/// 7. Reconstructing the Merkle root and verifying it against the commitment stored in the contract
pub async fn is_proof_verified_in_aggregation_mode(
    proof_hash: [u8; 32],
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: Option<u64>,
) -> Result<[u8; 32], ProofVerificationAggModeError> {
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;
    let beacon_client = BeaconClient::new(beacon_client_url);

    let from_block = match from_block {
        Some(from_block) => from_block,
        None => {
            let block_number = eth_rpc_provider
                .get_block_number()
                .await
                .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;
            block_number.as_u64() - FROM_BLOCKS_AGO_DEFAULT
        }
    };

    let filter = Filter::new()
        .address(network.get_aligned_proof_agg_service_address())
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(from_block);

    let logs = eth_rpc_provider.get_logs(&filter).await.unwrap();
    for log in logs {
        let blob_versioned_hash: [u8; 32] = log.data[0..32]
            .try_into()
            .map_err(|_| ProofVerificationAggModeError::EventDecoding)?;
        let merkle_root = log.topics[1].0;
        let Some(block_number) = log.block_number else {
            continue;
        };

        let Some(block) = eth_rpc_provider
            .get_block(block_number.as_u64())
            .await
            .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?
        else {
            continue;
        };

        let Some(beacon_parent_root) = block.parent_beacon_block_root else {
            continue;
        };

        let Some(beacon_block) = beacon_client
            .get_block_header_from_parent_hash(beacon_parent_root.0)
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
        else {
            continue;
        };

        let Some(blob) = beacon_client
            .get_blob_by_versioned_hash(
                beacon_block
                    .header
                    .message
                    .slot
                    .parse()
                    .expect("Slot to be parsable number"),
                blob_versioned_hash,
            )
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
        else {
            continue;
        };

        let blob_data = hex::decode(blob.blob.replace("0x", "")).expect("A valid hex encoded data");
        let proof_hashes = decoded_blob(blob_data);

        if proof_hashes.contains(&proof_hash) {
            if verify_blob_merkle_root(proof_hashes, merkle_root) {
                return Ok(merkle_root);
            } else {
                return Err(ProofVerificationAggModeError::UnmatchedBlobAndEventMerkleRoot);
            }
        } else {
            continue;
        }
    }

    Err(ProofVerificationAggModeError::ProofNotFoundInLogs)
}

fn decoded_blob(blob_data: Vec<u8>) -> Vec<[u8; 32]> {
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

pub fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(hash_a);
    hasher.update(hash_b);
    hasher.finalize().into()
}

fn verify_blob_merkle_root(mut proof_hashes: Vec<[u8; 32]>, merkle_root: [u8; 32]) -> bool {
    while proof_hashes.len() > 1 {
        proof_hashes = proof_hashes
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => combine_hashes(a, b),
                [a] => combine_hashes(a, a),
                _ => panic!("Unexpected chunk size in leaves"),
            })
            .collect()
    }

    proof_hashes[0] == merkle_root
}
