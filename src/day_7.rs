use std::collections::HashMap;
use crate::helper::file_to_vec;

type Bag = String;
struct Rule {
    inner_bag : Bag,
    n_bags : usize // Number of bags with color "inner_bag"
}

type RuleSet<'a> = HashMap<Bag, &'a Vec<Rule>>;

fn substr_to_rule(s : &str) -> Rule {
    let start_index = if None == s.find(',') { 0 } else { 2 };
    println!("S: {}", &s[start_index..]);

    let space_seps = s[start_index..].match_indices(' ');
    let num_index = space_seps.clone().next().unwrap().0 + start_index;
    let bag_index = space_seps.clone().nth(2).unwrap().0 + start_index;

    let n_bags : usize = s[start_index..num_index].parse().unwrap();
    let bag_type = s[num_index + 1..bag_index].to_string();

    Rule {
        inner_bag :  bag_type,
        n_bags
    }
}

fn str_to_rule(s : &str) -> (Bag, Vec<Rule>) {
    // By replacing the period with a comma, we simplify the parsing by a bit.
    let s_copy = s.replace(".", ",");

    let v: (usize, &str) = s.match_indices(' ').nth(1).unwrap();
    let source_bag : &str = &s[0.. v.0];

    if None != s.find("no other bags") {
        return (source_bag.to_owned() , Vec::new())
    };

    let mut rules : Vec<Rule> = Vec::new();

    let separator = "contain ";
    let dest_rule_start_ind = s.find(separator).unwrap() + separator.len();
    let dest_rule_str : &str = &s_copy[dest_rule_start_ind..];

    // Multiple rules.
    let mut prev_index = 0;
    let comma_indices = dest_rule_str.match_indices(',');
    for (index, _match_str) in comma_indices {
        rules.push(substr_to_rule(&dest_rule_str[prev_index..index]));
        prev_index = index;
    }

    (source_bag.to_owned(), rules)
}



/// SILVER SOLUTION
fn can_hold_bag(rules : &RuleSet, memo: &mut HashMap<Bag, bool>, curr_bag : &str)
                -> bool {
    if memo.contains_key(curr_bag) {
        return *memo.get(curr_bag).unwrap();
    }

    // Otherwise, get the set of bags the current bag can hold and recursively
    // figure out whether we can hold the gold bag.
    let curr_rule : &Vec<Rule> = rules.get(curr_bag).unwrap();

    for rule in curr_rule {
        // Base Case.
        if rule.inner_bag == "shiny gold" {
            return true
        }

        // Recursive case.
        let recursive_can_hold = can_hold_bag(rules, memo, &rule.inner_bag);
        if recursive_can_hold {
            memo.insert(curr_bag.to_string(), true);
            return true;
        }
    }
    memo.insert(curr_bag.to_string(), false);
    false
}

/// GOLD SOLUTION
fn get_num_inner_bags(curr_bag : &str, ruleset : &RuleSet) -> usize {
    let rhs : &Vec<Rule> = ruleset.get(curr_bag).unwrap();
    if rhs.is_empty() {
        return 1;
    }
    rhs.iter().fold(
        1 /* Include this bag.*/, |sum, curr_rule : &Rule|
        sum + /* Include child bags*/
        (curr_rule.n_bags * get_num_inner_bags(&curr_rule.inner_bag, ruleset)))
}

fn build_ruleset(rule_vec : &[(Bag, Vec<Rule>)]) -> HashMap<Bag, &Vec<Rule>>{
    let mut ruleset : HashMap<Bag, &Vec<Rule>> = HashMap::new();

    for rule in rule_vec{
        ruleset.insert(rule.0.clone(), &rule.1);
    }

    ruleset
}

pub fn day_7_soln() {
    let str_vec = file_to_vec("src/7_input.txt").unwrap();
    let rule_vec : Vec<(Bag, Vec<Rule>)> = str_vec.iter().map(|s| str_to_rule(s)).collect();

    let ruleset = build_ruleset(&rule_vec);
    let mut memo : HashMap<Bag, bool> = HashMap::new();

    let n_able: usize = rule_vec.iter().filter(|x| can_hold_bag(&ruleset, &mut memo, &x.0)).count();
    println!("Silver {}", n_able);

    // GOLD
    println!("Gold: {} ", get_num_inner_bags(&"shiny gold".to_owned(), &ruleset) -1 );
}
