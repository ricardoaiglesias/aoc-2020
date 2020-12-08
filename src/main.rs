// mod day_0;
// mod day_1;
mod helper;
// mod day_2;
// mod day_4;
// mod day_5;
//mod day_6;
mod day_8;

use std::time::{Instant};

fn main() {
    let start = Instant::now();
    day_8::day_8_soln();
    let duration = start.elapsed();

    println!("Time Elapsed: {:?}", duration);
}
