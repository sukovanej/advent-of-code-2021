mod day_1;
mod day_2;

use crate::day_2::{solution2};
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("inputs/input_2_1.txt").expect("expected file");
    let result = solution2(&data);
    println!("{}", result);
}
