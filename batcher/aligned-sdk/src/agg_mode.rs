use crate::{
    beacon::{BeaconClient, BeaconClientError},
    core::types::Network,
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Filter,
};

#[derive(Debug)]
pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
}

/// Given aligned verification data, it verifies if the proof was verified in the last aggregated proof
/// Currently, this in Beta mode so there isn't a way to know exactly to which proof it belongs
/// So currently we check if included in the last one and verify the merkle root commitment
/// The step by step verification consists of:
/// 1. Query the blob versioned hash of latest event from aligned proof aggregation service contract
/// 2. Get the beacon block via the block parent beacon root
/// 3. Fetch the blobs for that slot
/// 4. Filter the blob with the blob versioned hash
/// 5. Decode the blobs proofs
/// 6. Find if the proofs hash is inside the blob proofs
/// 7. Construct merkle root and verify it matches the one in the contract
pub async fn is_proof_verified_in_aggregation_mode(
    proof_hash: String,
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: u64,
) -> Result<bool, ProofVerificationAggModeError> {
    // TODO: check if the from_block is past 18 days as the blob_data won't be available anymore

    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    let filter = Filter::new()
        .address(network.get_aligned_proof_agg_service_address())
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(from_block);

    let mut to_check: Vec<([u8; 32], [u8; 32], u64)> = vec![];

    let logs = eth_rpc_provider.get_logs(&filter).await.unwrap();
    for log in logs {
        let blob_versioned_hash: [u8; 32] = log.data[0..32]
            .try_into()
            .expect("Data has incorrect length");
        let merkle_root = log.topics.get(1).expect("to decode merkle root in index").0;

        to_check.push((
            blob_versioned_hash,
            merkle_root,
            log.block_number.unwrap().0[0],
        ));
    }

    let beacon_client = BeaconClient::new(beacon_client_url);

    // Start checking each log and blob versioned hash
    for (blob_versioned_hash, merkle_root, block_number) in to_check {
        let block = eth_rpc_provider
            .get_block(block_number)
            .await
            .unwrap()
            .unwrap();
        let beacon_parent_root = block.parent_beacon_block_root.unwrap();

        let beacon_block = beacon_client
            .get_block_header_from_parent_hash(beacon_parent_root.0)
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
            .unwrap();

        let blob = beacon_client
            .get_blob_by_versioned_hash(
                beacon_block.header.message.slot.parse().expect("a number"),
                blob_versioned_hash,
            )
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
            .unwrap();

        let blob_data = hex::decode(blob.blob.replace("0x", "")).expect("A valid hex encoded data");

        let proof_hashes = decoded_blob(blob_data);

        // decoded blob and get all leaves and see if it the has is inside
        let proof_hash_bytes: [u8; 32] = hex::decode(proof_hash.replace("0x", ""))
            .unwrap()
            .try_into()
            .unwrap();

        if proof_hashes.contains(&proof_hash_bytes) {
            return Ok(verify_merkle_root(proof_hashes, merkle_root));
        } else {
            continue;
        }
    }

    Ok(false)
}

fn decoded_blob(blob_data: Vec<u8>) -> Vec<[u8; 32]> {
    let mut proof_hashes = vec![];

    let mut current_hash = [0u8; 32];
    let mut current_hash_count = 0;
    let mut total_bytes_count = 0;

    let mut i = 0;

    while i < blob_data.len() {
        // Every 32 bytes (or 64 characters) there is a 0x00 acting as padding, so we need to skip the byte (two iterations)
        let is_pad = total_bytes_count % 32 == 0;
        if is_pad {
            i += 1;
            total_bytes_count += 1;
            continue;
        }

        current_hash[current_hash_count] = blob_data[i];

        if current_hash_count + 1 == 32 {
            if current_hash == [0u8; 32] {
                break;
            }
            proof_hashes.push(current_hash);
            current_hash = [0u8; 32];
            current_hash_count = 0;
            continue;
        } else {
            current_hash_count += 1;
        }

        i += 1;
        total_bytes_count += 1;
    }

    proof_hashes
}

fn verify_merkle_root(proof_hashes: Vec<[u8; 32]>, merkle_root: [u8; 32]) -> bool {
    true
}
