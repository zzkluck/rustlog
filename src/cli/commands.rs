use std::path::Path;
use log::info;
use crate::cli::args::{BenchmarkArgs, ParseArgs};
use crate::dataset::LogDataset;
use crate::dataset::loghub_common_dataset::LoghubCommonDataset;
use crate::evaluator::get_accuracy;
use crate::log_parser::easylog::EasyLog;
use crate::log_parser::{LogParser};
use crate::utils;

fn get_parse_method(method: &Option<String>, config_path: &Path) -> Box<dyn LogParser> {
    match method {
        None => { unimplemented!() }
        Some(parser_type) => {
            if parser_type == "easylog" {
                Box::new(EasyLog::new(config_path))
            } else {
                unimplemented!()
            }
        }
    }
}

fn parse_from_loghub(parser: Box<dyn LogParser>, log_type: &str) {
    let data_root = format!("./data/loghub_2k_corrected/{}", log_type);
    let log_path = format!("{}/{}_2k.log", data_root, log_type);
    let structured_path = format!("{}/{}_2k.log_structured_corrected.csv",
                                  data_root, log_type);

    let pl = parser.parse_from_file(log_path.as_ref());
    info!("{}: {} templates found.", log_type, pl.templates.len());
    let dataset = LoghubCommonDataset::from_file(structured_path.as_ref());
    info!("{}: Group Accuracy {:}", log_type,  get_accuracy(dataset.iter_event_id(), pl.parsed_list).3)
}
pub fn parse_command(args: ParseArgs) {
    let parser = get_parse_method(&args.method, &args.config_path);
    parse_from_loghub(parser, &args.log_type);
}

pub fn benchmark_command(args: BenchmarkArgs) {
    println!("Benchmark enable.");

    for log_type in utils::LOG_TYPES.iter() {
        let config_path = format!("./data/easylog_configs/{}.config.toml", log_type);
        // let data_root = format!("./data/loghub_2k_corrected/{}", log_type);
        // let log_path = format!("{}/{}_2k.log", data_root, log_type);
        // let structured_path = format!("{}/{}_2k.log_structured_corrected.csv", data_root, log_type);

        let parser = get_parse_method(&args.method, config_path.as_ref());
        parse_from_loghub(parser, &log_type);
    }
}
