// Here, define the modules and functions to expose.
// Write documentation comments for your public API using '///'.
// Generate HTML documentation with 'cargo doc'.

// Provide example code in the examples directory to demonstrate usage.

pub struct SubmitArgs {
    ws_stream: WebSocketStream<S>,
    verification_data: VerificationData,
}

pub struct VerificationData {
    pub proving_system: ProvingSystemId,
    pub proof: Vec<u8>,
    pub pub_input: Option<Vec<u8>>,
    pub verification_key: Option<Vec<u8>>,
    pub vm_program_code: Option<Vec<u8>>,
    pub proof_generator_addr: Address,
}

/// BatchInclusionData is the information that is retrieved to the clients once
/// the verification data sent by them has been processed by Aligned.
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchInclusionData {
    pub verification_data_commitment: VerificationDataCommitment,
    pub batch_merkle_root: [u8; 32],
    pub batch_inclusion_proof: Proof<[u8; 32]>,
    pub verification_data_batch_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerificationDataCommitment {
    pub proof_commitment: [u8; 32],
    pub pub_input_commitment: [u8; 32],
    // This could be either the VM code (ELF, bytecode) or the verification key
    // depending on the proving system.
    pub proving_system_aux_data_commitment: [u8; 32],
    pub proof_generator_addr: [u8; 20],
}

/// Submits proof to batcher
/// 
/// Example
/// 
pub fn submit(submit_args: SubmitArgs) -> Result<(), errors::BatcherClientError> {

    let (mut ws_write, ws_read) = submit_args.ws_stream.split();
    
    // The sent verification data will be stored here so that we can calculate
    // their commitments later.
    let mut sent_verification_data: Vec<VerificationData> = Vec::new(); // todo ?
    
    let json_data = serde_json::to_string(&submit_args.verification_data)?; // todo check if clone

    ws_write.send(Message::Text(json_data.to_string())).await?;
    sent_verification_data.push(submit_args.verification_data.clone()); // todo check if clone
    info!("Message sent...");

    
    // This vector is reversed so that when responses are received, the commitments corresponding
    // to that response can simply be popped of this vector.
    let mut verification_data_commitments_rev: Vec<VerificationDataCommitment> =
        sent_verification_data
            .into_iter()
            .map(|vd| vd.into())
            .rev()
            .collect();
    
    receive(
        ws_read
    )
    .await?;
    Ok(())
}


async fn receive(
    ws_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<(), BatcherClientError> {
    // Responses are filtered to only admit binary or close messages.
    let mut response_stream =
        ws_read.try_filter(|msg| future::ready(msg.is_binary() || msg.is_close()));

    let Some(Ok(msg)) = response_stream.next().await;
    if let Message::Close(close_frame) = msg {
        if let Some(close_msg) = close_frame {
            error!("Connection was closed before receiving response message. Reason: {}. Try submitting your proof again", close_msg.to_owned());
            return Ok(()); // TODO return error, from batcher error lib
            // create/import batcher error codes:
            // https://github.com/yetanotherco/aligned_layer/blob/main/batcher/aligned/src/errors.rs

        }
        error!("Connection was closed before receiving response message. Try submitting your proof again");
        return Ok(()); // TODO return error, from batcher error lib
    } else {
        let data = msg.into_data();
        match serde_json::from_slice::<BatchInclusionData>(&data) {
            Ok(batch_inclusion_data) => {
                info!("Batcher response received: {}", batch_inclusion_data);

                let batch_merkle_root = hex::encode(batch_inclusion_data.batch_merkle_root);

                // file.write_all(data.as_slice()).unwrap(); //TODO return this

            }
            Err(e) => {
                error!("Error while deserializing batcher response: {}", e);
            }
        }
    }

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
