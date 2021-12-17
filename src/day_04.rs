use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution1(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<Box<Board>> = boards.iter().map(|i| Box::new(i.clone())).collect();

    for &number in numbers.iter() {
        for board in boards.iter_mut() {
            let changed = board.play(number);

            if !changed || !board.is_winning() {
                continue;
            }

            let unmarked_numbers_sum: u32 = board.get_unmarked_numbers().iter().sum();
            return number * unmarked_numbers_sum;
        }
    }

    panic!("shouldn't never happen");
}

pub fn solution2(numbers: &[u32], boards: &[Board]) -> u32 {
    let mut boards: Vec<Box<Board>> = boards.iter().map(|i| Box::new(i.clone())).collect();
    let mut missing_boards = HashSet::new();
    missing_boards.extend(0..boards.len());

    for &number in numbers.iter() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if !missing_boards.contains(&board_idx) {
                continue;
            }

            let changed = board.play(number);

            if !changed || !board.is_winning() {
                continue;
            }

            missing_boards.remove(&board_idx);

            if missing_boards.is_empty() {
                let unmarked_numbers_sum: u32 = board.get_unmarked_numbers().iter().sum();
                return number * unmarked_numbers_sum;
            }
        }
    }

    panic!("shouldn't never happen");
}

static BOARD_SIZE: usize = 5;

#[derive(Clone)]
pub struct Board {
    pub numbers: HashSet<u32>,
    pub board: [[u32; 5]; 5],
    pub marked: [[bool; 5]; 5],
}

impl Board {
    /// Returns whether something changed
    pub fn play(&mut self, number: u32) -> bool {
        if !self.numbers.contains(&number) {
            return false;
        }

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == number {
                    self.marked[i][j] = true;
                    return true;
                }
            }
        }

        panic!("shouldn't never happen");
    }

    pub fn is_winning(&self) -> bool {
        for i in 0..BOARD_SIZE {
            let mut horizontal = true;
            let mut vertical = true;

            for j in 0..BOARD_SIZE {
                horizontal &= self.marked[i][j];
                vertical &= self.marked[j][i];
            }

            if vertical || horizontal {
                return true;
            }
        }

        false
    }

    pub fn get_unmarked_numbers(&self) -> Vec<u32> {
        let mut marked_numbers = vec![];

        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if !self.marked[i][j] {
                    marked_numbers.push(self.board[i][j]);
                }
            }
        }

        marked_numbers
    }

    fn new(input: &[Vec<u32>]) -> Self {
        let mut board = [[0; 5]; 5];
        let mut numbers = HashSet::new();
        let marked = [[false; 5]; 5];

        for (i, line) in input.iter().enumerate() {
            for (j, &value) in line.iter().enumerate() {
                board[i][j] = value;
                numbers.insert(value);
            }
        }

        Board {
            board,
            numbers,
            marked,
        }
    }
}

pub fn parse_input(file_name: &str) -> (Vec<u32>, Vec<Board>) {
    let file = File::open(file_name).expect("file not found");
    let mut lines = BufReader::new(file).lines();

    let numbers_line = lines
        .next()
        .expect("numbers line expected")
        .expect("numbers line expected");

    let numbers = numbers_line
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    let mut boards = vec![];

    loop {
        let empty_line = lines.next();

        if empty_line.is_none() {
            break;
        }

        let mut board_numbers = vec![];

        for _ in 0..BOARD_SIZE {
            let line_str = lines.next().unwrap().unwrap();
            board_numbers.push(
                line_str
                    .split_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }

        boards.push(Board::new(&board_numbers));
    }

    (numbers, boards)
}

#[test]
fn test_solution1() {
    let file_name = "./inputs/input_4_test.txt";
    let (numbers, boards) = parse_input(&file_name);

    assert_eq!(solution1(&numbers, &boards), 4512);
}

#[test]
fn test_solution2() {
    let file_name = "./inputs/input_4_test.txt";
    let (numbers, boards) = parse_input(&file_name);

    assert_eq!(solution2(&numbers, &boards), 1924);
}
