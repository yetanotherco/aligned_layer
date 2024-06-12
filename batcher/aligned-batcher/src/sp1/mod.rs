use lazy_static::lazy_static;
use log::{info, warn};
use sp1_sdk::ProverClient;

lazy_static! {
    static ref SP1_PROVER_CLIENT: ProverClient = ProverClient::new();
}

pub fn verify_sp1_proof(proof: &[u8], elf: &[u8]) -> bool {
    info!("Client Handler: Verifying SP1 proof");
    let (_pk, vk) = SP1_PROVER_CLIENT.setup(elf);
    if let Ok(proof) = bincode::deserialize(proof) {
        let res = SP1_PROVER_CLIENT.verify_compressed(&proof, &vk).is_ok();
        info!("Client Handler: SP1 proof is valid: {}", res);
        if res {
            return true;
        }
    }

    warn!("Client Handler: Failed to decode SP1 proof");

    false
}
