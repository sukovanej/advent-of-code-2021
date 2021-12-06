use std::fs::read_to_string;

pub fn solution1(input: &[u64], days: u64) -> u64 {
    let mut state = input.to_vec();
    let mut day = 1;

    while day <= days {
        for i in 0..state.len() {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }

        day += 1;
    }

    state.len() as u64
}

pub fn parse_input(file_name: &str) -> Vec<u64> {
    read_to_string(file_name)
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect()
}

#[test]
fn test_day6_solution1() {
    let input = vec![3, 4, 3, 1, 2];

    assert_eq!(solution1(&input, 80), 5934);
    // assert_eq!(solution1(&input, 256), 26984457539);
}
