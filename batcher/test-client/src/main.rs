use std::env;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

use batcher::types::{ProvingSystemId, VerificationData, get_proving_system_from_str};
use log::error;

#[tokio::main]
async fn main() {
    let connect_addr = env::args() // ex: ws://localhost:8080
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));
    
    let proving_system_flag = env::args() // ex: SP1
        .nth(2)
        .unwrap_or_else(|| panic!("this program requires at least two arguments"));

    let file_name = env::args() // ex: ./test_files/sp1/sp1_fibonacci
        .nth(3)
        .unwrap_or_else(|| panic!("this program requires at least three arguments"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
    let (mut ws_write, ws_read) = ws_stream.split();

    let proving_system = get_proving_system_from_str(&proving_system_flag);

    // Read proof file
    let proof =
        std::fs::read(file_name.clone() + ".proof").expect("Failed to read .proof file");

    // Read public input file
    let mut public_input: Option<Vec<u8>> = None;
    if let Ok(data) = std::fs::read(file_name.clone() + ".pub") {
        public_input = Some(data);
    } else {
        println!("Warning: File {}.pub does not exist, continuing with no public_input", file_name);
    }

    let verification_key: Option<Vec<u8>>;
    let vm_program_code: Option<Vec<u8>>;
    // Read verification key / vm_program_code
    if proving_system == ProvingSystemId::SP1 {
        verification_key = None;
            // TODO check -elf file name
        vm_program_code = Some(std::fs::read(file_name + "-elf").expect("Failed to read .vm file")); //"./test_files/sp1/riscv32im-succinct-zkvm-elf") //previous
    } else {
        verification_key = Some(std::fs::read(file_name + ".vk").expect("Failed to read .vk file"));
        vm_program_code = None;
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
