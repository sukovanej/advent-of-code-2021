mod day_1;
mod day_2;
mod day_3;

use crate::day_3::{solution1, BitArray, str_to_bit_array};
use std::fs::read_to_string;

fn main() {
    let data = read_to_string("inputs/input_3_1.txt").expect("Can't open input file");
    let lines = data.lines().map(|s| str_to_bit_array(s)).collect::<Vec<BitArray>>();

    let result = solution1(&lines);
    println!("{}", result);
}
