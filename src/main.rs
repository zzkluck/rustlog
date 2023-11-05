#[macro_use]
extern crate lazy_static;

mod log_parser;
mod evaluator;
mod utils;
mod dataset;
mod cli;

use clap::Parser;
use crate::cli::Cli;
use crate::cli::args::{CliOperation};
use crate::cli::commands::{benchmark_command, parse_command};
use crate::dataset::LogDataset;
use crate::log_parser::LogParser;


fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.subcommand {
        CliOperation::Parse(args) => { parse_command(args); }
        CliOperation::Benchmark(args) => { benchmark_command(args); }
    }
}
