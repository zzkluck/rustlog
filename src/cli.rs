pub mod args;
pub mod commands;

use clap::{Args, Parser, Subcommand};
use crate::cli::args::ParseArgs;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: ParseArgs
}

