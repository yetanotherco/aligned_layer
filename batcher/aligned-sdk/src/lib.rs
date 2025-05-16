pub mod aggregation_layer;
pub mod verification_layer;

/// Common types used across the Aggregation Layer and the Verification Layer AVS
pub mod common;

// Eth and Communication modules should be outside the SDK in the future
/// Communication module is mostly for internal use
/// It contains code for communication protocols
pub mod communication;
/// Eth module is mostly for internal use
pub mod eth;

pub(crate) mod beacon;
