use std::fs::read_to_string;

pub fn solution1(depths: &[i32]) -> i32 {
    let mut increases = 0;
    let mut previous_depth = depths[0];

    for &depth in depths.iter().skip(1) {
        if depth > previous_depth {
            increases += 1;
        }

        previous_depth = depth;
    }

    increases
}

pub fn solution2(depths: &[i32]) -> i32 {
    let mut increases = 0;

    for (&left, &right) in depths.iter().zip(depths.iter().skip(3)) {
        if right > left {
            increases += 1;
        }
    }

    increases
}

pub fn read_numbers(file_name: &str) -> Vec<i32> {
    let data = read_to_string(file_name).expect("expected file");
    data.lines().filter_map(|line| line.parse().ok()).collect()
}

#[test]
fn test_solution1() {
    let files = vec!["./inputs/input_1_1.txt", "./inputs/input_1_test.txt"];
    let expected_results = vec![1791, 7];

    for (&filename, &expected_result) in files.iter().zip(expected_results.iter()) {
        let input = read_numbers(filename);
        let result = solution1(&input);
        assert_eq!(result, expected_result);
    }
}
