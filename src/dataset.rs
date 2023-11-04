use std::path::Path;
use std::str::FromStr;
use crate::data_model::LogRecord;
use serde::Deserialize;
use chrono::prelude::*;

pub trait LogDataset {
    fn from_file(file_path: &Path) -> Self;
    fn get_records(&self) -> &Vec<LogRecord>;

    fn iter_event_id(&self) -> Vec<&str> {
        self.get_records().iter()
            .map(|r| r.event_id.as_str())
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AndroidRecord {
    #[serde(rename="LineId")]
    line_id: u32,
    #[serde(rename="Date")]
    date: String,
    #[serde(rename="Time")]
    time: String,
    #[serde(rename="Pid")]
    pid: i32,
    #[serde(rename="Tid")]
    tid: i32,
    #[serde(rename="Level")]
    level: String,
    #[serde(rename="Component")]
    component: String,
    #[serde(rename="Content")]
    content: String,
    #[serde(rename="EventId")]
    event_id: String,
    #[serde(rename="EventTemplate")]
    event_template: String
}


pub struct AndroidDataset {
    records: Vec<LogRecord>
}

impl LogDataset for AndroidDataset {
    fn from_file(file_path: &Path) -> Self {
        let mut records = vec![];
        let mut reader = csv::Reader::from_path(file_path).expect("TODO: panic!");
        for record in reader.deserialize() {
            let record: AndroidRecord = record.expect("TODO: panic!");
            records.push(LogRecord {
                index: record.line_id,
                event_id: record.event_id,
                timestamp: DateTime::parse_from_str(
                    &format!("1970 {} {} +08:00", record.date, record.time),
                    "%Y %m-%d %H:%M:%S.%f %z"
                    ).unwrap_or_default(),
                attribute: String::default(),
                resource: record.component,
                trace_id: record.pid.to_string(),
                span_id: record.tid.to_string(),
                severity: record.level,
                body: record.content,
                label: String::default(),
            })
        }
        AndroidDataset { records }
    }

    fn get_records(&self) -> &Vec<LogRecord> {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn iter_event_id_test() {
        let ad =
            AndroidDataset::from_file
                (r"C:\Users\zzkluck\Desktop\LogCodes\clog\logs\Andriod\Andriod_2k.log_structured.csv".as_ref());
        let event_ids = ad.iter_event_id();
        assert_eq!("E100", event_ids[0]);
        assert_eq!("E10", event_ids[1]);
        assert_eq!("E103", event_ids[2]);
        assert_eq!("E131", event_ids[3]);
    }
}