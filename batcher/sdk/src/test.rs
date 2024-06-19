use crate::lib::submit;

// pub struct SubmitArgs {
//     ws_stream: WebSocketStream<S>,
//     verification_data: VerificationData,
// }

// pub struct VerificationData {
//     pub proving_system: ProvingSystemId,
//     pub proof: Vec<u8>,
//     pub pub_input: Option<Vec<u8>>,
//     pub verification_key: Option<Vec<u8>>,
//     pub vm_program_code: Option<Vec<u8>>,
//     pub proof_generator_addr: Address,
// }

// pub fn submit(submit_args: SubmitArgs) -> Result<(), errors::BatcherClientError> {


//     aligned submit --proving_system GnarkPlonkBls12_381 \
// --proof batcher/aligned/test_files/plonk_bls12_381/plonk.proof \
// --public_input batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub \
// --vk batcher/aligned/test_files/plonk_bls12_381/plonk.vk \
// --repetitions 4

fn main() {

    let proof = read_file("batcher/aligned/test_files/plonk_bls12_381/plonk.proof")?;
    
    let vm_program_code = None;
    let verification_key = Some(read_file("batcher/aligned/test_files/plonk_bls12_381/plonk.vk")?);
    let pub_input = Some(read_file("batcher/aligned/test_files/plonk_bls12_381/plonk_pub_input.pub")?);
    let proof_generator_addr = Address::from_str(&args.proof_generator_addr).unwrap();


    let submit_args = SubmitArgs {
        ws_stream: WebSocketStream::new(),
        verification_data: VerificationData {
            proving_system: ProvingSystemId::GnarkPlonkBls12_381,
            proof: proof,
            pub_input: pub_input,
            verification_key: verification_key,
            vm_program_code: vm_program_code,
            proof_generator_addr: proof_generator_addr,
            // proof_generator_addr: Address::new(),
        },
    };
    submit();
}
