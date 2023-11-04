use serde::Deserialize;
use crate::utils::LogRecord;
use super::*;

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