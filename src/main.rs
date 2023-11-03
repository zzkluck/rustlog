mod log_parser;
mod easylog;

use std::path::PathBuf;
use clap::{Parser};
use crate::easylog::EasyLog;
use crate::log_parser::LogParser;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, value_name = "LOG_PATH")]
    log_path: Option<PathBuf>
}


fn main() {
    let cli = Cli::parse();
    let easylog = EasyLog{};
    if let Some(log_path) = cli.log_path.as_deref() {
        let pl = easylog.parse_from_file(log_path);
        println!("{:?}", pl.templates);
    }
}
