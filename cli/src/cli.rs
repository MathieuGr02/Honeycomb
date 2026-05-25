use clap::{Parser, Subcommand};

use crate::{build::BuildArguments, run::RunArguments};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Debug, Subcommand)]
pub enum CLICommands {
    Build(BuildArguments),
    Run(RunArguments),
}
