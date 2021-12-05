use std::fs::read_to_string;

type Point = (usize, usize);
type Vector = (Point, Point);
type Board = Vec<Vec<u32>>;

pub fn solution(input: &[Vector], check_diagonals: bool) -> u32 {
    let mut board = create_board(input);
    let mut visited_points = 0;

    for vector in input {
        if !check_diagonals && vector.0 .0 != vector.1 .0 && vector.0 .1 != vector.1 .1 {
            continue;
        }

        let points = get_all_points_on_vector(vector);

        for (x, y) in points {
            board[y][x] += 1;

            if board[y][x] == 2 {
                visited_points += 1;
            }
        }
    }

    visited_points
}

fn get_all_points_on_vector(vector: &Vector) -> Vec<Point> {
    let start = (vector.0 .0 as i32, vector.0 .1 as i32);
    let end = (vector.1 .0 as i32, vector.1 .1 as i32);

    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    let direction = ((end_x - start_x).signum(), (end_y - start_y).signum());
    let mut current = start;

    let mut points: Vec<Point> = vec![];

    while current != end {
        points.push((current.0.try_into().unwrap(), current.1.try_into().unwrap()));
        current = (current.0 + direction.0, current.1 + direction.1);
    }

    points.push((end.0.try_into().unwrap(), end.1.try_into().unwrap()));

    points
}

pub fn parse_input(file_name: &str) -> Vec<Vector> {
    let input = read_to_string(file_name).unwrap();
    let mut vectors = vec![];

    for line in input.lines() {
        let points: Vec<Vec<usize>> = line
            .split(" -> ")
            .map(|i| i.split(',').map(|i| i.parse().unwrap()).collect())
            .collect();

        vectors.push(((points[0][0], points[0][1]), (points[1][0], points[1][1])));
    }

    vectors
}

fn create_board(input: &[Vector]) -> Board {
    let mut max_x = 0;
    let mut max_y = 0;

    for ((x1, y1), (x2, y2)) in input {
        if x1 > &max_x {
            max_x = *x1;
        }

        if x2 > &max_x {
            max_x = *x2;
        }

        if y1 > &max_y {
            max_y = *y1;
        }
        if y2 > &max_y {
            max_y = *y2;
        }
    }

    let mut board = vec![];

    for _ in 0..max_y + 1 {
        board.push(vec![0; max_x + 1]);
    }

    board
}

#[test]
fn test_day5_solution1() {
    let file_name = "inputs/input_5_test.txt";
    let input = parse_input(file_name);

    assert_eq!(solution(&input, false), 5);
    assert_eq!(solution(&input, true), 12);
}
