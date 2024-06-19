use log::{debug, warn};
use nexus_prover::{self, types::{IVCProof, SeqPP}};
use zstd::stream::Decoder;

pub fn verify_nexus_proof(proof: &[u8], params: &[u8], key: &[u8]) -> bool {
    debug!("Verifying Nexus proof");
    let mut params_dec = Decoder::new(params).unwrap();
    if let Ok(proof) = ComProof::deserialize_uncompressed(proof) {
        if let Ok(params) = ComPP::deserialize_uncompressed(&mut params) {
            let res proof.verify(&params, proof.step_num() as usize).is_ok()
            debug!("Nexus proof is valid: {}", res);
            if res {
                return true;
            }
        }
    }

    warn!("Failed to decode Nexus proof");

    false
}
