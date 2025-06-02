use l2::l2::L2;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use l2_cmd::utils::load_config;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = load_config();
    let mut l2 = L2::new(config);

    info!("Loading proof from disk to verify on chain");

    let proof_bytes = std::fs::read("./proof.bin").unwrap();
    let proof = bincode::deserialize(&proof_bytes).unwrap();
    l2.update_state_on_chain(proof).await;
}
