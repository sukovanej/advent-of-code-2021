#![feature(is_sorted)]

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;

use crate::day_14::{parse_input, solution2};
use std::fs::read_to_string;

fn main() {
    let file_name = "inputs/input_14_1.txt";
    let input_content = read_to_string(file_name).unwrap();
    let input = parse_input(&input_content);

    println!("{}", solution2(&input));
}
