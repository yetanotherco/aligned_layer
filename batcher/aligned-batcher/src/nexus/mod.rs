use log::{debug, warn};
use nexus_prover::{self, types::{IVCProof, SeqPP}};
use zstd::stream::Decoder;
use ark_serialize::CanonicalDeserialize;

pub fn verify_nexus_proof(proof: &[u8], params: &[u8]) -> bool {
    debug!("Verifying Nexus proof");
    let mut params_dec = Decoder::new(params).unwrap();
    if let Ok(proof) = IVCProof::deserialize_compressed(proof) {
        if let Ok(params) = SeqPP::deserialize_compressed(&mut params_dec) {
            let res = proof.verify(&params, proof.step_num() as usize).is_ok();
            debug!("Nexus proof is valid: {}", res);
            if res {
                return true;
            }
        }
    }
    warn!("Failed to decode Nexus proof");

    false
}
