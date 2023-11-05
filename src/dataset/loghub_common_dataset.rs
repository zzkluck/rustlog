use std::collections::HashMap;
use std::path::Path;
use chrono::DateTime;
use crate::dataset::{LogDataset, LogRecord};

pub struct LoghubCommonDataset {
    records: Vec<LogRecord>
}

impl LogDataset for LoghubCommonDataset {
    fn from_file(file_path: &Path) -> Self {
        let mut records = vec![];
        let mut reader = csv::Reader::from_path(file_path).expect("TODO: panic!");
        let mut idx: HashMap<&str, usize> = HashMap::new();
        let header = reader.headers().cloned().unwrap();
        for (i, field) in header.iter().enumerate() {
            idx.insert(field, i);
        }
        for record in reader.records() {
            let record = record.unwrap();
            records.push(LogRecord {
                index: record[idx["LineId"]].parse().unwrap(),
                event_id: record[idx["EventId"]].to_string(),
                timestamp: DateTime::default(),
                attribute: String::default(),
                resource: String::default(),
                trace_id: String::default(),
                span_id: String::default(),
                severity: String::default(),
                body: record[idx["Content"]].to_string(),
                label: String::default(),
            })
        }
        LoghubCommonDataset { records }
    }

    fn get_records(&self) -> &Vec<LogRecord> {
        &self.records
    }
}