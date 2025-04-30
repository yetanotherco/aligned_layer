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
pub enum AggregationModeVerificationData {
    SP1 {
        vk: [u8; 32],
        public_inputs: Vec<u8>,
    },
    Risc0 {
        image_id: [u8; 32],
        public_inputs: Vec<u8>,
    },
}

impl AggregationModeVerificationData {
    fn commitment(&self) -> [u8; 32] {
        match self {
            AggregationModeVerificationData::SP1 { vk, public_inputs } => {
                let mut hasher = Keccak256::new();
                hasher.update(vk);
                hasher.update(public_inputs);
                hasher.finalize().into()
            }
            AggregationModeVerificationData::Risc0 {
                image_id,
                public_inputs,
            } => {
                let mut hasher = Keccak256::new();
                hasher.update(image_id);
                hasher.update(public_inputs);
                hasher.finalize().into()
            }
        }
    }
}

#[derive(Debug)]
pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
    UnmatchedBlobAndEventMerkleRoot,
    ProofNotFoundInLogs,
    EventDecoding,
}

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
pub async fn is_proof_verified_in_aggregation_mode(
    verification_data: AggregationModeVerificationData,
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

            block_number
                .as_u64()
                .saturating_sub(FROM_BLOCKS_AGO_DEFAULT)
        }
    };

    let filter = Filter::new()
        .address(network.get_aligned_proof_agg_service_address())
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(from_block);

    let logs = eth_rpc_provider.get_logs(&filter).await.unwrap();
    for log in logs {
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
            continue;
        };

        let blob_bytes =
            hex::decode(blob_data.blob.replace("0x", "")).expect("A valid hex encoded data");
        let proof_commitments = decoded_blob(blob_bytes);

        if proof_commitments.contains(&verification_data.commitment()) {
            return if verify_blob_merkle_root(proof_commitments, merkle_root) {
                Ok(merkle_root)
            } else {
                Err(ProofVerificationAggModeError::UnmatchedBlobAndEventMerkleRoot)
            };
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

pub fn combine_hashes(hash_a: &[u8; 32], hash_b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(hash_a);
    hasher.update(hash_b);
    hasher.finalize().into()
}

fn verify_blob_merkle_root(mut commitments: Vec<[u8; 32]>, merkle_root: [u8; 32]) -> bool {
    while commitments.len() > 1 {
        commitments = commitments
            .chunks(2)
            .map(|chunk| match chunk {
                [a, b] => combine_hashes(a, b),
                [a] => combine_hashes(a, a),
                _ => panic!("Unexpected chunk size in leaves"),
            })
            .collect()
    }

    commitments[0] == merkle_root
}
