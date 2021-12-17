use std::fs::read_to_string;

pub fn solution(input: &[u64], fuel_consumption: Box<dyn Fn(&[u64], u64) -> u64>) -> u64 {
    let mut numbers = input.to_vec();
    numbers.sort_unstable();

    let mut best_middle = numbers[numbers.len() / 2];
    let mut best_consumption = fuel_consumption(&numbers, best_middle);
    let mut max_left = numbers[0];
    let mut max_right = *numbers.last().unwrap();

    loop {
        let new_left = (best_middle + max_left) / 2;
        let new_right = (max_right + best_middle) / 2;

        let new_left_consumption = fuel_consumption(&numbers, new_left);
        let new_right_consumption = fuel_consumption(&numbers, new_right);

        if new_left_consumption < best_consumption {
            max_right = best_middle;
            best_middle = new_left;
            best_consumption = new_left_consumption;
        } else if new_right_consumption < best_consumption {
            max_left = best_middle;
            best_middle = new_right;
            best_consumption = new_right_consumption;
        } else {
            max_right = new_right;
            max_left = new_left;
        }

        if new_right - new_left <= 1 {
            break;
        }
    }

    best_consumption
}

pub fn solution1(input: &[u64]) -> u64 {
    solution(input, Box::new(fuel_consumption_constant_rate))
}

pub fn solution2(input: &[u64]) -> u64 {
    solution(input, Box::new(fuel_consumption))
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

    assert_eq!(solution1(&input), 37);
    assert_eq!(solution1(&parse_input("inputs/input_7_1.txt")), 336120);
}

#[test]
fn test_solution2() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    assert_eq!(solution2(&input), 168);
    assert_eq!(solution2(&parse_input("inputs/input_7_1.txt")), 96864235);
}

#[test]
fn test_fuel_consumption() {
    assert_eq!(fuel_consumption(&[1, 2, 3], 2), 2);
    assert_eq!(fuel_consumption(&[1, 3, 4], 3), 3 + 1);
    assert_eq!(fuel_consumption(&[1, 4, 5], 4), 6 + 1);
    assert_eq!(fuel_consumption(&[16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 5), 168);
}

#[test]
fn test_sum_one_to_n() {
    assert_eq!(sum_one_to_n(4), 10);
    assert_eq!(sum_one_to_n(11), 66);
}
