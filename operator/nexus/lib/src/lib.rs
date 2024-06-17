use std::slice;
use nexus_prover::{self, verify_compressed, types::{ComProof, ComPP, SpartanKey}};
use ark_serialize::CanonicalDeserialize;

#[no_mangle]
pub extern "C" fn verify_nexus_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    params_bytes: *const u8,
    params_len: u32,
    key_bytes: *const u8,
    key_len: u32,
) -> bool {
    let proof_bytes = unsafe {
        assert!(!proof_bytes.is_null());
        slice::from_raw_parts(proof_bytes, proof_len as usize)
    };

    let params_bytes = unsafe {
        assert!(!params_bytes.is_null());
        slice::from_raw_parts(params_bytes, params_len as usize)
    };

    let key_bytes = unsafe {
        assert!(!key_bytes.is_null());
        slice::from_raw_parts(key_bytes, key_len as usize)
    };

    if let Ok(proof) = ComProof::deserialize_uncompressed(&*proof_bytes) {
        if let Ok(params) = ComPP::deserialize_uncompressed(&*params_bytes) {
            if let Ok(key) = SpartanKey::deserialize_uncompressed(&*key_bytes) {
                return verify_compressed(&key, &params, &proof).is_ok()
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.proof");
    const PARAMS: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.params");
    const KEY: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.key");

    #[test]
    fn verify_nexus_proof_works() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();
        let key_bytes = KEY.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, PROOF.len() as u32, params_bytes, PARAMS.len() as u32, key_bytes, KEY.len() as u32);
        assert!(result)
    }

    #[test]
    fn verify_nexus_aborts_with_bad_proof() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();
        let key_bytes = KEY.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, (PROOF.len() - 1) as u32, PARAMS.len() as u32, PARAMS.len() as u32, key_bytes, KEY.len() as u32);
        assert!(!result)
    }
}
