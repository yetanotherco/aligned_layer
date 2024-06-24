pub mod errors;
mod eth;
pub mod models;
mod sdk;
pub use sdk::{get_verification_key_commitment, submit, verify_proof_onchain};
