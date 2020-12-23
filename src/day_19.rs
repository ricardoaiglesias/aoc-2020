use crate::helper::*;

type Rule = Vec<usize>;
type RuleSet = Vec<Rule>;

pub struct Info {
    patterns: Vec<(usize, RuleSet)>,
    strings: Vec<String>
}

fn is_valid_rule(rulesets: &Info, rules_to_check: Vec<Rule>, string: &str) -> bool {
    let curr_ruleset = &rulesets.patterns[curr_rule_idx].1;
    for rule in curr_ruleset {
        
    }

    false
}

pub fn silver(data: &Info) {

}

pub fn gold() {}

pub fn setup(filepath: &str) -> Info {
    let data = std::fs::read_to_string(filepath).unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();

    let (rules, strings): (&str, &str) = (parts[0], parts[1]);

    let mut ruleset_collection: Vec<(usize, RuleSet)> = Vec::with_capacity(256);
    for line in rules.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        let rule_index = parts[0].parse::<usize>().unwrap();

        let mut ruleset: RuleSet = Vec::with_capacity(2);
        // Now, split by the pipe symbol.
        let rule_parts: Vec<&str> = parts[1].split(" | ").collect();
        for rule_option in rule_parts {
            let nums: Vec<usize> = rule_option.split(' ').map(|s: &str| s.trim().parse::<usize>().unwrap()).collect();
            ruleset.push(nums);
        }

        ruleset_collection.push((rule_index, ruleset));
    }

    ruleset_collection.sort_by_key(|v| v.0);
    let strings_to_check: Vec<String> = strings.split('\n').map(|s| s.to_string()).collect();

    Info {
        patterns: ruleset_collection,
        strings: strings_to_check
    }
}

pub fn day_19_soln() {
    setup("src/19_test.txt");
}
