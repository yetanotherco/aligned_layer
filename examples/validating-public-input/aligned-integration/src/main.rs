use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::core::errors::SubmitError;
use aligned_sdk::core::types::Chain::Holesky;
use aligned_sdk::core::types::{AlignedVerificationData, ProvingSystemId, VerificationData};
use aligned_sdk::sdk::submit_and_wait;
use ethers::signers::LocalWallet;
use ethers::types::Address;
use ethers::utils::hex;
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), SubmitError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let proof = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.proof",
    ))
    .unwrap_or_default();
    let pub_input = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci.pub",
    ));
    let image_id = read_file(PathBuf::from(
        "../risc_zero/fibonacci_proof_generator/risc_zero_fibonacci_id.bin",
    ));

    let proof_generator_addr =
        Address::from_str("0x66f9664f97F2b50F62D13eA064982f936dE76657").unwrap();

    let verification_data = VerificationData {
        proving_system: ProvingSystemId::Risc0,
        proof,
        pub_input,
        verification_key: None,
        vm_program_code: image_id,
        proof_generator_addr,
    };

    // Set to the 9th address of anvil that doesn't pay for the proof submission
    let wallet =
        LocalWallet::from_str("2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6")
            .expect("Failed to create wallet");

    info!("Submitting Fibonacci proof to Aligned and waiting for verification...");
    let aligned_verification_data = submit_and_wait(
        "wss://batcher.alignedlayer.com",
        "https://ethereum-holesky-rpc.publicnode.com",
        Holesky,
        &verification_data,
        wallet,
    )
    .await?;

    let batch_inclusion_data_directory_path = PathBuf::from("./batch_inclusion_data");

    info!("Saving verification data to {:?}", batch_inclusion_data_directory_path);
    if let Some(aligned_verification_data) = aligned_verification_data {
        save_response(
            batch_inclusion_data_directory_path,
            &aligned_verification_data,
        )?;
    } else {
        return Err(SubmitError::EmptyVerificationDataList);
    }

    Ok(())
}

fn read_file(file_name: PathBuf) -> Option<Vec<u8>> {
    std::fs::read(file_name).ok()
}

fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), SubmitError> {

    std::fs::create_dir_all(&batch_inclusion_data_directory_path).map_err(|e| {
        SubmitError::IoError(batch_inclusion_data_directory_path.clone(), e)
    })?;
    
    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".json";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let data = serde_json::to_vec(&aligned_verification_data)?;

    let mut file = File::create(&batch_inclusion_data_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;

    file.write_all(data.as_slice())
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;

    Ok(())
}
