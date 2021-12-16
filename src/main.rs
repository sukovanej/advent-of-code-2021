pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_13;
pub mod day_14;
pub mod day_15;

use crate::day_14::{parse_input, solution2};
use std::fs::read_to_string;

fn main() {
    let file_name = "inputs/input_14_1.txt";
    let input_content = read_to_string(file_name).unwrap();
    let input = parse_input(&input_content);

    println!("{}", solution2(&input));
}