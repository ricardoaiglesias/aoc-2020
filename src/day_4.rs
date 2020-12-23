use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

pub fn file_to_vec(filepath: String) -> Option<Vec<Vec<(String, String)>>> {
    let mut result: Vec<Vec<(String, String)>> = Vec::new();

    let reader = BufReader::new(File::open(filepath).unwrap());
    let mut current_kv_collection: Vec<(String, String)> = Vec::new();

    for line_ in reader.lines() {
        let line: String = line_.unwrap().to_string();
        if line.is_empty() {
            // Reset current kv_collection.
            result.push(current_kv_collection);
            current_kv_collection = Vec::new();
            continue;
        }

        // Split the current line on spaces.
        let split_line: Vec<&str> = line.split(' ').collect();

        let mut entry_vec: Vec<String> = Vec::new();
        for entry in split_line {
            entry_vec.push(entry.to_string());
        }

        // For every entry, split by key/value.
        for entry in entry_vec {
            let split_ind: usize = entry.find(':').unwrap();

            let my_str = entry.to_string();

            let key: &str = &my_str[..split_ind].trim();
            let value: &str = &my_str[split_ind + 1..].trim().to_string();

            current_kv_collection.push((key.to_string(), value.to_string()));
        }
    }

    Some(result)
}

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn is_valid_byr(s: &str) -> bool {
    let num_digits = s.len();

    let num: i64 = s.parse().unwrap();
    num >= 1920 && num <= 2002 && num_digits == 4
}

fn is_valid_iyr(s: &str) -> bool {
    let num_digits = s.len();
    let num: i64 = s.parse().unwrap();
    num >= 2010 && num <= 2020 && num_digits == 4
}

fn is_valid_eyr(s: &str) -> bool {
    let num_digits = s.len();
    let num: i64 = s.parse().unwrap();
    num >= 2020 && num <= 2030 && num_digits == 4
}

fn is_valid_hgt(s: &str) -> bool {
    // Try seeing if you can find 'cm'
    let found_cm = s.find("cm");
    let found_in = s.find("in");
    /*
     *     If cm, the number must be at least 150 and at most 193.
            If in, the number must be at least 59 and at most 76.

    */
    match found_cm {
        None => match found_in {
            // IN Case.
            Some(index) => {
                let height_str = s[0..index].to_owned();
                let height_value: i64 = height_str.parse().unwrap();

                height_value >= 59 && height_value <= 76
            }
            None => false,
        },
        // CM case
        Some(index) => {
            let height_str = s[0..index].to_owned();
            let height_value: i64 = height_str.parse().unwrap();
            height_value >= 150 && height_value <= 193
        }
    }
}

fn is_valid_hcl(s: &str) -> bool {
    let num_digits = s.len();
    let hex_str: &str = &s[1..];

    for c in hex_str.chars() {
        let is_hex_letter = c >= 'a' && c <= 'f';
        let is_hex_num = c >= '0' && c <= '9';
        if !(is_hex_letter || is_hex_num) {
            return false;
        }
    }

    num_digits == 7
}

fn is_valid_ecl(s: &str) -> bool {
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for color in valid_colors {
        if s == color {
            return true;
        }
    }
    false
}

fn is_valid_pid(s: &str) -> bool {
    for c in s.chars() {
        if !(c >= '0' && c <= '9') {
            return false;
        }
    }
    s.len() == 9
}

fn is_valid_set_gold(kv_set: &[(String, String)]) -> bool {
    let mut validity_criteria = HashMap::<String, fn(&str) -> bool>::new();
    validity_criteria.insert("byr".to_string(), is_valid_byr);
    validity_criteria.insert("iyr".to_string(), is_valid_iyr);
    validity_criteria.insert("eyr".to_string(), is_valid_eyr);
    validity_criteria.insert("hgt".to_string(), is_valid_hgt);
    validity_criteria.insert("hcl".to_string(), is_valid_hcl);
    validity_criteria.insert("ecl".to_string(), is_valid_ecl);
    validity_criteria.insert("pid".to_string(), is_valid_pid);

    for (key, value) in kv_set {
        if key == "cid" {
            continue;
        }
        let function = validity_criteria.get(key).unwrap();
        let is_valid = function(value);
        if !is_valid {
            return false;
        }
    }

    // Once it's done, return whether all the required keys were actually there.
    is_valid_kv_set(kv_set)
}

fn is_valid_kv_set(kv_set: &[(String, String)]) -> bool {
    let mut required_keys_map: HashMap<String, bool> = map! {
        "byr".to_owned() => false,
        "iyr".to_owned() => false,
        "eyr".to_owned() => false,
        "hgt".to_owned() => false,
        "hcl".to_owned() => false,
        "ecl".to_owned() => false,
        "pid".to_owned() => false
    };

    for (key, _value) in kv_set {
        if !required_keys_map.contains_key(&key.to_string()) && key != "cid" {
            println!("FUCK! {}", key);
            return false;
        }

        // Otherwise, change the corresponding entry to true.
        required_keys_map.insert(key.to_string(), true);
    }

    for (key, value) in required_keys_map {
        if !value {
            println!("Missing Key ({})", key);
            return false;
        }
    }
    true
}

pub fn day_4_soln() {
    let vec: Vec<Vec<(String, String)>> = file_to_vec("src/4_input.txt".to_string()).unwrap();
    let iter = vec.into_iter();
    let iter2 = iter.clone();

    let part_1 = &iter.filter(|kv_set| is_valid_kv_set(kv_set)).count();
    println!("Silver - {} Valid Passports", part_1);

    let part_2 = &iter2.filter(|kv_set| is_valid_set_gold(kv_set)).count();
    println!("Gold - {} Valid Passports", part_2);
}
