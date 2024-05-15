use std::env;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

use batcher::types::{ProvingSystemId, Task};
use log::error;

#[tokio::main]
async fn main() {
    let connect_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    // Read proof file
    let proof =
        std::fs::read("./test_files/sp1/sp1_fibonacci.proof").expect("Failed to read proof file");

    // Read public input file
    let public_input = std::fs::read("./test_files/sp1/riscv32im-succinct-zkvm-elf")
        .expect("Failed to read public input file");

    let task = Task {
        proving_system: ProvingSystemId::SP1,
        proof,
        public_input,
        verification_key: vec![5, 6, 7, 8],
        quorum_numbers: vec![0],
        quorum_threshold_percentages: vec![100],
        fee: 123,
    };

    let (mut write, read) = ws_stream.split();

    let json_data = serde_json::to_string(&task).expect("Failed to serialize task");
    write
        .send(tungstenite::Message::Text(json_data.to_string()))
        .await
        .unwrap();

    read.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    })
    .await;
}

#[allow(unused)]
async fn upload_proof_to_s3() -> Option<Duration> {
    let proof =
        std::fs::read("./test_files/sp1/sp1_fibonacci.proof")
            .expect("Failed to read proof file");

    let client = batcher::s3::create_client().await;
    let bytes = bytes::Bytes::from(proof);
    let bucket_name = "storage.alignedlayer.com";
    let key = "10mb_file";

    // start timer
    let start = std::time::Instant::now();

    println!("Uploading object to S3");
    let result = batcher::s3::upload_object(&client, bucket_name, bytes, key).await;
    match result {
        Ok(_) => {
            println!("Uploaded object to S3");
            // end timer
            let elapsed = start.elapsed();
            println!("Time elapsed: {:?}", elapsed);

            Some(elapsed)
        },
        Err(e) => {
            error!("Failed to upload object to S3: {:?}", e);
            None
        },
    }
}
