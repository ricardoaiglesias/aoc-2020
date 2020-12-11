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

pub fn file_to_vec_transform<T>(filepath: &str, transform_fn: fn(&str) -> T)
                                -> Vec<T> {
    let file = File::open(filepath);
    let data = match file {
        Err(err) => panic!("Unable to open filepath: {} | Error: {}", filepath, err),
        Ok(handle) => BufReader::new(handle)
    };

    data.lines().map(|l| transform_fn(&l.unwrap())).collect()
}

pub fn file_to_vec_transform_with_idx<T>(filepath: &str, transform_fn: fn(usize, &str) -> T)
                                -> Vec<T> {
    let file = File::open(filepath);
    let data = match file {
        Err(err) => panic!("Unable to open filepath: {} | Error: {}", filepath, err),
        Ok(handle) => BufReader::new(handle)
    };

    data.lines().enumerate().map(|(idx, data)| transform_fn(idx, &data.unwrap())).collect()
}
