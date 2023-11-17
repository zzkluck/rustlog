// type Error = &'static str;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;
use counter::Counter;
use colored::{Colorize, Color};

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

pub fn counter<'a, T: Eq + Hash + 'a>(list: impl Iterator<Item=&'a T>) -> HashMap<&'a T, u64> {
    let mut counter: HashMap<&T, u64> = HashMap::new();
    for item in list {
        *counter.entry(item).or_insert(0) += 1;
    }
    counter
}

pub fn combine_print(log_path: &Path) -> () {
    let mut f = File::open(log_path)
        .expect(&format!("Fail to open {}", log_path.to_str().unwrap()));
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)
        .expect(&format!("Fail to open {}", log_path.to_str().unwrap()));

    let mut lines: Vec<&str> = buffer.split("\n").collect();
    if let Some(last_line) = lines.last() {
        if *last_line == "" {
            lines.pop();
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
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