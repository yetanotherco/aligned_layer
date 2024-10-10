use log::info;

#[link(name = "nexus_verifier", kind = "static")]
extern "C" {
    fn verify_nexus_proof_ffi(
        proof_bytes: *const u8,
        proof_len: u32,
        params_bytes: *const u8,
        params_len: u32,
    ) -> bool;
}

pub fn verify_nexus_proof(proof: &[u8], params: &[u8]) -> bool {
    info!("Verifying Nexus Proof");
    unsafe {
        verify_nexus_proof_ffi(
            proof.as_ptr(),
            proof.len() as u32,
            params.as_ptr(),
            params.len() as u32,
        )
    }
}
