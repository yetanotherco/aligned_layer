use primitive_types::H160;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use types::{Transfer, UserState};

#[derive(Deserialize, Serialize)]
pub struct ProgramInput {
    pub user_states: BTreeMap<H160, UserState>,
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize)]
pub struct ProgramOutput {
    pub initial_state_merkle_root: [u8; 32],
    pub post_state_merkle_root: [u8; 32],
}
