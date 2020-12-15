use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

const PREAMBLE_LENGTH : usize = 25;

pub struct XMasData {
    data: Vec<i64>,
}

struct SumTable {
    table : Vec<(i64, Vec<i64>)>
}

impl SumTable {
    fn init_from_slice(&mut self, to_insert: &[i64]) {
        for (i, outer) in to_insert.iter().enumerate() {
            let mut vec: Vec<i64> = Vec::with_capacity(PREAMBLE_LENGTH);
            for(j, inner) in to_insert.iter().enumerate() {
                if i == j { continue; }
                vec.push(inner + outer);
            }
            self.table.push((*outer, vec));
        }
    }

    fn insert_element(&mut self, to_insert: i64, index : usize) {
        let insert_index = index % PREAMBLE_LENGTH;

        let mut sum_vec : Vec<i64> = Vec::with_capacity(PREAMBLE_LENGTH - 1);
        for (i, value) in self.table.iter().enumerate() {
            if i == insert_index { continue; }
            sum_vec.push(value.0 + to_insert);
        }

        // Now insert it into the table.
        self.table[insert_index] = (to_insert, sum_vec);
    }

    fn contains_number(&self, target: i64) -> bool {
        for row in self.table.iter() {
            let sums : &Vec<i64> = &row.1;
            if sums.iter().any(|&num| num == target){
                return true;
            }
        }
        false 
    }
}



// O(N), where N is the number of elements in our table. The important thing
// here is that we don't re-compute sums. 
fn can_form_target_efficient(table: &SumTable, target: i64) -> bool {
    table.contains_number(target)
}

pub fn silver(data: &mut XMasData) -> Option<(i64, i64)>{
    let mut table : SumTable = SumTable{
        table: Vec::with_capacity(PREAMBLE_LENGTH)
    };

    table.init_from_slice(&data.data[0..PREAMBLE_LENGTH]);

    // Contains non-preamble data.
    let rest : &[i64] = &data.data[PREAMBLE_LENGTH..];
    for (i, &value) in rest.iter().enumerate() {
        if !can_form_target_efficient(&table, value) {
            // println!("(Silver) Found number that can't be formed: {}", value);
            return Some((i as i64, value));
        }
        table.insert_element(value, i);
    }
    None
}

pub fn gold(data: &mut XMasData) {
    let (invalid_num_index, invalid_number) = silver(data).unwrap();

    // New and improved O(N) algorithm.
    let number_data = &data.data[0..invalid_num_index as usize];
    let mut sum_vec : Vec<i64> = Vec::new();
    let mut sum = 0;

    for value in number_data {
        sum_vec.push(*value);
        sum += value;

        while sum > invalid_number {
            sum -= sum_vec[0];
            sum_vec.remove(0);
        }

        if sum == invalid_number {
            let min = sum_vec.iter().min().unwrap();
            let max = sum_vec.iter().max().unwrap();
            println!("Gold: A winner is you! {} + {} = {}" , min, max, min + max);
        }
    }

}

pub fn setup() -> XMasData{
    let file = BufReader::new(File::open("src/9_input.txt").unwrap());

    XMasData {
        data: file.lines().map(|s| s.unwrap().parse().unwrap()).collect()
    }
}

pub fn day_9_soln() {
    let mut data : XMasData = setup();

    silver(&mut data);
    gold(&mut data);
}


struct SumTableHash {
    table: Vec<(i64, HashSet<i64>)>
}

impl SumTableHash {
    fn init_from_slice(&mut self, to_insert: &[i64]) {
        for (i, outer) in to_insert.iter().enumerate() {
            let mut set: HashSet<i64> = HashSet::with_capacity(PREAMBLE_LENGTH);
            for(j, inner) in to_insert.iter().enumerate() {
                if i == j { continue; }
                set.insert(inner + outer);
            }
            self.table.push((*outer, set));
        }
    }

    fn insert_element(&mut self, to_insert: i64, index : usize) {
        let insert_index = index % PREAMBLE_LENGTH;

        let mut set : HashSet<i64> = HashSet::with_capacity(PREAMBLE_LENGTH - 1);
        for (i, value) in self.table.iter().enumerate() {
            if i == insert_index { continue; }
            set.insert(value.0 + to_insert);
        }

        // Now insert it into the table.
        self.table[insert_index] = (to_insert, set);
    }

    fn contains_number(&self, target: i64) -> bool {
        for row in self.table.iter() {
            let sums : &HashSet<i64> = &row.1;
            if sums.iter().any(|&num| num == target){
                return true;
            }
        }
        false
    }
}

pub fn silver_hash(data: &mut XMasData) -> Option<(i64, i64)>{
    let mut table : SumTableHash = SumTableHash{
        table: Vec::with_capacity(PREAMBLE_LENGTH)
    };

    table.init_from_slice(&data.data[0..PREAMBLE_LENGTH]);

    // Contains non-preamble data.
    let rest : &[i64] = &data.data[PREAMBLE_LENGTH..];
    for (i, &value) in rest.iter().enumerate() {
        if !table.contains_number(value) {
            return Some((i as i64, value));
        }
        table.insert_element(value, i);
    }
    None
}

pub fn gold_hash(data: &mut XMasData) {
    let (invalid_num_index, invalid_number) = silver(data).unwrap();

    // New and improved O(N) algorithm.
    let number_data = &data.data[0..invalid_num_index as usize];
    let mut sum_vec : Vec<i64> = Vec::new();
    let mut sum = 0;

    for value in number_data {
        sum_vec.push(*value);
        sum += value;

        while sum > invalid_number {
            sum -= sum_vec[0];
            sum_vec.remove(0);
        }

        if sum == invalid_number {
            let min = sum_vec.iter().min().unwrap();
            let max = sum_vec.iter().max().unwrap();
            println!("Min/Max: {:?}", (min, max));
        }
    }

}

// Implementation of Day 9, but with SumTableHash
pub fn day_9_soln_hash () {
    let mut data = setup();
    silver_hash(&mut data);
    gold_hash(&mut data);
}
