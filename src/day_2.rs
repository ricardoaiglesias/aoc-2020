use std::io::{BufReader};

struct PasswordEntry {
    min_num : usize,
    max_num : usize,
    letter : char,
    password : String
}

fn file_to_text(path : String) -> Vec<PasswordEntry> {
    let mut result : Vec<PasswordEntry> = Vec::new();

    let reader = BufReader::new(File::open(path).unwrap());
    for line_ in reader.lines() {
        let line = line_.unwrap().to_string();

        let index_1 = line.find('-').unwrap();
        let index_2 = line.find(' ').unwrap();
        let index_3 = line.find(':').unwrap();

        let min_str = &line[0..index_1];
        let max_str = &line[index_1+1..index_2];
        let char_str = &line[index_2+1 .. index_3];

        let password = &line[index_3 + 1..].trim();

        let pword_entry = PasswordEntry {
            min_num : min_str.parse().unwrap(),
            max_num : max_str.parse().unwrap(),
            letter : char_str.chars().next().unwrap(),
            password : password.to_string()
        };

        result.push(pword_entry);
    }

    result
}

fn is_valid_password_1(info : &PasswordEntry) -> bool{
    let num_matches = info.password.matches(info.letter).count();
    num_matches >= info.min_num  && num_matches <= info.max_num
}

fn is_valid_password_2(info : &PasswordEntry) -> bool {
    let c = info.letter;

    let min_match : bool = info.password.chars().nth(info.min_num - 1).unwrap() == c;
    let max_match : bool= info.password.chars().nth(info.max_num - 1).unwrap() == c;

    min_match ^ max_match
}


pub fn day_1_solve()
{
    let info : Vec<PasswordEntry> = file_to_text("src/day_1.txt".to_string());
    let iter = info.iter();
    let iter_2 = iter.clone();

    let valid_1 = iter.filter( |x| is_valid_password_1(x)).count() ;
    let valid_2 = iter_2.filter( |x| is_valid_password_2(x)).count();

    println!("Count - 1: {}", valid_1);
    println!("Count - 2: {}", valid_2);
}
