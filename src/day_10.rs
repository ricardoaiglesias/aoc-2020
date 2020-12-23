use crate::helper::file_to_vec_transform;

pub struct JoltAdapter {
    jolt_diffs: [usize; 3],
}

pub fn silver(data: &[usize], adapter_info: &mut JoltAdapter) {
    println!("Data: {:?}", data);

    for (i, elem) in data.iter().enumerate() {
        if i == data.len() - 1 {
            break;
        }

        let next = data[i + 1];
        adapter_info.jolt_diffs[next - elem - 1] += 1;
    }

    let jolt_1 = adapter_info.jolt_diffs[0];
    let jolt_3 = adapter_info.jolt_diffs[2] + 1;

    println!(
        "1-Jolt Diffs: {} | 3-Jolt Diffs: {}. Result: {}",
        jolt_1,
        jolt_3,
        jolt_1 * jolt_3
    );
}

fn gold_helper(data: &[usize], index: usize, end: usize, memo: &mut [Option<usize>]) -> usize {
    if memo[index].is_some() {
        return memo[index].unwrap();
    }

    if data[index] == end - 3 {
        return 1;
    }

    // Otherwise, check the next three indices and recurse.
    let mut result = 0;
    let curr_value = data[index];
    for next_ind in index + 1..=index + 3 {
        if next_ind >= data.len() {
            break;
        }

        if curr_value + 3 >= data[next_ind] {
            result += gold_helper(data, next_ind, end, memo)
        }
    }
    memo[index] = Some(result);
    result
}

pub fn gold(data: &[usize]) {
    let end = data.last().unwrap() + 3;
    let mut memo = vec![None; data.len()];

    let result = gold_helper(data, 0, end, &mut memo);
    println!("(Gold) Result: {}", result);
}

pub fn setup() -> Vec<usize> {
    let mut data: Vec<usize> = file_to_vec_transform("src/10_input.txt", |x| x.parse().unwrap());
    data.push(0);
    data.sort();
    data
}
pub fn day_10_soln() {
    let data = setup();

    let mut ja: JoltAdapter = JoltAdapter { jolt_diffs: [0; 3] };
    silver(&data, &mut ja);
    println!("Starting gold...");
    gold(&data);
}
