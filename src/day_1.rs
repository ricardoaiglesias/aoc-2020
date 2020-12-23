use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn file_to_vec(filepath: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let file = File::open(filepath.to_owned()).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let num_str = line.unwrap().to_string();
        result.push(num_str.parse().unwrap());
    }

    result
}

/// Pairs algorithm:
/// For each number in your list, have the key of a map entry be the number and
/// have the value be "2020 - key".
///
/// Part 1: Then, once you create the map, simply iterate through all the map's
/// entries. For each key, if the value is also a key in the map, you have found
/// the solution.
///
/// Part 2: Create a hash map for each element, e, in your index. Then, run the
/// pairs algorithm to see if there are two numbers, a and b that add to (2020 -
/// e) If (a + b == 2020 - e), then (a + b + e == 2020), giving us our solution.
fn create_map(nums: &[i32], target: i32) -> HashMap<i64, i64> {
    let mut map: HashMap<i64, i64> = HashMap::new();
    for n in nums {
        if n >= &target {
            continue;
        }
        map.insert(i64::from(*n), (target - n).into());
    }
    map
}

/// Day 1:
/// Part 1: 989824
/// Part 2: 66432240
pub fn day_1_soln() {
    let nums = file_to_vec("src/1_input.txt");
    let sum_map = create_map(&nums, 2020);

    // // Part 1.
    // for (num, result) in &sum_map {
    //     if sum_map.get(result).is_some() {
    //         println!("(Day 1 - Silver) Found value! {} * {} = {}", num, result, num * result);
    //         break;

    //     }
    // }

    // Part 2.
    for inner in &nums {
        if inner >= &2020 {
            continue;
        }

        let inner_sum_map = create_map(&nums, 2020 - inner);
        for (lhs, rhs) in &inner_sum_map {
            if sum_map.get(rhs).is_some() {
                println!(
                    "(Day 1 - Gold) Found value {} * {} * {} = {}",
                    inner,
                    lhs,
                    rhs,
                    i64::from(*inner) * rhs * lhs
                );
                return;
            }
        }
    }
}
