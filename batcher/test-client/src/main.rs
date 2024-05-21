use std::env;

use batcher::types::{ProvingSystemId, VerificationData};
use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

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
    let vm_program_code = std::fs::read("./test_files/sp1/riscv32im-succinct-zkvm-elf")
        .expect("Failed to read public input file");

    let task = VerificationData {
        proving_system: ProvingSystemId::SP1,
        proof,
        public_input: None,
        verification_key: None,
        vm_program_code: Some(vm_program_code),
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
