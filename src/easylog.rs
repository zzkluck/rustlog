use std::collections::HashMap;
use tqdm::Iter;
use crate::log_parser::{LogParser, ParsedLog};

fn is_variable(token: &str) -> bool {
    for c in token.chars(){
        if c.is_numeric() {
            return true;
        }
    }
    false
}

pub struct EasyLog {}
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
        let mut template = String::new();
        for token in line.split(' ') {
            if !is_variable(token) {
                if template.len() != 0 {
                    template.push(' ');
                }
                template.push_str(token);
            }
        }
        template
    }
}