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
use crate::utils::*;

fn main1() {
    let lines = read_lines_from_file("./data/loghub_2k_corrected/HDFS/HDFS_2k.log".as_ref());
    let re = generate_logformat_regex("<Date> <Time> <Pid> <Level> <Component>: <Content>");
    let df = log_to_dataframe(lines.iter().map(|x| x.as_ref()).collect(), re);
    let content: Vec<&str> =
        df.column("Content").expect("No column in df named 'Content'.")
            .utf8().expect("Fail to convert column to type str.")
            .into_iter()
            .map(|x: Option<&str>| x.unwrap_or("[default null value]"))
            .collect();
    combine_print(content);
}
fn main() {
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
