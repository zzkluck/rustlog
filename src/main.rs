use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use tqdm::Iter;

fn is_variable(token: &str) -> bool {
    for c in token.chars(){
        if c.is_numeric() {
            return true;
        }
    }
    false
}

fn main() {
    let mut f = File::open(r"C:\Users\zzkluck\Desktop\HDFS.log").unwrap();
    let mut logs = String::new();
    f.read_to_string(&mut logs).unwrap();
    let lines: Vec<&str> = logs.split("\r\n").collect();

    let mut cluster: HashMap<String, Vec<&str>> = HashMap::new();
    for line in lines.iter().tqdm() {
        let mut template = String::new();
        for token in line.split(' ') {
            if !is_variable(token) {
                if template.len() != 0 {
                    template.push(' ');
                }
                template.push_str(token);
            }
        }
        if cluster.contains_key(&template) {
            cluster.get_mut(&template).unwrap().push(line);
        }
        else {
            cluster.insert(template, vec![line]);
        }
    }
    println!("{:?}", cluster.keys());
}
