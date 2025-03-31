use std::{env, fs};

use proof_aggregator::{
    backend::{config::Config, ProofAggregator},
    zk::{
        backends::sp1::{vk_from_elf, SP1Proof},
        Proof,
    },
};
use sp1_sdk::SP1ProofWithPublicValues;
use tracing_subscriber::FmtSubscriber;

fn read_config_filepath_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!(
            "You mus provide a config file. Usage: {} <config-file-path>",
            args[0]
        );
    }

    args[1].clone()
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // init proof aggregator
    let config_file_path = read_config_filepath_from_args();
    tracing::info!("Loading config from {}..", config_file_path);
    let config = Config::from_file(&config_file_path).expect("Config is valid");
    let mut proof_aggregator = ProofAggregator::new(config);
    tracing::info!("Config loaded proof aggregator initialized");

    // push some proofs from fs
    for _ in 0..2 {
        let sp1_proof =
            SP1ProofWithPublicValues::load("scripts/test_files/sp1/sp1_fibonacci_4_1_3.proof")
                .expect("loading proof failed");
        let proof_elf =
            fs::read("scripts/test_files/sp1/sp1_fibonacci_4_1_3.elf").expect("elf bytes");
        let proof = Proof::SP1(SP1Proof {
            proof: sp1_proof,
            vk: vk_from_elf(&proof_elf),
        });

        proof_aggregator
            .add_proof(proof, &proof_elf)
            .expect("Proof to be valid");
    }

    // start service
    proof_aggregator.start().await;
}
