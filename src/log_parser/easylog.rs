use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tqdm::Iter;
use serde::Deserialize;
use fancy_regex::Regex;
use log::{info, warn};
use crate::utils::*;
use super::*;

fn is_variable(token: &str) -> bool {
    token.len() == 0
    || token == " "
    || token.as_bytes()[0] == u8::try_from('<').unwrap()
    || token.chars().any(|x| x.is_numeric())
    || token.chars().all(|x| ('a' <= x.to_ascii_lowercase() && x.to_ascii_lowercase() <= 'f'))
    || DATE_ALIAS.contains(token.to_ascii_uppercase().as_str())
}

#[derive(Deserialize)]
struct Config {
    specific: Option<Vec<String>>,
    substitute: Option<HashMap<String, String>>,
    logformat: Option<String>
}

pub struct EasyLog {
    specific: Vec<Regex>,
    substitute: Vec<(Regex, String)>,
    logformat: Regex
}

impl Default for EasyLog {
    fn default() -> Self {
        EasyLog {specific:vec![], substitute:vec![], logformat: DEFAULT_LOG_FORMAT_REGEX.clone() }
    }
}

impl EasyLog {
    pub fn new(config_path: Option<&Path>) -> Self{
        let config_path = match config_path {
            Some(path) => { path }
            None => {
                info!("Config file path is not provided. Use default.");
                return EasyLog::default()
            }
        };
        let mut f = match File::open(config_path) {
            Ok(handle) => { handle }
            Err(e) => {
                warn!("{}", e.to_string());
                warn!("File to open provided config file. Use default.");
                return EasyLog::default()
            }
        };
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Read Config File Failed");

        let config: Config = toml::from_str(&buffer).expect("Toml Parse Failed");

        let specific: Vec<Regex> = match config.specific {
            None => { vec![] }
            Some(raw_specific) => {
                raw_specific.iter()
                    .map(|s| match Regex::new(s.as_str()) {
                        Ok(re) => { Some(re) }
                        Err(e) => {
                            warn!("{}", e.to_string());
                            warn!("Fail to parse {s} to regex.");
                            None
                        }
                    })
                    .filter(|r| r.is_some())
                    .map(|r| r.unwrap())
                    .collect()
            }
        };

        let substitute: Vec<(Regex, String)> = match config.substitute {
            None => { vec![] }
            Some(raw_substitute) => {
                raw_substitute.into_iter()
                    .map(|(p, s)| match Regex::new(p.as_str()) {
                        Ok(re) => { Some((re, s)) }
                        Err(e) => {
                            warn!("{}", e.to_string());
                            warn!("Fail to parse {p} to regex.");
                            None
                        }
                    })
                    .filter(|r| r.is_some())
                    .map(|r| r.unwrap())
                    .collect()
            }
        };

        let logformat = match config.logformat {
            None => { DEFAULT_LOG_FORMAT_REGEX.clone() }
            Some(p) => {
                generate_logformat_regex(&p)
            }
        };
        EasyLog { specific, substitute, logformat }
    }
}

impl LogParser for EasyLog {
    fn parse(&self, lines: Vec<&str>) -> ParsedLog {
        let mut clusters: HashMap<String, Vec<String>> = HashMap::new();
        let mut templates: HashMap<String, usize> = HashMap::new();
        let mut parsed_list = Vec::<usize>::new();

        for line in lines.iter().tqdm() {
            let template = self.parse_line(line);
            if templates.contains_key(&template) {
                clusters.get_mut(&template).unwrap().push(line.to_string());
            }
            else {
                clusters.insert(template.clone(), vec![line.to_string()]);
                templates.insert(template.clone(), templates.len());
            }
            parsed_list.push(templates[&template]);
        }

        let mut templates_vec: Vec<String> = vec![String::new(); templates.len()];
        for (template, i) in templates {
            templates_vec[i] = template;
        }
        ParsedLog::new(templates_vec, clusters, parsed_list)
    }

    fn parse_line(&self, line: &str) -> String {
        let mut template:Vec<&str> = vec![];

        for re in self.specific.iter() {
            let result = re.find_iter(&line)
                .map(|x| x.unwrap().as_str());
            template.extend(result);
        }

        let mut line = line.to_string();
        for (re, sub) in self.substitute.iter() {
            line = re.replace_all(&line, sub).parse().unwrap();
        }

        for token in line.split(' ') {
            if !is_variable(token) {
                template.push(token);
            }
        }
        template.join(" ")
    }

    fn parse_from_file(&self, log_path: &Path) -> ParsedLog {
        assert!(get_all_named_group(&self.logformat).iter().find(|s| **s=="Content").is_some());
        let mut buffer = String::new();
        let lines = read_lines_from_file(log_path, &mut buffer);

        let timer_start = Instant::now();
        let contents:Vec<&str> = lines.iter().tqdm()
            .map(|line| match self.logformat.captures(line) {
                Err(e) => {
                    warn!("{}. Skip.", e.to_string());
                    None
                }
                Ok(result) => {
                    match result {
                        None => {
                            warn!("Log |{line}| not match to logformat. Skip.");
                            None
                        }
                        Some(cap) => {
                            Some(cap.name("Content").unwrap().as_str())
                        }
                    }
                }
            })
            .filter(|r| r.is_some())
            .map(|r| r.unwrap())
            .collect();

        debug!("Extract content form logs completed. Time cost: {:?}.", timer_start.elapsed());

        let timer_start = Instant::now();
        let res = self.parse(contents);
        debug!("Parse completed. {} templates found. Time cost: {:?}.", res.templates.len() ,timer_start.elapsed());
        return res;
    }
}