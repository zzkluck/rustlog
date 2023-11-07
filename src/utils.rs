use std::fmt::format;
use regex::Regex;
type Error = &'static str;

lazy_static! {
    pub static ref LOG_TYPES: Vec<String> = std::fs::read_dir("./data/loghub_2k_corrected")
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
}

pub fn generate_log_format_regex(log_format: &str) -> Result<(Vec<&str>, Regex), Error> {
    let mut header:Vec<&str> = vec![];
    let field_re = Regex::new(r"(<[^<>]+>)").unwrap();
    let space_re = Regex::new(r" +").unwrap();
    let mut regex = String::new();

    for (i, splitter) in field_re.split(log_format).enumerate() {
        println!("{}", splitter);
        if i % 2 == 0 {
            regex.push_str(space_re.replace(splitter,r"\s+").as_ref());
        }
        else {
            let field = splitter.strip_prefix("<").unwrap()
                                      .strip_suffix(">").unwrap();
            header.push(field);
            regex.push_str(format!("(?P<{field}>.*?").as_str());
        }
    }
    Ok((header, Regex::new(regex.as_str()).unwrap()))
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    # [test]
    fn test_generate_log_format_regex() {
        generate_log_format_regex("<Date> <Time> <Pid> <Level> <Content>")
            .expect("TODO: panic message");
    }


}