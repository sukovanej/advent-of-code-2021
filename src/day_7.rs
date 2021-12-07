use std::fs::read_to_string;

pub fn solution(input: &[u64], fuel_consumption: Box<dyn Fn(&[u64], u64) -> u64>) -> u64 {
    let mut numbers = input.to_vec();
    numbers.sort();

    let mut best_middle_idx = input.len() / 2;
    let mut best_middle = fuel_consumption(&numbers, numbers[best_middle_idx]);
    let mut max_left_idx: usize = 0;
    let mut max_right_idx = input.len();

    loop {
        println!(
            "min = {}, best {}, max = {}",
            max_left_idx, best_middle_idx, max_right_idx
        );

        let new_left_idx = (best_middle_idx + max_left_idx) / 2;
        let new_right_idx = (max_right_idx + best_middle_idx) / 2;

        let new_left = fuel_consumption(&numbers, numbers[new_left_idx]);
        let new_right = fuel_consumption(&numbers, numbers[new_right_idx]);

        if new_left < best_middle {
            max_right_idx = best_middle_idx;
            best_middle_idx = new_left_idx;
            best_middle = new_left;
        } else if new_right < best_middle {
            max_left_idx = best_middle_idx;
            best_middle_idx = new_right_idx;
            best_middle = new_right;
        } else {
            max_right_idx = new_right_idx;
            max_left_idx = new_left_idx;
        }

        if new_left_idx + 1 == best_middle_idx && best_middle_idx == new_right_idx {
            break;
        }
    }

    best_middle
}

pub fn solution1(input: &[u64]) -> u64 {
    solution(&input, Box::new(fuel_consumption_constant_rate))
}

pub fn solution2(input: &[u64]) -> u64 {
    solution(&input, Box::new(fuel_consumption))
}

fn fuel_consumption_constant_rate(input: &[u64], middle_value: u64) -> u64 {
    input
        .iter()
        .map(|&i| i64::abs(i as i64 - middle_value as i64) as u64)
        .sum()
}

fn fuel_consumption(input: &[u64], middle_value: u64) -> u64 {
    input
        .iter()
        .map(|&i| sum_one_to_n(i64::abs(i as i64 - middle_value as i64) as u64))
        .sum()
}

fn sum_one_to_n(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn parse_input(file_name: &str) -> Vec<u64> {
    read_to_string(file_name)
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

#[test]
fn test_solution1() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    // 16,1,2,0,4,2,7,1,2,14
    // 0, 1, 1, 2, 2, 2, 4, 7, 14, 16

    assert_eq!(solution1(&input), 37);

    assert_eq!(solution1(&parse_input("inputs/input_7_1.txt")), 336120);
}

#[test]
fn test_solution2() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    // 16,1,2,0,4,2,7,1,2,14
    // 0, 1, 1, 2, 2, 2, 4, 7, 14, 16

    assert_eq!(solution2(&input), 168);

    // assert_eq!(solution2(&parse_input("inputs/input_7_1.txt")), 336120);
}
