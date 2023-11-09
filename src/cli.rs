pub mod args;
pub mod commands;

use clap::{Parser};
use crate::cli::args::{CliOperation};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: CliOperation
}

