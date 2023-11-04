mod log_parser;
mod easylog;
mod evaluator;
mod data_model;
mod dataset;

use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use crate::dataset::{AndroidDataset, LogDataset};
use crate::easylog::EasyLog;
use crate::log_parser::LogParser;
use crate::evaluator::get_accuracy;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    pub subcommand: ParseArgs
}

#[derive(Subcommand, Debug)]
pub enum  ParseArgs {
    EasyLog(EasyLogArgs),
}

#[derive(Args, Debug)]
pub struct EasyLogArgs {
    #[arg(short, long, value_name = "LOG_PATH")]
    log_path: PathBuf,
    #[arg(short, long, value_name = "CONFIG_PATH")]
    config_path: PathBuf,
    #[arg(short, long, value_name = "STRUCTURED_PATH")]
    structured_path: PathBuf
}


fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.subcommand {
        ParseArgs::EasyLog(args) => {
            let easylog = EasyLog::new(&args.config_path);
            let mut pl = easylog.parse_from_file(&args.log_path);
            println!("{} templates found.", pl.templates.len());
            let dataset = AndroidDataset::from_file(&args.structured_path);
            println!("{:?}", get_accuracy(dataset.iter_event_id(), pl.parsed_list))
        }
    }
}