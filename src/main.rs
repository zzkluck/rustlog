mod log_parser;
mod easylog;
mod evaluator;

use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use crate::easylog::EasyLog;
use crate::log_parser::LogParser;

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
    config_path: PathBuf
}


fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match cli.subcommand {
        ParseArgs::EasyLog(args) => {
            let log_path = args.log_path;
            let config_path = args.config_path;
            let easylog = EasyLog::new(&config_path);
            let mut pl = easylog.parse_from_file(&log_path);
            println!("{} templates found.", pl.templates.len());
            pl.templates.sort();
            for t in pl.templates {
                println!("{:}", t);
            }
        }
    }
}