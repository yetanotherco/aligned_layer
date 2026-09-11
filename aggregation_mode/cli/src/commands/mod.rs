use crate::commands::{deposit::SendPaymentArgs, submit::SubmitCommand, verify::VerifyOnChainArgs};
use clap::{Parser, Subcommand};

pub mod deposit;
mod helpers;
pub mod submit;
pub mod verify;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Submit(SubmitCommand),
    /// Check whether a proof has been verified on AlignedProofAggregationService contract
    #[command(name = "verify-on-chain")]
    VerifyOnChain(VerifyOnChainArgs),
    /// Send 1 ether to the aggregation mode payment service
    #[command(name = "deposit")]
    Deposit(SendPaymentArgs),
}
