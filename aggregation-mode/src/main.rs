use std::fs;

use proof_aggregator::{
    backend::{Config, ProofAggregator},
    zk::{
        backends::sp1::{vk_from_elf, SP1Proof},
        Proof,
    },
};
use sp1_sdk::SP1ProofWithPublicValues;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let proofs_to_push = 2;

    // TODO read proof aggregator yaml config file
    let config = Config {
        private_key: "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".into(),
        rpc_url: "http://localhost:8545".into(),
        max_proofs_in_queue: proofs_to_push,
        submit_proofs_every_secs: 2,
    };
    let mut proof_aggregator = ProofAggregator::new(config);

    for _ in 0..proofs_to_push {
        let sp1_proof =
            SP1ProofWithPublicValues::load("../scripts/test_files/sp1/sp1_fibonacci_4_1_3.proof")
                .expect("loading proof failed");
        let proof_elf =
            fs::read("../scripts/test_files/sp1/sp1_fibonacci_4_1_3.elf").expect("elf bytes");
        let proof = Proof::SP1(SP1Proof {
            proof: sp1_proof,
            vk: vk_from_elf(&proof_elf),
        });

        proof_aggregator
            .add_proof(proof, &proof_elf)
            .expect("Proof to be valid");
    }

    proof_aggregator.start().await;
}
