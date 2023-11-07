use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tqdm::Iter;
use serde::Deserialize;
use fancy_regex::Regex;
use super::*;

fn is_variable(token: &str) -> bool {
    for c in token.chars(){
        if c.is_numeric() {
            return true;
        }
    }
    false
}

#[derive(Deserialize)]
struct Config {
    specific: Vec<String>,
    substitute: HashMap<String, String>
}

#[allow(dead_code)]
pub struct EasyLog {
    specific: Vec<Regex>,
    substitute: Vec<(Regex, String)>
}

impl EasyLog {
    pub fn new(config_path: &Path) -> Self{
        let mut f = File::open(config_path)
            .expect(&format!("Fail to open {}", config_path.to_str().unwrap()));
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("TODO: panic message");
        let config: Config = toml::from_str(&buffer).expect("TODO: panic message");

        let mut specific: Vec<Regex> = vec![];
        for k in config.specific.iter() {
            let re  = Regex::new(&k).unwrap();
            specific.push(re);
        }

        let mut substitute: Vec<(Regex, String)> = vec![];
        for (k,v) in config.substitute.into_iter() {
            let re  = Regex::new(&k).unwrap();
            substitute.push((re, v));
        }

        EasyLog {
            specific,
            substitute,
        }
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
}