use proof_aggregator::backend::{Config, ProofAggregator};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // TODO read proof aggregator yaml config file
    let config = Config {
        private_key: "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".into(),
        rpc_url: "http://localhost:8545".into(),
    };
    let mut proof_aggregator = ProofAggregator::new(config);

    // TODO read proofs from fs
    // proof_aggregator.add_proof(proof)

    proof_aggregator.start().await;
}
