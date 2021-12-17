use std::{collections::HashSet, fs::read_to_string};

type Input = Vec<Vec<u32>>;
type InputRef<'a> = &'a [Vec<u32>];

pub fn solution1(input: InputRef) -> u32 {
    let mut low_points = vec![];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let adjecents = get_adjacent(input, x, y);
            let value = input[y][x];

            if adjecents.iter().all(|(x, y)| input[*y][*x] > value) {
                low_points.push(value);
            }
        }
    }

    low_points.len() as u32 + low_points.iter().sum::<u32>()
}

pub fn solution2(input: InputRef) -> u32 {
    let mut remaining = HashSet::new();
    let mut basins = vec![];

    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] != 9 {
                remaining.insert((x, y));
            }
        }
    }

    while !remaining.is_empty() {
        let item = remaining.iter().next().unwrap();
        let adjecents = get_adjacent(input, item.0, item.1);
        break;
    }

    basins.sort_unstable();
    basins.reverse();
    basins.iter().take(4).product()
}

fn get_adjacent(input: InputRef, x: usize, y: usize) -> Vec<(usize, usize)> {
    let (max_x, max_y) = (input[0].len() - 1, input.len() - 1);
    let pos = (x, y);

    if pos == (0, 0) {
        vec![(x + 1, y), (x, y + 1)]
    } else if pos == (max_x, max_y) {
        vec![(x - 1, y), (x, y - 1)]
    } else if pos == (0, max_y) {
        vec![(x, y - 1), (x + 1, y)]
    } else if pos == (max_x, 0) {
        vec![(x - 1, y), (x, y + 1)]
    } else if x == 0 {
        vec![(x + 1, y), (x, y + 1), (x, y - 1)]
    } else if x == max_x {
        vec![(x - 1, y), (x, y + 1), (x, y - 1)]
    } else if y == 0 {
        vec![(x - 1, y), (x + 1, y), (x, y + 1)]
    } else if y == max_y {
        vec![(x - 1, y), (x + 1, y), (x, y - 1)]
    } else {
        vec![(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)]
    }
}

pub fn parse_input(path: &str) -> Input {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect()
}

#[test]
fn test_solution1() {
    let input = parse_input("inputs/input_9_test.txt");
    assert_eq!(solution1(&input), 15);
}

#[test]
fn test_solution2() {
    let input = parse_input("inputs/input_9_test.txt");
    assert_eq!(solution2(&input), 1134);
}
