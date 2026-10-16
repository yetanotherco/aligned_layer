use alloy::hex;
use serde_json::json;
use sp1_sdk::{Elf, HashableKey, Prover, ProvingKey};
use std::{fs, path::Path};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const SP1_PROGRAM_ELF: Elf = Elf::Static(include_bytes!("../sp1/elf/sp1_state_transition_program"));

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("About to write sp1 program vk hash bytes");
    let client = sp1_sdk::ProverClient::builder().light().build().await;
    let pk = client
        .setup(SP1_PROGRAM_ELF)
        .await
        .expect("Prover setup to run well");
    let sp1_vk_hash = pk.verifying_key().hash_bytes();
    let sp1_vk_hash_hex = hex::encode(sp1_vk_hash);

    let dest_path = Path::new("programs_ids.json");
    let json_data = json!({
        "sp1_vk_hash": format!("0x{}", sp1_vk_hash_hex),
    });

    // Write to the file
    fs::write(dest_path, serde_json::to_string_pretty(&json_data).unwrap()).unwrap();

    info!("Program ids written to {:?}", dest_path);
}
