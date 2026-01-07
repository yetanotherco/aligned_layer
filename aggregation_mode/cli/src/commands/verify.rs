use agg_mode_sdk::{
    blockchain::{
        provider::ProofAggregationServiceProvider, AggregationModeVerificationData, ProofStatus,
    },
    types::Network,
};
use alloy::hex;
use clap::{self, Args};
use std::path::PathBuf;

use crate::commands::helpers::{parse_network, ProvingSystemArg};

#[derive(Debug, Clone, Args)]
pub struct VerifyOnChainArgs {
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
    #[arg(long = "rpc-url")]
    rpc_url: String,
    #[arg(long = "beacon-url")]
    beacon_url: String,
    #[arg(long = "from-block")]
    from_block: Option<u64>,
    #[arg(long = "proving-system")]
    proving_system: ProvingSystemArg,
    #[arg(
        name = "Program verification key hash",
        long = "vk-hash",
        required = true
    )]
    program_vk: PathBuf,
    #[arg(name = "Public input file name", long = "public-inputs")]
    pub_input_file_name: Option<PathBuf>,
}

pub async fn run(args: VerifyOnChainArgs) {
    let program_id_key: [u8; 32] = std::fs::read(&args.program_vk)
        .expect("to read program vk file")
        .try_into()
        .expect("Invalid hexadecimal encoded vk hash");

    let Some(pub_inputs_file_name) = args.pub_input_file_name else {
        tracing::error!("Public input file not provided");
        return;
    };
    let public_inputs =
        std::fs::read(&pub_inputs_file_name).expect("to read program public inputs file");

    let provider =
        ProofAggregationServiceProvider::new(args.network, args.rpc_url, args.beacon_url);

    let verification_data = AggregationModeVerificationData::SP1 {
        vk: program_id_key,
        public_inputs,
    };

    let proof_status = match provider
        .check_proof_verification(args.from_block, verification_data)
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
