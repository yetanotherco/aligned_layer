use std::time::Duration;
use std::path::PathBuf;

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

use batcher::types::{ProvingSystemId, VerificationData, get_proving_system_from_str};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(name = "Batcher address", long = "conn", default_value = "ws://localhost:8080")]
    connect_addr: String,

    #[arg(name = "Proving System", long = "proving_system")]
    proving_system_flag: String,

    #[arg(name = "Proof file name", long = "proof")]
    proof_file_name: PathBuf,

    #[arg(name = "Public input file name", long = "public_input", default_value = ".")]
    public_input_file_name: PathBuf,

    #[arg(name = "Verification key file name", long = "vk", default_value = ".")]
    verification_key_file_name: PathBuf,

    #[arg(name = "VM prgram code file name", long = "vm_program", default_value = ".")]
    vm_program_code_file_name: PathBuf,

    #[arg(name = "Loop timer", long = "timer", default_value_t = 0)]
    timer_loop: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = url::Url::parse(&args.connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut ws_write, ws_read) = ws_stream.split();

    let proving_system = get_proving_system_from_str(&args.proving_system_flag);

    // Read proof file
    let proof =
        std::fs::read(args.proof_file_name).expect("Failed to read .proof file"); //file_name.clone() + ".proof").expect("Failed to read .proof file");

    // Read public input file
    let mut public_input: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.public_input_file_name) {
        public_input = Some(data);
    } else {
        println!("Warning: No Public Input file, continuing with no public_input");
    }

    let mut verification_key: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.verification_key_file_name) {
        verification_key = Some(data);
    } else {
        println!("Warning: no Verification Key File, continuing with no VK File");
    }

    let mut vm_program_code: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(args.vm_program_code_file_name) {
        vm_program_code = Some(data);
    } else {
        println!("Warning: no VM Program Code File, continuing with no VM Program Code");
    }

    let task = VerificationData {
        proving_system,
        proof,
        public_input,
        verification_key,
        vm_program_code,
    };

    let json_data = serde_json::to_string(&task).expect("Failed to serialize task");
    ws_write
        .send(tungstenite::Message::Text(json_data.to_string()))
        .await
        .unwrap();

    ws_read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    })
    .await;
}


// #[allow(unused)]
// async fn upload_proof_to_s3() -> Option<Duration> {
//     let proof =
//         std::fs::read("./test_files/sp1/sp1_fibonacci.proof")
//             .expect("Failed to read proof file");

//     let client = batcher::s3::create_client().await;
//     let bucket_name = "storage.alignedlayer.com";
//     let key = "10mb_file";

//     // start timer
//     let start = std::time::Instant::now();

//     println!("Uploading object to S3");
//     let result = batcher::s3::upload_object(&client, bucket_name, proof, key).await;
//     match result {
//         Ok(_) => {
//             println!("Uploaded object to S3");
//             // end timer
//             let elapsed = start.elapsed();
//             println!("Time elapsed: {:?}", elapsed);

//             Some(elapsed)
//         },
//         Err(e) => {
//             error!("Failed to upload object to S3: {:?}", e);
//             None
//         },
//     }
// }
