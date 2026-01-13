use agg_mode_sdk::{
    blockchain::{
        provider::ProofAggregationServiceProvider, AggregationModeVerificationData, ProofStatus,
    },
    types::Network,
};
use alloy::hex;
use clap::{command, Args, Subcommand};
use std::path::PathBuf;

use crate::commands::helpers::parse_network;

#[derive(Debug, Subcommand)]
pub enum VerifyCommand {
    #[command(name = "sp1")]
    SP1(VerifySP1Args),
    #[command(name = "risc0")]
    Risc0(VerifyRisc0Args),
    #[command(name = "zisk")]
    Zisk(VerifyZiskArgs),
}

#[derive(Debug, Clone, Args)]
pub struct VerifySP1Args {
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
    #[arg(long = "beacon-url")]
    beacon_url: String,
    #[arg(long = "from-block")]
    from_block: Option<u64>,
    #[arg(long = "vk-hash")]
    vk_hash: PathBuf,
    #[arg(long = "public-inputs")]
    public_inputs: PathBuf,
}

#[derive(Debug, Clone, Args)]
pub struct VerifyRisc0Args {
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
    #[arg(long = "beacon-url")]
    beacon_url: String,
    #[arg(long = "from-block")]
    from_block: Option<u64>,
    #[arg(long = "image-id")]
    image_id: PathBuf,
    #[arg(long = "public-inputs")]
    public_inputs: PathBuf,
}

#[derive(Debug, Clone, Args)]
pub struct VerifyZiskArgs {
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
    #[arg(long = "beacon-url")]
    beacon_url: String,
    #[arg(long = "from-block")]
    from_block: Option<u64>,
    #[arg(short = 'p', long = "proof")]
    proof: PathBuf,
}

pub async fn run_sp1(args: VerifySP1Args) {
    tracing::info!("Verifying SP1 proof on {:?}...", args.network);

    let vk: [u8; 32] = std::fs::read(&args.vk_hash)
        .expect("to read vk hash file")
        .try_into()
        .expect("Invalid vk hash (expected 32 bytes)");

    let public_inputs = std::fs::read(&args.public_inputs).expect("to read public inputs file");

    let verification_data = AggregationModeVerificationData::SP1 { vk, public_inputs };

    verify_proof(
        args.network,
        args.rpc_url,
        args.beacon_url,
        args.from_block,
        verification_data,
    )
    .await;
}

pub async fn run_risc0(args: VerifyRisc0Args) {
    tracing::info!("Verifying Risc0 proof on {:?}...", args.network);

    let image_id: [u8; 32] = std::fs::read(&args.image_id)
        .expect("to read image id file")
        .try_into()
        .expect("Invalid image id (expected 32 bytes)");

    let public_inputs = std::fs::read(&args.public_inputs).expect("to read public inputs file");

    let verification_data = AggregationModeVerificationData::Risc0 {
        image_id,
        public_inputs,
    };

    verify_proof(
        args.network,
        args.rpc_url,
        args.beacon_url,
        args.from_block,
        verification_data,
    )
    .await;
}

pub async fn run_zisk(args: VerifyZiskArgs) {
    tracing::info!("Verifying Zisk proof on {:?}...", args.network);

    let proof = std::fs::read(&args.proof).expect("to read proof file");

    let verification_data = AggregationModeVerificationData::Zisk { proof };

    verify_proof(
        args.network,
        args.rpc_url,
        args.beacon_url,
        args.from_block,
        verification_data,
    )
    .await;
}

async fn verify_proof(
    network: Network,
    rpc_url: String,
    beacon_url: String,
    from_block: Option<u64>,
    verification_data: AggregationModeVerificationData,
) {
    let provider = ProofAggregationServiceProvider::new(network, rpc_url, beacon_url);

    let proof_status = match provider
        .check_proof_verification(from_block, verification_data)
        .await
    {
        Ok(res) => res,
        Err(e) => {
            tracing::error!("Error while trying to verify proof {:?}", e);
            return;
        }
    };

    match proof_status {
        ProofStatus::Verified { merkle_root, .. } => {
            tracing::info!(
                "Your proof has been verified in the aggregated proof with merkle root 0x{}",
                hex::encode(merkle_root)
            );
        }
        ProofStatus::Invalid => {
            tracing::error!(
                "Your proof was found in the blob but the Merkle Root verification failed."
            )
        }
        ProofStatus::NotFound => {
            tracing::error!("Your proof wasn't found in the logs. Try specifying an earlier `from_block` to search further back in history.")
        }
    }
}
