use crate::helper::file_to_vec;

struct Seat {
    row : usize,
    col : usize
}

type Partition = (usize, usize);

enum Direction {
    UPPER,
    LOWER
}

fn divide_partition(current : Partition, dir : Direction) -> Partition {
    let low = current.0; let high = current.1;
    let midpoint : usize = (low + high ) / 2;

    match dir {
        Direction::UPPER => {
            (midpoint + 1, high)
        },
        Direction::LOWER => {
            (low, midpoint)
        }
    }
}

fn pass_to_seat(boarding_pass : &str) -> Seat {
    let bf_string = &boarding_pass[0..7];
    let lr_string = &boarding_pass[7..];

    println!("Lengths: {} {}", bf_string.len(), lr_string.len());

    let mut curr_bf_partition = (0, 127);
    for bf in bf_string.chars() {
        let dir : Direction = if bf == 'F' { Direction::LOWER }else { Direction::UPPER};
        curr_bf_partition = divide_partition(curr_bf_partition, dir);
    }

    let mut curr_lr_partition = (0, 7);
    for lr in lr_string.chars() {
        let dir : Direction = if lr == 'L' { Direction::LOWER } else { Direction::UPPER};
        curr_lr_partition = divide_partition(curr_lr_partition, dir);
    }
    assert_eq!(curr_lr_partition.0, curr_lr_partition.1);
    assert_eq!(curr_bf_partition.0, curr_bf_partition.1);

    Seat { row : curr_bf_partition.0, col: curr_lr_partition.0 }
}

fn seat_to_id(s : &Seat ) -> usize { s.row * 8 + s.col }

pub fn day_5_soln() {
    let vec : Vec<String> = file_to_vec("./src/5_input.txt".to_owned()).unwrap();

    let seat_vec : Vec<Seat> = vec.iter().map(|pass| pass_to_seat(pass)).collect();
    let mut id_vec : Vec<usize>= seat_vec.iter().map(|s| seat_to_id(s)).collect();
    id_vec.sort(); // If we sort, the problem becomes much simpler.  O(N Log N). 

    // Part 1. Actually fairly simple.

    println!("(Silver) Max: {}", id_vec[id_vec.len() - 1]);

    // Part 2: Our seat is one where there's a gap between two IDs, so just find
    // the ID where (PREV_ID +1 != ID)
    let mut prev_id = 0;
    for id in id_vec {
        if id - 1 != prev_id  && prev_id != 0{
            println!("(Gold) Missing seat: ID {} vs {}, so missing is {}", id, prev_id, id - 1);
        }

        prev_id = id;
    }

}
