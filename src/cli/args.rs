use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum  ParseArgs {
    EasyLog(EasyLogArgs),
}

#[derive(Args, Debug)]
pub struct EasyLogArgs {
    #[arg(short, long, value_name = "LOG_TYPE")]
    pub(crate) log_type: String,
    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub(crate) config_path: PathBuf,
}