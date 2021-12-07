pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

use crate::day_6::{parse_input, solution};

// use clap::Parser;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref DAYS_WITH_INPUT_FILES: HashMap<u32, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(1, vec!["input_1_1.txt", "input_1_test.txt"]);
        map.insert(2, vec!["input_2_1.txt"]);
        map.insert(3, vec!["input_3_1.txt", "input_3_test.txt"]);
        map.insert(4, vec!["input_4_1.txt", "input_4_test.txt"]);
        map
    };
}

// #[derive(Parser)]
// #[clap(version = "1.0", author = "Milan Suk <milansuk@email.cz>")]
// struct Opts {
//     #[clap(short, long, parse(from_occurrences))]
//     day: u32,
// }

fn main() {
    //let opts: Opts = Opts::parse();

    let file_name = "inputs/input_6_1.txt";
    let input = parse_input(file_name);

    println!("{}", solution(&input, 256));
}

// fn run_day_1(filename: &str) {
//
// }
