pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

use crate::day_4::{parse_input, solution2};

fn main() {
    let file_name = "inputs/input_4_1.txt";
    let (numbers, boards) = parse_input(&file_name);

    println!("{}", solution2(&numbers, &boards));
}
