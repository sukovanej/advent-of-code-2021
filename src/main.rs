pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_13;

use crate::day_13::{parse_input, solution2};
use std::fs::read_to_string;

fn main() {
    let file_name = "inputs/input_13_1.txt";
    let input_content = read_to_string(file_name).unwrap();
    let input = parse_input(&input_content);

    println!("{}", solution2(&input));
}