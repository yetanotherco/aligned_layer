use agg_mode_sdk::{gateway::provider::AggregationModeGatewayProvider, types::Network};
use clap::{command, Args, Subcommand};
use sp1_sdk::{SP1ProofWithPublicValues, SP1VerifyingKey};
use std::path::PathBuf;

use crate::commands::helpers::{parse_network, PrivateKeyType};

#[derive(Debug, Subcommand)]
pub enum SubmitCommand {
    #[command(name = "sp1")]
    SP1(SubmitSP1Args),
}

#[derive(Debug, Clone, Args)]
pub struct SubmitSP1Args {
    #[arg(short = 'p', long = "proof")]
    proof_path: PathBuf,
    #[arg(long = "vk")]
    verifying_key_path: PathBuf,
    #[command(flatten)]
    private_key_type: PrivateKeyType,
    #[arg(short = 'n', long = "network", default_value = "devnet", value_parser = parse_network)]
    network: Network,
}

pub async fn run(args: SubmitSP1Args) {
    tracing::info!("Submitting SP1 proof to {:?} ", args.network);

    let proof = load_proof(&args.proof_path).expect("Valid proof");
    let vk = load_vk(&args.verifying_key_path).expect("Valid vk");

    let signer = match args.private_key_type.into_signer() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("{e}");
            return;
        }
    };

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

fn load_proof(path: &PathBuf) -> Result<SP1ProofWithPublicValues, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read proof from {}: {e}", path.display()))?;

    bincode::deserialize(&bytes)
        .map_err(|e| format!("failed to deserialize proof {}: {e}", path.display()))
}

fn load_vk(path: &PathBuf) -> Result<SP1VerifyingKey, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read verifying key from {}: {e}", path.display()))?;

    bincode::deserialize(&bytes).map_err(|e| {
        format!(
            "failed to deserialize verifying key {}: {e}",
            path.display()
        )
    })
}
