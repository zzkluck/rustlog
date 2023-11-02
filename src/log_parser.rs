use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::path::Path;

pub struct ParsedLog {
    pub templates: Vec<String>,
    clusters: HashMap<String, Vec<String>>,
    parsed_list: Vec<usize>,
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
        let mut f = File::open(log_path)
            .expect(&format!("Fail to open {}", log_path.to_str().unwrap()));
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let lines: Vec<&str> = buffer.split("\r\n").collect();
        self.parse(lines)
    }
}