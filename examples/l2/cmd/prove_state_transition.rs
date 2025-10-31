use std::{fs::File, io::Write};

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

    let proof = l2.prove_state_transition_and_send_proof_to_aligned().await;
    info!("Serializing and saving proof on disk to be verified later on chain");

    let proof_bytes = bincode::serialize(&proof).unwrap();
    let mut file = File::create("./proof.bin").expect("Unable to create file");
    file.write(&proof_bytes).unwrap();
    info!("Proof stored in disk, in the next 24hs it will be aggregated and verified and aligned, so you should retrieve its status later");
}
