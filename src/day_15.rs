use std::collections::BTreeSet;

pub type Input = Vec<Vec<u32>>;
pub type InputRef<'a> = &'a [Vec<u32>];

pub fn solution1(input: InputRef) -> u32 {
    let (size_x, size_y) = (input[0].len(), input.len());
    let mut risks = vec![vec![u32::MAX; size_x]; size_y];

    risks[0][0] = 0;

    let mut visited = BTreeSet::new();
    let mut remainings = BTreeSet::new();

    remainings.insert((0, 0));

    while !remainings.is_empty() {
        if remainings.len() % 100 == 0 {
            println!("{} REMAINING", remainings.len());
        }
        let (x, y) = remainings.iter().min_by(|(a, b), (c, d)| risks[*b][*a].cmp(&risks[*d][*c])).unwrap();

        let (x, y) = (*x, *y);
        remainings.remove(&(x, y));
        visited.insert((x, y));

        if risks[y][x] == u32::MAX {
            panic!("choosen the infinite edge");
        }

        let neighbors = [
            (x as i32 - 1, y as i32),
            (x as i32 + 1, y as i32),
            (x as i32, y as i32 - 1),
            (x as i32, y as i32 + 1),
        ];

        for (nx, ny) in neighbors {
            if nx < 0 || nx >= size_x as i32 || ny < 0 || ny >= size_y as i32 {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            let new_risk = risks[y][x] + input[ny][nx];

            if new_risk < risks[ny][nx] {
                risks[ny][nx] = new_risk;

                if !visited.contains(&(nx, ny)) {
                    remainings.insert((nx, ny));
                }
            }
        }
    }

    let result = risks[size_y - 1][size_x - 1];

    let (mut x, mut y) = (size_x - 1, size_y - 2);

    while (x, y) != (0, 0) {
        let (a, b, c, d) = (
            risks[(x - 1).max(0)][y],
            risks[(x + 1).min(size_x - 1)][y],
            risks[x][(y - 1).max(0)],
            risks[x][(y + 1).min(size_y - 1)],
        );
    }

    for line in risks.iter() {
        for i in line {
            print!("{:6}", i);
        }
        println!();
    }

    result
}

pub fn solution2(input: InputRef) -> u32 {
    let input = multiple_input(input);
    solution1(&input)
}

fn multiple_input(input: InputRef) -> Input {
    let (size_x, size_y) = (input[0].len(), input.len());
    let mut result = vec![vec![0; size_x * 5]; size_x * 5];

    for y in 0..(size_y * 5) {
        for x in 0..(size_x * 5) {
            result[y][x] = (input[y % size_y][x % size_x] + (x / size_x) as u32 + (y / size_y) as u32);

            if result[y][x] > 9 {
                result[y][x] = result[y][x] % 10 + 1;
            }
        }
    }

    result
}

pub fn parse_input(content: &str) -> Input {
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| i.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::day_15::*;

    #[test]
    fn test_solution1() {
        let content = read_to_string("inputs/input_15_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 40);
    }

    #[test]
    fn test_solution1_real_data() {
        let content = read_to_string("inputs/input_15_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 423);
    }

    #[test]
    fn test_solution2() {
        let content = read_to_string("inputs/input_15_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 315);
    }

    #[test]
    fn test_solution2_real_data() {
        let content = read_to_string("inputs/input_15_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 2778);
    }
}
