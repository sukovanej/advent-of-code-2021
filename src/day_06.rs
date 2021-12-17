use std::fs::read_to_string;

pub fn solution(input: &[usize], days: usize) -> u64 {
    let mut state = [0; 9];

    for &i in input {
        state[i] += 1;
    }

    for _ in 0..days {
        let zeros = state[0];

        for i in 1..9 {
            state[i - 1] = state[i];
        }
        state[8] = zeros;
        state[6] += zeros;
    }

    state.iter().sum()
}

pub fn parse_input(file_name: &str) -> Vec<usize> {
    read_to_string(file_name)
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

#[test]
fn test_solution() {
    let input = vec![3, 4, 3, 1, 2];

    assert_eq!(solution(&input, 80), 5934);
    assert_eq!(solution(&input, 256), 26984457539);
}
