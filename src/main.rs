mod log_parser;
mod evaluator;
mod utils;
mod dataset;
mod cli;

use clap::Parser;
use crate::cli::Cli;
use crate::cli::args::ParseArgs;
use crate::cli::commands::easy_log_command;
use crate::dataset::LogDataset;
use crate::log_parser::LogParser;


fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.subcommand {
        ParseArgs::EasyLog(args) => easy_log_command(args)
    }
}
