pub mod easylog;


use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::path::Path;
use log::{debug, trace};
use std::time::Instant;

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
        debug!("Try read log file {:?}", log_path);
        let mut f = File::open(log_path)
            .expect(&format!("Fail to open {}", log_path.to_str().unwrap()));
        let mut buffer = String::new();

        let timer_start = Instant::now();
        f.read_to_string(&mut buffer)
            .expect(&format!("Fail to open {}", log_path.to_str().unwrap()));
        debug!("Read file content to memory. Finished. Time cost: {:?}.", timer_start.elapsed());

        let timer_start = Instant::now();
        let mut lines: Vec<&str> = buffer.split("\n").collect();
        if let Some(last_line) = lines.last() {
            if *last_line == "" {
                lines.pop();
            }
        }
        trace!("Split raw text to {} log lines. Time cost: {:?}.", lines.len() ,timer_start.elapsed());

        let timer_start = Instant::now();
        let res = self.parse(lines);
        debug!("Parse completed. Time cost: {:?}.", timer_start.elapsed());

        return res;
    }
}
