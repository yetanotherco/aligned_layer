use log::{debug, warn};
use mina_bridge_core::proof::state_proof::{MinaStateProof, MinaStatePubInputs};

pub fn verify_proof_integrity(proof: &[u8], public_input: &[u8]) -> bool {
    debug!("Deserializing Mina Proof of State");
    if let Err(err) = bincode::deserialize::<MinaStateProof>(proof) {
        warn!("Couldn't deserialize Mina Proof of State: {err}");
        return false;
    }

    debug!("Deserializing Mina Proof of State public inputs");
    if let Err(err) = bincode::deserialize::<MinaStatePubInputs>(public_input) {
        warn!("Couldn't deserialize Mina Proof of State public inputs: {err}");
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::verify_proof_integrity;

    const PROTOCOL_STATE_PROOF_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.proof");
    const PROTOCOL_STATE_PUB_BYTES: &[u8] =
        include_bytes!("../../../../batcher/aligned/test_files/mina/protocol_state.pub");

    #[test]
    fn verify_proof_integrity_does_not_fail() {
        assert!(verify_proof_integrity(
            PROTOCOL_STATE_PROOF_BYTES,
            PROTOCOL_STATE_PUB_BYTES,
        ));
    }
}
