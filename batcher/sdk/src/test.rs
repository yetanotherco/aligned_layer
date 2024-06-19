use crate::lib::submit;

fn main() {

    let proof = read_file("batcher/aligned/test_files/plonk_bls12_381/plonk.proof")?;
    
    let vm_program_code = None;
    let verification_key = Some(read_file("batcher/aligned/test_files/plonk_bls12_381/plonk.vk")?);
    let pub_input = Some(read_file("batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub")?);
    
    let url = "http//localhost:8080/ws";
    let (ws_stream, _) = connect_async(url).await?;

    let submit_args = SubmitArgs {
        ws_stream: WebSocketStream::new(),
        verification_data: VerificationData {
            proving_system: ProvingSystemId::GnarkPlonkBls12_381,
            proof: proof,
            pub_input: pub_input,
            verification_key: verification_key,
            vm_program_code: vm_program_code,
            proof_generator_addr: Address::new(),
        },
    };
    submit(submit_args);
}
