use proof_aggregator::backend::ProofAggregator;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // TODO read proof aggregator yaml config file

    let mut proof_aggregator = ProofAggregator::new("http://localhost:8545");

    // TODO read proofs from fs
    // proof_aggregator.add_proof(proof)

    proof_aggregator.start().await;
}
