use crate::cli::args::EasyLogArgs;
use crate::dataset::android_dataset::AndroidDataset;
use crate::dataset::LogDataset;
use crate::evaluator::get_accuracy;
use crate::log_parser::easylog::EasyLog;
use crate::log_parser::LogParser;

pub fn easy_log_command(args: EasyLogArgs) {
    let easylog = EasyLog::new(&args.config_path);
    let data_root = format!("./data/loghub_2k_corrected/{}", args.log_type);
    let log_path = format!("{}/{}_2k.log", data_root, args.log_type);
    let structured_path = format!("{}/{}_2k.log_structured_corrected.csv", data_root, args.log_type);

    let pl = easylog.parse_from_file(log_path.as_ref());
    println!("{} templates found.", pl.templates.len());
    let dataset = AndroidDataset::from_file(structured_path.as_ref());
    println!("{:?}", get_accuracy(dataset.iter_event_id(),
                                  pl.parsed_list))
}