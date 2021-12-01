mod day_1;

use crate::day_1::{read_numbers, solution2};

fn main() {
    let data = read_numbers("inputs/input_1_1.txt");
    let result = solution2(&data);
    println!("{}", result);
}
