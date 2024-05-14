//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use std::env;

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

    let (mut write, read) = ws_stream.split();
    let json_data = r#"{
        "proving_system": "GnarkPlonkBls12_381",
        "proof": [1, 2, 3, 4],
        "public_input": [1, 2, 3, 4],
        "verification_key": [5, 6, 7, 8],
        "quorum_numbers": [0],
        "quorum_threshold_percentages": [100],
        "fee": 123
    }"#;

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

// fn main() {
//
//     let task: batcher::Task = serde_json::from_str(json_data).unwrap();
//     println!("{:?}", task);
// }
