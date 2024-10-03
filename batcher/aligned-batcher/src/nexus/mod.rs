use ark_serialize::CanonicalDeserialize;
use log::{info, warn};
use nexus_core::{
    self,
    prover::nova::types::{IVCProof, SeqPP},
};

use zstd::stream::Decoder;

pub fn verify_nexus_proof(proof: &[u8], params: &[u8]) -> bool {
    info!("Verifying Nexus Proof");
    let params_dec = Decoder::new(params).unwrap();

    let Ok(proof) = IVCProof::deserialize_compressed(proof) else {
        warn!("Failed to deserialize Nexus Proof");
        return false;
    };

    let Ok(params) = SeqPP::deserialize_compressed(params_dec) else {
        warn!("Failed to deserialize Nexus Parameters");
        return false;
    };

    proof.verify(&params).is_ok()
}
