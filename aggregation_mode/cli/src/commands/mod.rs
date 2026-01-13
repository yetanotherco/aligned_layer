use crate::commands::{deposit::SendPaymentArgs, submit::SubmitCommand, verify::VerifyCommand};
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
    #[command(subcommand, name = "verify-on-chain")]
    VerifyOnChain(VerifyCommand),
    /// Send 1 ether to the aggregation mode payment service
    #[command(name = "deposit")]
    Deposit(SendPaymentArgs),
}
