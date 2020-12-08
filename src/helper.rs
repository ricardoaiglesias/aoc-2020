use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn file_to_vec(filepath : &str) -> Option<Vec<String>> {
    let mut result : Vec<String> = Vec::new();

    let reader = BufReader::new(File::open(filepath).unwrap());
    for line_ in reader.lines() {
        let line = line_.unwrap().to_string();
        result.push(line)
    }

    Some(result)
}
