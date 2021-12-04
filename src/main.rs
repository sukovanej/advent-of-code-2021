pub mod day_1;
pub mod day_2;
pub mod day_3;

use crate::day_3::{solution2, str_to_bit_array, BitArray};
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("inputs/input_3_1.txt").expect("Can't open input file");
    let lines = data
        .lines()
        .map(str_to_bit_array)
        .collect::<Vec<BitArray>>();

    let result = solution2(&lines);
    println!("{}", result);
}
