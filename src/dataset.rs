pub mod android_dataset;
pub mod loghub_common_dataset;
mod raw_text_dataset;

use std::path::Path;
use chrono::{DateTime, FixedOffset};

#[allow(dead_code)]
#[derive(Debug)]
pub struct LogRecord {
    pub index: u32,
    pub event_id: String,
    pub timestamp: DateTime<FixedOffset>,
    pub attribute: String,
    pub resource: String,
    pub trace_id: String,
    pub span_id: String,
    pub severity: String,
    pub body: String,
    pub label: String
}
pub trait LogDataset {
    fn from_file(file_path: &Path) -> Self;
    fn get_records(&self) -> &Vec<LogRecord>;

    fn get_event_ids(&self) -> Vec<&str> {
        self.get_records().iter()
            .map(|r| r.event_id.as_str())
            .collect()
    }

    fn get_log_contents(&self) -> Vec<&str> {
        self.get_records().iter()
            .map(|r| r.body.as_str())
            .collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use android_dataset::AndroidDataset;
    # [test]
    fn iter_event_id_test() {
        let ad = AndroidDataset::from_file
                (r"./data/loghub_2k_corrected/Android/Android_2k.log_structured_corrected.csv".as_ref());
        let event_ids = ad.get_event_ids();
        assert_eq!("E100", event_ids[0]);
        assert_eq!("E10", event_ids[1]);
        assert_eq!("E103", event_ids[2]);
        assert_eq!("E131", event_ids[3]);
    }
}