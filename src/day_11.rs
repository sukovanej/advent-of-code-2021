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

pub fn solution2(input: InputRef) -> u32 {
    let mut matrix = input.to_owned();

    matrix = run_step(&matrix);
    let mut step = 1;

    while !all_flashes(&matrix) {
        matrix = run_step(&matrix);
        step += 1;
    }

    step
}

fn all_flashes(matrix: InputRef) -> bool {
    matrix.iter().all(|line| line.iter().all(|&i| i == 0))
}

fn count_flashing_points(input: InputRef) -> u32 {
    input
        .iter()
        .map(|line| line.iter().filter(|&&i| i == 0).count() as u32)
        .sum()
}

fn get_neighbors(input: InputRef, x: usize, y: usize) -> Vec<(usize, usize)> {
    let (x, y) = (x as isize, y as isize);
    let (size_x, size_y) = (input[0].len() as isize, input.len() as isize);
    let directions: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    directions
        .map(|(dx, dy)| (x + dx, y + dy))
        .iter()
        .filter(|(x, y)| *x >= 0 && *x < size_x && *y >= 0 && *y < size_y)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect()
}

fn run_step(input: InputRef) -> Input {
    let mut matrix = input.to_owned();

    for line in matrix.iter_mut() {
        for v in line.iter_mut() {
            *v += 1;
        }
    }

    let mut previous_matrix = matrix;
    let mut new_matrix = explode(&previous_matrix);

    while new_matrix != previous_matrix {
        previous_matrix = new_matrix;
        new_matrix = explode(&previous_matrix);
    }

    new_matrix
}

fn explode(matrix: InputRef) -> Input {
    let mut matrix = matrix.to_owned();
    let (size_y, size_x) = (matrix.len(), matrix[0].len());

    for y in 0..size_y {
        for x in 0..size_x {
            if matrix[y][x] == 10 {
                matrix[y][x] = 0;
                let neighbors = get_neighbors(&matrix, x, y);
                for (u, v) in neighbors {
                    if matrix[v][u] != 0 && matrix[v][u] <= 9 {
                        matrix[v][u] += 1;
                    }
                }
            }
        }
    }

    // print_matrix(&matrix);
    // println!();

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
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let content = read_to_string("inputs/input_11_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 1656);
    }

    #[test]
    fn test_solution_real_data() {
        let content = read_to_string("inputs/input_11_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 1665);
    }

    #[test]
    fn test_solution2() {
        let content = read_to_string("inputs/input_11_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 195);
    }

    #[test]
    fn test_solution2_real_data() {
        let content = read_to_string("inputs/input_11_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 235);
    }

    #[test]
    fn test_step_1() {
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
