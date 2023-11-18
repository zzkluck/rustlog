pub mod easylog;

use std::collections::HashMap;
use std::path::Path;
use log::{debug};
use std::time::Instant;
use crate::utils::read_lines_from_file;

pub struct ParsedLog {
    pub templates: Vec<String>,
    pub clusters: HashMap<String, Vec<String>>,
    pub parsed_list: Vec<usize>,
}

impl ParsedLog {
    pub fn new(templates: Vec<String>,
               clusters: HashMap<String, Vec<String>>,
               parsed_list: Vec<usize>
    ) -> Self {
        ParsedLog { templates, clusters, parsed_list, }
    }
}

pub trait LogParser {
    fn parse(&self, logs: Vec<&str>) -> ParsedLog;
    fn parse_line(&self, log: &str) -> String;
    fn parse_from_file(&self, log_path: &Path) -> ParsedLog {
        let mut buffer = String::new();
        let lines = read_lines_from_file(log_path, &mut buffer);

        let timer_start = Instant::now();
        let res = self.parse(lines);
        debug!("Parse completed. Time cost: {:?}.", timer_start.elapsed());

        return res;
    }
}
