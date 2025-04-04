use log::{debug, error, warn};
use sp1_sdk::{EnvProver, ProverClient};
use std::sync::OnceLock;

static SP1_PROVER_CLIENT: OnceLock<EnvProver> = OnceLock::new();

pub fn verify_sp1_proof(proof: &[u8], elf: &[u8]) -> bool {
    if proof.is_empty() || elf.is_empty() {
        error!("SP1 Input buffers zero size");
        return false;
    }

    debug!("Verifying SP1 proof");
    let prover_client = SP1_PROVER_CLIENT.get_or_init(ProverClient::from_env);

    let (_pk, vk) = prover_client.setup(elf);
    if let Ok(proof) = bincode::deserialize(proof) {
        //client.verify(&proof, &vk).expect("failed to verify proof");
        let res = prover_client.verify(&proof, &vk).is_ok();
        debug!("SP1 proof is valid: {}", res);
        if res {
            return true;
        }
    }

    warn!("Failed to decode SP1 proof");

    false
}
