pub mod android_dataset;
pub mod loghub_common_dataset;

use std::path::Path;
use chrono::prelude::*;
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

    fn iter_event_id(&self) -> Vec<&str> {
        self.get_records().iter()
            .map(|r| r.event_id.as_str())
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
                (r"C:\Users\zzkluck\Desktop\LogCodes\clog\logs\Andriod\Andriod_2k.log_structured.csv".as_ref());
        let event_ids = ad.iter_event_id();
        assert_eq!("E100", event_ids[0]);
        assert_eq!("E10", event_ids[1]);
        assert_eq!("E103", event_ids[2]);
        assert_eq!("E131", event_ids[3]);
    }
}