// type Error = &'static str;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use counter::Counter;
use colored::{Colorize, Color};
use fancy_regex::Regex;
use log::{debug, trace};
use polars_core::prelude::*;

lazy_static! {
    pub static ref LOG_TYPES: Vec<String> = std::fs::read_dir("./data/loghub_2k_corrected")
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect();
}

lazy_static! {
    pub static ref DATE_ALIAS: HashSet<&'static str> =
    HashSet::from_iter(vec!["JAN", "FEB", "MAR", "APR", "MAY", "JUN",
        "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN",
        "PDT", "UTC"]);
}

lazy_static! {
    pub static ref TERM_COLORS: Vec<Color> = vec![
        Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan
    ];
}

pub const DEFAULT_LOG_FORMAT: &'static str = r"^<Content>$";

lazy_static! {
    pub static ref DEFAULT_LOG_FORMAT_REGEX: Regex =
        Regex::new(DEFAULT_LOG_FORMAT).unwrap();
}

pub fn counter<'a, T: Eq + Hash + 'a>(list: impl Iterator<Item=&'a T>) -> HashMap<&'a T, u64> {
    let mut counter: HashMap<&T, u64> = HashMap::new();
    for item in list {
        *counter.entry(item).or_insert(0) += 1;
    }
    counter
}

pub fn combine_print(lines: Vec<&str>) -> () {
    let sentences: Vec<Vec<&str>> = lines.iter().map(|line| line.split(' ').collect()).collect();
    let word_bag = sentences.iter().flat_map(|x| x).collect::<Counter<_>>();
    // let highest_frequency = word_bag.k_most_common_ordered(1)[0].1;
    // let weight = 0.1;
    // let threshold: usize = (highest_frequency as f64 * weight) as usize;
    let threshold: usize = 20;

    for words in sentences.iter() {
        let combine_len = words.iter().map(|w| word_bag.get(w).unwrap()).collect::<Counter<_>>();
        let mut color_map = HashMap::<&usize, _>::new();
        for (k, v) in combine_len.k_most_common_ordered(TERM_COLORS.len()) {
            if *k < threshold { continue; }
            if v == 1 { break; }
            color_map.insert(k, TERM_COLORS[color_map.len()]);
        }
        for w in words.iter() {
            let wc = word_bag[w];
            if color_map.contains_key(&wc) {
                print!("{} ", w.color(color_map[&wc]));
            }
            else {
                print!("{} ", w);
            }
        }
        println!();
    }
}

pub fn get_all_named_group(re: &Regex) -> Vec<&str> {
    re.capture_names()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<&str>>()
}

pub fn generate_logformat_regex(logformat: &str) -> Regex {
    let re_square = Regex::new(r"(<[^<>]+>)").unwrap();
    let re_space = Regex::new(r" +").unwrap();

    let mut regex = String::new();
    let mut start = 0usize;

    for m in re_square.find_iter(logformat) {
        let m = m.unwrap();
        let splitter = re_space.replace_all(&logformat[start..m.start()], r"\s+");
        regex.push_str(splitter.as_ref());
        let header = &logformat[m.start()+1..m.end()-1];
        regex.push_str(&format!(r"(?P<{header}>.*?)"));
        start = m.end();
    }
    if start != logformat.len() {
        let splitter = re_space.replace_all(&logformat[start..], r"\s+");
        regex.push_str(splitter.as_ref());
    }
    Regex::new(&format!("^{regex}$")).unwrap()
}

pub fn read_lines_from_file(path: &Path) -> Vec<String> {
    debug!("Try read log file {:?}", path);
    let mut f = File::open(path)
        .expect(&format!("Fail to open {}", path.to_str().unwrap()));
    let mut buffer = String::new();

    let timer_start = Instant::now();
    f.read_to_string(&mut buffer)
        .expect(&format!("Fail to open {}", path.to_str().unwrap()));
    debug!("Read file content to memory. Finished. Time cost: {:?}.", timer_start.elapsed());

    let timer_start = Instant::now();
    let mut lines: Vec<String> = buffer.split("\n").map(|x| String::from_str(x).unwrap()).collect();
    if let Some(last_line) = lines.last() {
        if *last_line == "" {
            lines.pop();
        }
    }
    trace!("Split raw text to {} log lines. Time cost: {:?}.", lines.len() ,timer_start.elapsed());
    lines
}

pub fn log_to_dataframe(lines: Vec<&str>, re: Regex) -> DataFrame {
    let headers = get_all_named_group(&re);
    let mut series_buffer = vec![];
    for _ in headers.iter() {
        series_buffer.push(vec![]);
    }
    for line in lines {
        match re.captures(line) {
            Ok(captures) => {
                let captures = captures.unwrap();
                for (i, name) in headers.iter().enumerate() {
                    series_buffer[i].push(captures.name(name).unwrap().as_str());
                }
            }
            Err(_) => {
                debug!("Log format not match. {}", line);
            }
        }
    }
    let mut series = vec![];
    for (i, name) in headers.iter().enumerate() {
        series.push(Series::new(name, &series_buffer[i]));
    }
    DataFrame::new(series).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_logformat_regex_success() {
        let re =
            generate_logformat_regex("<Date> <Time> <Pid> <Level> <Component>: <Content>");
        let headers = get_all_named_group(&re);
        assert_eq!(headers, vec!["Date", "Time", "Pid", "Level", "Component", "Content"]);
        let test_log =
            "081110 115218 13 INFO dfs.DataBlockScanner: Verification succeeded for blk_4545188422655940106";
        let m = re.captures(test_log).unwrap().unwrap();

        assert_eq!(m.name("Date").unwrap().as_str(), "081110");
        assert_eq!(m.name("Time").unwrap().as_str(), "115218");
        assert_eq!(m.name("Pid").unwrap().as_str(), "13");
        assert_eq!(m.name("Level").unwrap().as_str(), "INFO");
        assert_eq!(m.name("Component").unwrap().as_str(), "dfs.DataBlockScanner");
        assert_eq!(m.name("Content").unwrap().as_str(), "Verification succeeded for blk_4545188422655940106");
    }
    #[test]
    fn log_to_dataframe_success(){
        let re =
            generate_logformat_regex("<Date> <Time> <Pid> <Level> <Component>: <Content>");
        let lines = &read_lines_from_file(r"data/loghub_2k_corrected/HDFS/HDFS_2k.log".as_ref())[..10];
        let df = log_to_dataframe(lines.iter().map(|x| x.as_ref()).collect(), re);
        println!("{:?}", df);
    }


    #[test]
    fn counter_normal_success() {
        let stub: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let test_counter: HashMap<&i32, u64> = counter(stub.iter());
        assert_eq!(test_counter[&1], 1);
        assert_eq!(test_counter[&2], 2);
        assert_eq!(test_counter[&3], 3);
        assert_eq!(test_counter[&4], 4);
    }

    #[test]
    fn counter_str_slice_success() {
        let mut stub: Vec<&str> = vec!["1", "2", "2", "3", "3", "3", "4", "4", "4", "4"];
        stub.sort();
        let test_counter: HashMap<&&str, u64> = counter(stub.iter());
        assert_eq!(test_counter[&"1"], 1);
        assert_eq!(test_counter[&"2"], 2);
        assert_eq!(test_counter[&"3"], 3);
        assert_eq!(test_counter[&"4"], 4);
    }

    #[test]
    fn counter_string_success() {
        let stub: Vec<String> =
            vec!["1", "2", "2", "3", "3", "3", "4", "4", "4", "4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect();
        let test_counter: HashMap<&String, u64> = counter(stub.iter());
        assert_eq!(test_counter[&"1".to_string()], 1);
        assert_eq!(test_counter[&"2".to_string()], 2);
        assert_eq!(test_counter[&"3".to_string()], 3);
        assert_eq!(test_counter[&"4".to_string()], 4);
    }
}