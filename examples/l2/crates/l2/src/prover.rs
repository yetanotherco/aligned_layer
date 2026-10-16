use sp1_sdk::{Elf, ProveRequest, Prover, ProvingKey, SP1VerifyingKey};
use types::Transfer;

use crate::db::DB;

pub const PROGRAM_ELF: Elf = Elf::Static(include_bytes!(
    "../zkvm_programs/sp1/elf/sp1_state_transition_program"
));

pub async fn prove_state_transition(
    db: &DB,
    transfers: Vec<Transfer>,
) -> (sp1_sdk::SP1ProofWithPublicValues, sp1_sdk::SP1VerifyingKey) {
    let mut stdin = sp1_sdk::SP1Stdin::new();
    let program_input = sp1_state_transition_program::ProgramInput {
        transfers,
        user_states: db.user_states.clone(),
    };
    stdin.write(&program_input);

    let prover = sp1_sdk::ProverClient::from_env().await;
    let pk = prover
        .setup(PROGRAM_ELF)
        .await
        .expect("Prover setup to run well");
    let vk = pk.verifying_key().clone();
    let proof = prover
        .prove(&pk, stdin)
        .compressed()
        .await
        .expect("Prover to run well");

    (proof, vk)
}

pub async fn vk_from_elf(elf: Elf) -> SP1VerifyingKey {
    let prover = sp1_sdk::ProverClient::builder().light().build().await;
    let pk = prover.setup(elf).await.expect("Prover setup to run well");
    pk.verifying_key().clone()
}
