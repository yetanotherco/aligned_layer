use std::slice;
use nexus_prover::{self, types::{IVCProof, SeqPP}};
use zstd::stream::Decoder;
use ark_serialize::CanonicalDeserialize;

#[no_mangle]
pub extern "C" fn verify_nexus_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    params_bytes: *const u8,
    params_len: u32,
) -> bool {
    let proof_bytes = unsafe {
        assert!(!proof_bytes.is_null());
        slice::from_raw_parts(proof_bytes, proof_len as usize)
    };

    let params_bytes = unsafe {
        assert!(!params_bytes.is_null());
        slice::from_raw_parts(params_bytes, params_len as usize)
    };

    let mut params_dec = Decoder::new(params_bytes).unwrap();

    if let Ok(proof) = IVCProof::deserialize_compressed(&*proof_bytes) {
        if let Ok(params) = SeqPP::deserialize_compressed(&mut params_dec) {
                return proof.verify(&params, proof.step_num() as usize).is_ok()
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus-proof");
    const PARAMS: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/target/nexus-cache/nexus-public-seq-16.zst");

    #[test]
    fn verify_nexus_proof_works() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, PROOF.len() as u32, params_bytes, PARAMS.len() as u32);
        assert!(result)
    }

    #[test]
    fn verify_nexus_aborts_with_bad_proof() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, (PROOF.len() - 1) as u32, params_bytes, PARAMS.len() as u32);
        assert!(!result)
    }
}
