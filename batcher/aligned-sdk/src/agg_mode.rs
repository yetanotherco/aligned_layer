use crate::{
    beacon::{BeaconClient, BeaconClientError},
    core::types::{Network, ProvingSystemId, VerificationData},
    eth::aligned_proof_agg_service::aligned_proof_aggregation_service,
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::Filter,
};
use sha3::{Digest, Keccak256};

pub enum ProofVerificationAggModeError {
    ProvingSystemNotSupportedInAggMode,
    EthereumProviderError(String),
    BeaconClient(BeaconClientError),
}

/// Given aligned verification data, it verifies if the proof was verified in the last aggregated proof
/// Currently, this in Beta mode so there isn't a way to know exactly to which proof it belongs
/// So currently we check if included in the last one and verify the merkle root commitment
pub async fn is_proof_verified_in_aggregation_mode(
    aligned_verification_data: &VerificationData,
    network: Network,
    eth_rpc_url: String,
    beacon_client_url: String,
    from_block: u64,
) -> Result<bool, ProofVerificationAggModeError> {
    let supported = match aligned_verification_data.proving_system {
        ProvingSystemId::SP1 => true,
        _ => false,
    };

    if !supported {
        return Err(ProofVerificationAggModeError::ProvingSystemNotSupportedInAggMode);
    }

    // TODO: check if the from_block is past 18 days as the blob_data won't be available anymore

    let proof_hash: [u8; 32] = match aligned_verification_data.proving_system {
        ProvingSystemId::SP1 => {
            let mut hasher = Keccak256::new();
            let vk = aligned_verification_data.verification_key.clone().unwrap();
            let public_inputs = aligned_verification_data.pub_input.clone().unwrap();
            hasher.update(&vk);
            hasher.update(&public_inputs);
            hasher.finalize().into()
        }
        // we already filter the supported ones
        _ => unreachable!(),
    };

    /// We have to
    /// 1. Query the blob versioned hash of latest event from aligned proof aggregation service contract
    /// 2. Get the beacon block via the block parent beacon root
    /// 3. Fetch the blobs for that slot
    /// 4. Filter the blob with the blob versioned hash
    /// 5. Decode the blobs proofs
    /// 6. Find if the proofs hash is inside the blob proofs
    /// 7. Construct merkle root and verify it matches the one in the contract
    let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url)
        .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

    let filter = Filter::new()
        .event("AggregatedProofVerified(bytes32,bytes32)")
        .from_block(from_block);

    let mut to_check: Vec<(String, String, u64)> = vec![];

    let logs = eth_rpc_provider.get_logs(&filter).await.unwrap();
    for log in logs {
        let blob_versioned_hash = String::from_utf8(log.data[0..66].to_vec()).unwrap();
        let merkle_root = String::from_utf8(log.topics.get(1).unwrap().0.to_vec()).unwrap();
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

        let block = beacon_client
            .get_block_header_from_parent_hash(beacon_parent_root.0)
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
            .unwrap();

        let blob = beacon_client
            .get_blob_by_versioned_hash(block.header.message.slot, blob_versioned_hash.clone())
            .await
            .map_err(ProofVerificationAggModeError::BeaconClient)?
            .unwrap();

        let proof_hashes = decoded_blob(blob.blob.into());

        // decoded blob and get all leaves and see if it the has is inside
        if proof_hashes.contains(&blob_versioned_hash) {
            return Ok(verify_merkle_root(proof_hashes, merkle_root));
        } else {
            continue;
        }
    }

    Ok(false)
}

fn decoded_blob(blob_data: Vec<u8>) -> Vec<String> {
    let proof_hashes = vec![];

    proof_hashes
}

fn verify_merkle_root(proof_hashes: Vec<String>, merkle_root: String) -> bool {
    true
}
