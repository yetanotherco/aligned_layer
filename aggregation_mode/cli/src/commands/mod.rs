use crate::commands::{submit::SubmitCommand, verify::VerifyOnChainArgs};
use clap::{Parser, Subcommand};

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
    #[command(name = "verify-on-chain")]
    VerifyOnChain(VerifyOnChainArgs),
}
