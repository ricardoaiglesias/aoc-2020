use std::collections::HashSet;
use std::fs;

fn line_to_answers_1(s : &str, chars_so_far : &mut HashSet<char>){
    for c in s.chars() {
        chars_so_far.insert(c);
    }
}

fn line_to_answers_2(s : &str, chars_so_far : &HashSet<char>) -> HashSet<char>{
    let mut curr_set : HashSet<char> = HashSet::new();
    for c in s.chars() {
        curr_set.insert(c);
    }
    let inter : HashSet<&char> = chars_so_far.intersection(&curr_set).collect();

    let mut return_set : HashSet<char> = HashSet::new();
    for c in inter {
        return_set.insert(*c);
    }

    return_set
}


fn silver()
{
    let file_str = fs::read_to_string("./src/6_test.txt").unwrap();

    let groups = file_str.split("\n\n");

    let mut total_answered : usize = 0;
    for group in groups {
        let mut group_answers : HashSet<char> = HashSet::new();
        let group_line = String::from(group);

        for line in  group_line.split('\n') {
            line_to_answers_1(line, &mut group_answers);
        }

        println!("Size of Group: {} ", group_answers.iter().count());
        total_answered += group_answers.iter().count();
    }
    println!("(Silver) Answer: {}", total_answered);

}

pub fn day_6_soln() {
    let file_str = fs::read_to_string("./src/6_input.txt").unwrap();

    // Part 2 (Struggling with Lifetimes).
    //let file_str = fs::read_to_string("./src/6_test.txt").unwrap();

    let mut total_answered = 0;
    for group in file_str.split("\n\n") {
        let mut group_answers : HashSet<char> = HashSet::new();

        let group_line = String::from(group);
        let n_chars = group_line.chars().count();
        let first_line = group_line[0..group_line.find('\n').unwrap_or(n_chars)].to_owned();

        println!("First Line: {}", first_line);

        for c in first_line.chars() {
            group_answers.insert(c);
        }

        println!("Size Before: {}", group_answers.iter().count());
        for line in group_line.split('\n') {
            if line.chars().count() == 0 { // Literally just fuck my shit up.
                continue;
            }

            println!("Current Line: {}|", line);
            group_answers = line_to_answers_2(line, &group_answers);
        }
        println!("Size After: {}\n\n", group_answers.iter().count());

        total_answered += group_answers.iter().count();
    }
    println!("(Gold) Answer: {}", total_answered);
}
