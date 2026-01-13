use agg_mode_sdk::{gateway::provider::AggregationModeGatewayProvider, types::Network};
use alloy::signers::local::LocalSigner;
use clap::{command, Args, Subcommand};
use sp1_sdk::{SP1ProofWithPublicValues, SP1VerifyingKey};
use std::{path::PathBuf, str::FromStr};

use crate::commands::helpers::parse_network;

#[derive(Debug, Subcommand)]
pub enum SubmitCommand {
    #[command(name = "sp1")]
    SP1(SubmitSP1Args),
    #[command(name = "zisk")]
    Zisk(SubmitZiskArgs),
}

#[derive(Debug, Clone, Args)]
pub struct SubmitSP1Args {
    #[arg(short = 'p', long = "proof")]
    proof_path: PathBuf,
    #[arg(long = "vk")]
    verifying_key_path: PathBuf,
    #[arg(long = "private-key")]
    private_key: String,
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
}

#[derive(Debug, Clone, Args)]
pub struct SubmitZiskArgs {
    #[arg(short = 'p', long = "proof")]
    proof_path: PathBuf,
    #[arg(long = "private-key")]
    private_key: String,
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
}

pub async fn run_sp1(args: SubmitSP1Args) {
    tracing::info!("Submitting SP1 proof to {:?} ", args.network);

    let proof = load_sp1_proof(&args.proof_path).expect("Valid proof");
    let vk = load_sp1_vk(&args.verifying_key_path).expect("Valid vk");

    let signer =
        LocalSigner::from_str(args.private_key.trim()).expect("failed to parse private key: {e}");

    let provider = AggregationModeGatewayProvider::new_with_signer(args.network.clone(), signer)
        .expect("failed to initialize gateway client: {e:?}");

    let response = provider
        .submit_sp1_proof(&proof, &vk)
        .await
        .expect("failed to submit proof: {e:?}");

    tracing::info!(
        "Proof submitted successfully. Task ID: {}",
        response.data.task_id
    );
}

pub async fn run_zisk(args: SubmitZiskArgs) {
    tracing::info!("Submitting Zisk proof to {:?} ", args.network);

    let proof = std::fs::read(&args.proof_path).expect(&format!(
        "failed to read proof from {}",
        args.proof_path.display()
    ));

    let signer =
        LocalSigner::from_str(args.private_key.trim()).expect("failed to parse private key: {e}");

    let provider = AggregationModeGatewayProvider::new_with_signer(args.network.clone(), signer)
        .expect("failed to initialize gateway client: {e:?}");

    let response = provider
        .submit_zisk_proof(&proof)
        .await
        .expect("failed to submit proof: {e:?}");

    tracing::info!(
        "Proof submitted successfully. Task ID: {}",
        response.data.task_id
    );
}

fn load_sp1_proof(path: &PathBuf) -> Result<SP1ProofWithPublicValues, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read proof from {}: {e}", path.display()))?;

    bincode::deserialize(&bytes)
        .map_err(|e| format!("failed to deserialize proof {}: {e}", path.display()))
}

fn load_sp1_vk(path: &PathBuf) -> Result<SP1VerifyingKey, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read verifying key from {}: {e}", path.display()))?;

    bincode::deserialize(&bytes).map_err(|e| {
        format!(
            "failed to deserialize verifying key {}: {e}",
            path.display()
        )
    })
}
