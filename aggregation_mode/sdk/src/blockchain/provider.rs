use crate::{
    beacon::BeaconClient,
    blockchain::{
        helpers::decoded_blob,
        types::{
            AlignedProofAggregationService, AlignedProofAggregationServiceContract, Hash32,
            ProofStatus, RPCProvider,
        },
        AggregationModeVerificationData, ProofVerificationAggModeError,
    },
    types::Network,
};
use alloy::{
    eips::BlockId,
    hex,
    primitives::Address,
    providers::{Provider, ProviderBuilder},
    rpc::types::{Filter, Log},
};
use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use std::str::FromStr;

/// How much to go back from current block if from_block is not provided
/// 7500 blocks = 25hr
const FROM_BLOCKS_AGO_DEFAULT: u64 = 7500;

pub struct ProofAggregationServiceProvider {
    rpc_provider: RPCProvider,
    beacon_client: BeaconClient,
    proof_aggregation_service_contract: AlignedProofAggregationServiceContract,
}

impl ProofAggregationServiceProvider {
    pub fn new(network: Network, rpc_url: String, beacon_client_url: String) -> Self {
        let rpc_url: reqwest::Url = rpc_url.parse().expect("rpc_url should be valid");
        let rpc_provider = ProviderBuilder::new().connect_http(rpc_url.clone());

        let beacon_client = BeaconClient::new(beacon_client_url);

        let proof_aggregation_service_contract = AlignedProofAggregationService::new(
            // safe unwrap, we know the address in network enum is valid
            Address::from_str(&network.proof_aggregator_contract_address()).unwrap(),
            rpc_provider.clone(),
        );

        Self {
            rpc_provider,
            proof_aggregation_service_contract,
            beacon_client,
        }
    }

    pub async fn check_proof_verification(
        &self,
        from_block: Option<u64>,
        verification_data: AggregationModeVerificationData,
    ) -> Result<ProofStatus, ProofVerificationAggModeError> {
        let logs = self.fetch_verified_proofs_events(from_block).await?;
        let proof_commitment = verification_data.commitment();

        for log in logs {
            let (merkle_root, leaves) = self.get_blob_data_from_verified_proof_event(log).await?;

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

    async fn fetch_verified_proofs_events(
        &self,
        from_block: Option<u64>,
    ) -> Result<Vec<Log>, ProofVerificationAggModeError> {
        let from_block = match from_block {
            Some(from_block) => from_block,
            None => {
                let block_number = self.rpc_provider.get_block_number().await.map_err(|e| {
                    ProofVerificationAggModeError::EthereumProviderError(e.to_string())
                })?;

                block_number.saturating_sub(FROM_BLOCKS_AGO_DEFAULT)
            }
        };

        let filter = Filter::new()
            .address(*self.proof_aggregation_service_contract.address())
            .event("AggregatedProofVerified(bytes32,bytes32)")
            .from_block(from_block);

        let logs = self
            .rpc_provider
            .get_logs(&filter)
            .await
            .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?;

        Ok(logs)
    }

    async fn get_blob_data_from_verified_proof_event(
        &self,
        log: Log,
    ) -> Result<([u8; 32], Vec<[u8; 32]>), ProofVerificationAggModeError> {
        // First 32 bytes of the data are the bytes of the blob versioned hash
        let blob_versioned_hash: [u8; 32] = log.data().data[0..32]
            .try_into()
            .map_err(|_| ProofVerificationAggModeError::EventDecoding)?;

        // Event is indexed by merkle root
        let merkle_root = log.topics()[1].0;

        // Block Number shouldn't be empty, in case it is,
        // there is a problem with this log, and we skip it
        // This same logic is replicated for other checks.
        let Some(block_number) = log.block_number else {
            return Err(ProofVerificationAggModeError::EventDecoding);
        };

        let Some(block) = self
            .rpc_provider
            .get_block(BlockId::Number(alloy::eips::BlockNumberOrTag::Number(
                block_number,
            )))
            .await
            .map_err(|e| ProofVerificationAggModeError::EthereumProviderError(e.to_string()))?
        else {
            return Err(ProofVerificationAggModeError::EventDecoding);
        };

        let Some(beacon_parent_root) = block.header.parent_beacon_block_root else {
            return Err(ProofVerificationAggModeError::EventDecoding);
        };

        let Some(beacon_block) = self
            .beacon_client
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

        let Some(blob_data) = self
            .beacon_client
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
}
