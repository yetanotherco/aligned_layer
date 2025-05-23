use alloy::hex::hex;
use serde_json::json;
use sp1_sdk::{HashableKey, Prover};
use std::{fs, path::Path};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const SP1_PROGRAM_ELF: &[u8] = include_bytes!("../sp1/elf/sp1_state_transition_program");

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("About to write sp1 program vk hash bytes");
    let client = sp1_sdk::ProverClient::builder().cpu().build();
    let (_pk, vk) = client.setup(SP1_PROGRAM_ELF);
    let sp1_vk_hash = vk.hash_bytes();
    let sp1_vk_hash_hex = hex::encode(sp1_vk_hash);

    let dest_path = Path::new("programs_ids.json");
    let json_data = json!({
        "sp1_vk_hash": format!("0x{}", sp1_vk_hash_hex),
    });

    // Write to the file
    fs::write(dest_path, serde_json::to_string_pretty(&json_data).unwrap()).unwrap();

    info!("Program ids written to {:?}", dest_path);
}
