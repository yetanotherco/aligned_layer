pub mod errors;
mod eth;
pub mod models;
mod sdk;
pub use sdk::{submit, verify_proof_onchain};
