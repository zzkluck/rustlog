#[macro_use]
extern crate lazy_static;

mod log_parser;
mod evaluator;
mod utils;
mod dataset;
mod cli;

use std::fs::{OpenOptions};
use clap::Parser;
use simplelog::*;

use crate::cli::Cli;
use crate::cli::args::{CliOperation};
use crate::cli::commands::{benchmark_command, parse_command};
use crate::utils::combine_print;

fn main() {
    combine_print("./data/loghub_2k_corrected/HDFS/HDFS_2k.log".as_ref());
}
fn main1() {
    let config = ConfigBuilder::new()
        .set_time_offset_to_local().unwrap()
        .set_time_format_rfc3339()
        .build();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Trace, config.clone(), OpenOptions::new().append(true).open("easy.log").unwrap()),
        ]
    ).unwrap();

    let cli = Cli::parse();
    match cli.subcommand {
        CliOperation::Parse(args) => { parse_command(args); }
        CliOperation::Benchmark(args) => { benchmark_command(args); }
    }
}
