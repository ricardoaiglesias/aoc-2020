use std::fmt;
use std::fs;

use std::collections::{HashMap, HashSet};

type IRange = (usize, usize); // (lower, higher)

pub struct Rule {
    name: String,
    rules: Vec<IRange>,
}

pub struct Ticket {
    fields: Vec<usize>,
}

impl Rule {
    fn contains(&self, value: usize) -> bool {
        self.rules
            .iter()
            .any(|(low, high)| value >= *low && value <= *high)
    }
}

impl fmt::Debug for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.fields)
    }
}

/// Maps each field in a ticket to a list of rules that are valid for that
/// ticket.
fn ticket_to_valid_rules(rules: &[Rule], ticket: &Ticket) -> Vec<(usize, Vec<usize>)> {
    ticket
        .fields
        .iter()
        .map(|field| {
            let vec: Vec<usize> = rules
                .iter()
                .enumerate()
                .filter_map(|(index, rule)| match rule.contains(*field) {
                    false => None,
                    true => Some(index),
                })
                .collect();
            (*field, vec)
        })
        .collect()
}

pub struct Data {
    rules: Vec<Rule>,
    personal: Ticket,
    nearby: Vec<Ticket>,
}

fn line_to_ticket(line: &str) -> Ticket {
    let nums = line
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    Ticket { fields: nums }
}

pub fn setup(filepath: &str) -> Data {
    let filestr = fs::read_to_string(filepath).unwrap();
    let parts: Vec<&str> = filestr.split("\n\n").collect();

    let (rule_str, personal_str, nearby_str) = (parts[0], parts[1], parts[2]);

    let rule_vec: Vec<Rule> = rule_str
        .split('\n')
        .map(|s: &str| {
            let line: Vec<&str> = s.split(": ").collect();
            let name = line[0];
            let rules: Vec<IRange> = line[1]
                .split(" or ")
                .map(|range_str| {
                    let values: Vec<usize> = range_str
                        .split('-')
                        .map(|v| v.trim().parse::<usize>().unwrap())
                        .collect();
                    (values[0], values[1])
                })
                .collect();

            Rule {
                name: name.to_string(),
                rules,
            }
        })
        .collect();

    let personal_ticket: Ticket = line_to_ticket(personal_str.split('\n').nth(1).unwrap());

    let nearby_tickets: Vec<Ticket> = nearby_str
        .split('\n')
        .skip(1)
        .filter_map(|line| match line.find(',') {
            None => None,
            Some(_) => Some(line_to_ticket(line)),
        })
        .collect();

    Data {
        nearby: nearby_tickets,
        personal: personal_ticket,
        rules: rule_vec,
    }
}

pub fn silver(data: &Data) {
    // fn ticket_to_valid_rules(rules: &[Rule], ticket: &Ticket)
    //                             -> Vec<(usize, Vec<usize>)> {
    let sum: usize = data
        .nearby
        .iter()
        .map(|ticket| {
            let field_to_valid_rules = ticket_to_valid_rules(&data.rules, ticket);
            field_to_valid_rules
                .iter()
                .filter(|(_field, valid_rules)| valid_rules.is_empty())
                .fold(0, |sum, (field, _valid_rules)| {
                    println!("Invalid Field: {}", field);
                    sum + field
                })
        })
        .sum();
    println!("Sum: {}", sum);
}

type ValidTickets<'a> = &'a [Vec<(usize, Vec<usize>)>];
fn generate_possible_fields_for_columns(
    valid_tickets: ValidTickets,
) -> Vec<(usize, HashSet<usize>)> {
    let mut possible_fields: Vec<(usize, HashSet<usize>)> = Vec::with_capacity(32);
    let n_cols = valid_tickets[0].len();

    for col in 0..n_cols {
        let mut valid_fields: HashSet<usize> = HashSet::new();
        for data in valid_tickets[0][col].1.iter() {
            valid_fields.insert(*data);
        }

        for row in valid_tickets.iter().skip(1) {
            let mut fields_in_row: HashSet<usize> = HashSet::new();
            for data in row[col].1.iter() {
                fields_in_row.insert(*data);
            }

            let new: HashSet<usize> = valid_fields.intersection(&fields_in_row).copied().collect();
            valid_fields = new;
        }
        possible_fields.push((col, valid_fields));
    }

    possible_fields
        .sort_by(|(_index, possibilities), (_, rhs)| possibilities.len().cmp(&rhs.len()));
    possible_fields
}

type ColumnPossibilities<'a> = &'a [(usize, HashSet<usize>)];
fn map_rule_to_field(column_to_valid_fields: ColumnPossibilities) -> HashMap<usize, usize> {
    let mut index_to_field: HashMap<usize, usize> = HashMap::new();

    // Map from Rule Index (order in which it appears in the puzzle input) to
    // Ticket Index (Which column in your ticket it is).
    index_to_field.insert(
        *column_to_valid_fields[0].1.iter().next().unwrap(),
        column_to_valid_fields[0].0,
    );
    for column in 0..column_to_valid_fields.len() - 1 {
        let current = &column_to_valid_fields[column].1;
        let next = &column_to_valid_fields[column + 1];

        // Remove the current field's values from the next fields values.
        let next_fields: HashSet<usize> = next.1.clone();
        let next_field: HashSet<_> = next_fields.difference(current).collect();

        let rule_column = *next_field.iter().next().unwrap();
        let ticket_column = next.0;

        // Map from row index to ticket index.
        // Whenever we say "Get Rule Index A", we really mean "Get Index B"
        index_to_field.insert(*rule_column, ticket_column);
    }
    index_to_field
}

pub fn gold(data: &Data) {
    let valid_tickets: Vec<Vec<(usize, Vec<usize>)>> = data
        .nearby
        .iter()
        .filter_map(|ticket| {
            let valid_rules_in_ticket_fields: Vec<(usize, Vec<usize>)> =
                ticket_to_valid_rules(&data.rules, ticket);
            let is_invalid = valid_rules_in_ticket_fields
                .iter()
                .any(|(_, vec)| vec.is_empty());
            match is_invalid {
                true => None,
                false => Some(valid_rules_in_ticket_fields),
            }
        })
        .collect();

    let column_to_valid_fields = generate_possible_fields_for_columns(&valid_tickets);

    // Column Index -> Which field this index corresponds to.
    let rule_to_field = map_rule_to_field(&column_to_valid_fields);

    // Get the field columns with "departure".
    let indices: Vec<usize> = data
        .rules
        .iter()
        .enumerate()
        .filter(|(_index, rule)| rule.name.find("departure").is_some())
        .map(|(rule_index, _)| *rule_to_field.get(&rule_index).unwrap())
        .collect();

    assert!(indices.len() == 6);

    let value: usize = indices
        .iter()
        .map(|index| {
            println!("Index: {} | Value {}", index, data.personal.fields[*index]);
            data.personal.fields[*index]
        })
        .product();
    println!("Value: {}", value);
}

// Because of how the problem's laid out, when you organize by the
// length of the possibilities, the possibilities at index i+1 are the
// same as the possibilities at index i, plus one extra.
//
// By removing all of the ones in index i, we are left with the "plus
// one extra" column, which gives us the answer.

pub fn day_16_soln() {
    let data = setup("src/16.txt");
    gold(&data);
}
