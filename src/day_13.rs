pub enum Axis {
    X,
    Y,
}
pub type Point = (usize, usize);
pub type PaperDots = Vec<Point>;
type PaperDotsRef<'a> = &'a [Point];
pub type Input = (PaperDots, Vec<FoldAlong>);
pub type FoldAlong = (Axis, usize);

type Matrix = Vec<Vec<bool>>;
type MatrixRef<'a> = &'a [Vec<bool>];

pub fn solution1(input: &Input) -> u32 {
    let (dots, folds) = input;
    let matrix = create_matrix(dots);
    number_of_dots(&fold_along(&matrix, &folds[0]))
}

pub fn solution2(input: &Input) -> u32 {
    let (dots, folds) = input;
    let mut matrix = create_matrix(dots);

    for fold in folds {
        matrix = fold_along(&matrix, fold);
    }

    print_matrix(&matrix);
    number_of_dots(&matrix)
}

fn print_matrix(matrix: MatrixRef) {
    for line in matrix.iter() {
        for &x in line {
            if x {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn number_of_dots(matrix: MatrixRef) -> u32 {
    matrix
        .iter()
        .map(|line| line.iter().filter(|&&i| i).count() as u32)
        .sum()
}

fn create_matrix(dots: PaperDotsRef) -> Matrix {
    let max_x = dots.iter().map(|(x, _)| *x).max().unwrap() as usize;
    let max_y = dots.iter().map(|(_, y)| *y).max().unwrap() as usize;

    let mut matrix = vec![vec![false; max_x + 1]; max_y + 1];

    for (x, y) in dots {
        matrix[*y][*x] = true;
    }

    matrix
}

fn rotate(matrix: MatrixRef, clockwise: bool) -> Matrix {
    let size_x = matrix[0].len();
    let size_y = matrix.len();

    let mut new_matrix = vec![vec![false; size_y]; size_x];
    let (new_size_x, new_size_y) = (new_matrix[0].len(), new_matrix.len());

    for x in 0..new_size_x {
        for y in 0..new_size_y {
            if clockwise {
                new_matrix[y][x] = matrix[x][size_x - y - 1];
            } else {
                new_matrix[y][x] = matrix[size_y - x - 1][y];
            }
        }
    }

    new_matrix
}

fn fold_along(matrix: MatrixRef, (axis, value): &FoldAlong) -> Matrix {
    match axis {
        Axis::X => rotate(&fold_along_y(&rotate(matrix, false), *value), true),
        Axis::Y => fold_along_y(matrix, *value),
    }
}

fn fold_along_y(matrix: MatrixRef, value: usize) -> Matrix {
    if value < matrix.len() / 2 {
        return mirror_by_y(&_fold_along_y(&mirror_by_y(matrix), matrix.len() - value));
    }

    _fold_along_y(matrix, value)
}

fn _fold_along_y(matrix: MatrixRef, value: usize) -> Matrix {
    assert!(value >= matrix.len() / 2);

    let mut new_matrix = matrix.iter().take(value).cloned().collect::<Matrix>();
    let size_y = new_matrix.len();

    for (y, line) in matrix.iter().skip(value + 1).enumerate() {
        for (x, value) in line.iter().enumerate() {
            new_matrix[size_y - y - 1][x] |= value;
        }
    }

    new_matrix
}

fn mirror_by_y(matrix: MatrixRef) -> Matrix {
    matrix.iter().rev().cloned().collect::<Matrix>()
}

pub fn parse_input(input: &str) -> Input {
    let mut split_input = input.split("\n\n");
    let points_input = split_input.next().unwrap();
    let folds_input = split_input.next().unwrap();

    assert!(split_input.next().is_none());

    let points = points_input
        .lines()
        .map(|line| {
            vec_to_tuple(
                &line
                    .split(',')
                    .map(|i| i.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();
    let folds = folds_input.lines().map(parse_fold_input).collect();

    (points, folds)
}

fn parse_fold_input(input: &str) -> FoldAlong {
    let axis = match input.chars().nth(11).unwrap() {
        'x' => Axis::X,
        'y' => Axis::Y,
        _ => panic!("wtf broh, je to pici"),
    };
    let value = input.chars().skip(13).collect::<String>().parse().unwrap();

    (axis, value)
}

fn vec_to_tuple<T: Clone>(xs: &[T]) -> (T, T) {
    assert_eq!(xs.len(), 2);
    (xs[0].clone(), xs[1].clone())
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use crate::day_13::*;

    #[test]
    fn test_solution1() {
        let test_input = read_to_string("inputs/input_13_test.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution1(&parsed_input), 17);
    }

    #[test]
    fn test_solution1_2() {
        let test_input = read_to_string("inputs/input_13_test_2.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution1(&parsed_input), 17);
    }

    #[test]
    fn test_solution2() {
        let test_input = read_to_string("inputs/input_13_test.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution2(&parsed_input), 16);
    }

    #[test]
    fn test_solution2_2() {
        let test_input = read_to_string("inputs/input_13_1.txt").unwrap();
        let parsed_input = parse_input(&test_input);
        assert_eq!(solution2(&parsed_input), 104);
    }
}