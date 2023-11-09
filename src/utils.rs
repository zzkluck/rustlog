// type Error = &'static str;
use std::collections::HashSet;

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