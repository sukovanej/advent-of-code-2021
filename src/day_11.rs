pub type Input = Vec<Vec<u32>>;
pub type InputRef<'a> = &'a [Vec<u32>];

pub fn solution1(input: InputRef) -> u32 {
    let mut matrix = input.to_owned();
    let mut number_of_flashes = 0;

    for _ in 0..100 {
        matrix = run_step(&matrix);
        number_of_flashes += count_flashing_points(&matrix);
    }

    number_of_flashes
}

fn count_flashing_points(input: InputRef) -> u32 {
    input
        .iter()
        .map(|line| line.iter().filter(|&&i| i == 0).count() as u32)
        .sum()
}

fn run_step(input: InputRef) -> Input {
    let mut matrix = input.to_owned();
    let (size_y, size_x) = (input.len(), input[0].len());

    for y in 0..size_y {
        for x in 0..size_x {
            matrix[y][x] += 1;

            if matrix[y][x] >= 9 {
                for j in (y as i32 - 1).max(0)..(y as i32 + 2).min(size_y as i32) {
                    for i in (x as i32 - 1).max(0)..(x as i32 + 2).min(size_x as i32) {
                        let (i, j) = (i as usize, j as usize);
                        if (i, j) == (x, y) {
                            continue;
                        }

                        matrix[j][i] += 1;
                    }
                }
            }
        }
    }

    for y in 0..size_y {
        for x in 0..size_x {
            if matrix[y][x] < 9 {
                matrix[y][x] += 1;
            }
        }
    }

    for y in 0..size_y {
        for x in 0..size_x {
            if matrix[y][x] > 9 {
                matrix[y][x] = 0;
            }
        }
    }

    matrix
}

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_11::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let content = read_to_string("inputs/input_11_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 26397);
    }

    #[test]
    fn test_step() {
        let input = "11111
19991
19191
19991
11111";
        let expected = "34543
40004
50005
40004
34543";
        let parsed_input = parse_input(input);
        let parsed_expected = parse_input(expected);
        assert_eq!(run_step(&parsed_input), parsed_expected);
    }

    #[test]
    fn test_step_2() {
        let input = "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";
        let expected = "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";
        let parsed_input = parse_input(input);
        let parsed_expected = parse_input(expected);
        assert_eq!(run_step(&parsed_input), parsed_expected);
    }
}
