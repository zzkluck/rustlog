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