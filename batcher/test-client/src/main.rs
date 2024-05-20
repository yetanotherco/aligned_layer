use std::env;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

use batcher::types::{ProvingSystemId, VerificationData};
use log::error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    // Check args length
    if args.len() < 3 {
        panic!("Usage: {} <ws://addr> <sp1|plonk_bls12_381|plonk_bn254|groth16_bn254>", args[0]);
    }

    let connect_addr = args[1].clone();
    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");


    let task = match args[2].as_str() {
        "sp1" => {
            let proof =
                std::fs::read("./test_files/sp1/sp1_fibonacci.proof")
                    .expect("Failed to read proof file");

            let vm_program_code = std::fs::read("./test_files/sp1/riscv32im-succinct-zkvm-elf")
                .expect("Failed to read public input file");

            VerificationData {
                proving_system: ProvingSystemId::SP1,
                proof,
                public_input: None,
                verification_key: None,
                vm_program_code: Some(vm_program_code),
            }
        },
        "plonk_bls12_381" => {
            let proof =
                std::fs::read("./test_files/plonk_bls12_381/plonk.proof")
                    .expect("Failed to read proof file");

            let public_input =
                std::fs::read("./test_files/plonk_bls12_381/plonk_pub_input.pub")
                    .expect("Failed to read public input file");

            let verification_key =
                std::fs::read("./test_files/plonk_bls12_381/plonk.vk")
                    .expect("Failed to read verification key file");

            VerificationData {
                proving_system: ProvingSystemId::GnarkPlonkBls12_381,
                proof,
                public_input: Some(public_input),
                verification_key: Some(verification_key),
                vm_program_code: None,
            }
        },
        "plonk_bn254" => {
            let proof =
                std::fs::read("./test_files/plonk_bn254/plonk.proof")
                    .expect("Failed to read proof file");

            let public_input =
                std::fs::read("./test_files/plonk_bn254/plonk_pub_input.pub")
                    .expect("Failed to read public input file");

            let verification_key =
                std::fs::read("./test_files/plonk_bn254/plonk.vk")
                    .expect("Failed to read verification key file");

            VerificationData {
                proving_system: ProvingSystemId::GnarkPlonkBn254,
                proof,
                public_input: Some(public_input),
                verification_key: Some(verification_key),
                vm_program_code: None,
            }
        }
        "groth16_bn254" => {
            let proof =
                std::fs::read("./test_files/groth16_bn254/plonk.proof")
                    .expect("Failed to read proof file");

            let public_input =
                std::fs::read("./test_files/groth16_bn254/plonk_pub_input.pub")
                    .expect("Failed to read public input file");

            let verification_key =
                std::fs::read("./test_files/groth16_bn254/plonk.vk")
                    .expect("Failed to read verification key file");

            VerificationData {
                proving_system: ProvingSystemId::Groth16Bn254,
                proof,
                public_input: Some(public_input),
                verification_key: Some(verification_key),
                vm_program_code: None,
            }
        },
        _ => {
            panic!("Unsupported proving system: {}", args[2]);
        }
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
    // let bytes = bytes::Bytes::from(proof);
    let bucket_name = "storage.alignedlayer.com";
    let key = "10mb_file";

    // start timer
    let start = std::time::Instant::now();

    println!("Uploading object to S3");
    let result = batcher::s3::upload_object(&client, bucket_name, proof, key).await;
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
