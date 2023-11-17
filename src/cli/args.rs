use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum  CliOperation {
    Parse(ParseArgs),
    Benchmark(BenchmarkArgs),
}

#[derive(Args, Debug)]
pub struct BenchmarkArgs {
    #[arg(short, long, value_name = "METHOD")]
    pub(crate) method: Option<String>,
}

#[derive(Args, Debug)]
pub struct ParseArgs {
    #[arg(short, long, value_name = "LOG_PATH")]
    pub(crate) log_path: String,
    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub(crate) config_path: Option<PathBuf>,
    #[arg(short, long, value_name = "METHOD")]
    pub(crate) method: Option<String>,
}