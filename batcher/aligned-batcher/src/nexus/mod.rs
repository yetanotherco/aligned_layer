use log::{debug, warn};
use nexus_prover::{self, verify_compressed, types::{ComProof, ComPP, SpartanKey}};

pub fn verify_sp1_proof(proof: &[u8], params: &[u8], key: &[u8]) -> bool {
    debug!("Verifying Nexus proof");
    if let Ok(proof) = ComProof::deserialize_uncompressed(proof) {
        if let Ok(params) = ComPP::deserialize_uncompressed(params) {
            if let Ok(key) = SpartanKey::deserialize_uncompressed(key) {
                let res = verify_compressed(&key, &params, &proof).is_ok();
                debug!("Nexus proof is valid: {}", res);
                if res {
                    return true;
                }
            }
        }
    }

    warn!("Failed to decode Nexus proof");

    false
}
