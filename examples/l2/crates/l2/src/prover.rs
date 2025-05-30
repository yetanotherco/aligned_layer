use sp1_sdk::{Prover, SP1VerifyingKey};
use types::Transfer;

use crate::db::DB;

pub const PROGRAM_ELF: &[u8] =
    include_bytes!("../zkvm_programs/sp1/elf/sp1_state_transition_program");

pub fn prove_state_transition(
    db: &DB,
    transfers: Vec<Transfer>,
) -> (sp1_sdk::SP1ProofWithPublicValues, sp1_sdk::SP1VerifyingKey) {
    let mut stdin = sp1_sdk::SP1Stdin::new();
    let program_input = sp1_state_transition_program::ProgramInput {
        transfers,
        user_states: db.user_states.clone(),
    };
    stdin.write(&program_input);

    let prover = sp1_sdk::ProverClient::from_env();
    let (pk, vk) = prover.setup(PROGRAM_ELF);
    let proof = prover
        .prove(&pk, &stdin)
        .compressed()
        .run()
        .expect("Prover to run well");

    (proof, vk)
}

pub fn vk_from_elf(elf: &[u8]) -> SP1VerifyingKey {
    let prover = sp1_sdk::ProverClient::builder().cpu().build();
    let (_pk, vk) = prover.setup(elf);
    vk
}
