use alloy::hex::hex;
use proof_aggregator::aggregators::{
    risc0_aggregator::RISC0_AGGREGATOR_PROGRAM_ID_BYTES, sp1_aggregator,
};
use serde_json::json;
use sp1_sdk::HashableKey;
use std::{env, fs, path::Path};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

const SP1_PROGRAM_ELF: &[u8] =
    include_bytes!("../aggregation_programs/sp1/elf/sp1_aggregator_program");

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("About to write sp1 programs vk hash bytes + risc0 programs image id bytes");
    let sp1_vk_hash = sp1_aggregator::vk_from_elf(SP1_PROGRAM_ELF).bytes32_raw();
    let risc0_image_id_bytes = RISC0_AGGREGATOR_PROGRAM_ID_BYTES;

    let sp1_vk_hash_hex = hex::encode(sp1_vk_hash);
    let risc0_image_id_hex = hex::encode(risc0_image_id_bytes);

    let dest_path = Path::new("programs_ids.json");

    let json_data = json!({
        "sp1_vk_hash": format!("0x{}", sp1_vk_hash_hex),
        "risc0_image_id": format!("0x{}", risc0_image_id_hex),
    });

    // Write to the file
    fs::write(dest_path, serde_json::to_string_pretty(&json_data).unwrap()).unwrap();

    info!("Program ids written to {:?}", dest_path);
}
