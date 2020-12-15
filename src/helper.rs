use std::iter::Map;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

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

type Line =  std::io::Lines<std::io::BufReader<std::fs::File>>;

pub fn file_parse_lazy<T>(filepath: &str) ->
    Map<Line, fn(Result<String, std::io::Error>)-> T>
    where T: FromStr + Clone, <T as std::str::FromStr>::Err: std::fmt::Debug
{
    let file = File::open(filepath);
    let data = match file {
        Err(err) => panic!("Unable to open filepath: {} | Error: {}", filepath, err),
        Ok(handle) => BufReader::new(handle)
    };

    data.lines().map(|l: Result<String, std::io::Error>| {
        l.unwrap().parse::<T>().unwrap()
    })
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
