use std::fs::read_to_string;

/// 0 -> 6
/// 1 -> 2
/// 2 -> 5
/// 3 -> 5
/// 4 -> 4
/// 5 -> 5
/// 6 -> 5
/// 7 -> 3
/// 8 -> 7
/// 9 -> 5
///

type Input<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

pub fn solution1(input: Input) -> u64 {
    let unique_lengs = vec![2, 4, 3, 7];
    input
        .iter()
        .map(|line| {
            line.1
                .iter()
                .filter(|&&i| unique_lengs.contains(&(i.len() as u64)))
                .collect::<Vec<&&str>>()
                .len() as u64
        })
        .sum()
}

pub fn parse_input(filename: &str) -> Input {
    let mut lines = read_to_string(filename).unwrap().lines();
    let mut input = vec![];

    loop {
        let maybe_line = lines.next();

        if maybe_line.is_none() {
            break
        }
    }

    input
}

fn parse_input_line(line: &str) -> Vec<u64> {
    line.split(' ').map(|i| i.parse().unwrap()).collect()
}

#[test]
fn test_solution1() {
    let input_filename = "inputs/input_8_test.txt";
    let input = parse_input(input_filename);

    assert_eq!(solution1(input), 26);
}
