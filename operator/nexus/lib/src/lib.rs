use ark_serialize::CanonicalDeserialize;
use log::{info, warn};
use nexus_core::{
    self,
    prover::nova::types::{IVCProof, SeqPP},
};
use std::slice;
use zstd::stream::Decoder;

#[no_mangle]
pub extern "C" fn verify_nexus_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    params_bytes: *const u8,
    params_len: u32,
) -> bool {
    if proof_bytes.is_null() || params_bytes.is_null() {
        return false;
    }

    info!("Verifying Nexus Proof");
    let proof_bytes = unsafe { slice::from_raw_parts(proof_bytes, proof_len as usize) };

    let params_bytes = unsafe { slice::from_raw_parts(params_bytes, params_len as usize) };

    let params_dec = Decoder::new(params_bytes).unwrap();

    let Ok(proof) = IVCProof::deserialize_compressed(proof_bytes) else {
        warn!("Failed to deserialize Nexus Proof");
        return false;
    };

    let Ok(params) = SeqPP::deserialize_compressed(params_dec) else {
        warn!("Failed to deserialize Nexus Parameters");
        return false;
    };

    proof.verify(&params).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] = include_bytes!("../../../../scripts/test_files/nexus/nexus-proof");

    // NOTE: These are generate after calling `cargo nexus prove` and stored in
    // `../../scripts/test_files/nexus/target/nexus-cache/nexus-public-nova-seq-16.zst`
    const PARAMS: &[u8] =
        include_bytes!("../../../../scripts/test_files/nexus/nexus-public-nova-seq-16.zst");

    #[test]
    fn verify_nexus_proof_works() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();

        let result = verify_nexus_proof_ffi(
            proof_bytes,
            PROOF.len() as u32,
            params_bytes,
            PARAMS.len() as u32,
        );
        assert!(result)
    }

    #[test]
    fn verify_nexus_aborts_with_bad_proof() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();

        let result = verify_nexus_proof_ffi(
            proof_bytes,
            (PROOF.len() - 1) as u32,
            params_bytes,
            PARAMS.len() as u32,
        );
        assert!(!result)
    }
}
