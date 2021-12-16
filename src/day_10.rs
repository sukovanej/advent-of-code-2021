pub type Input = Vec<Vec<char>>;
pub type InputRef<'a> = &'a [Vec<char>];

pub fn solution1(input: InputRef) -> u64 {
    input
        .iter()
        .filter_map(find_first_ilegal)
        .map(get_points)
        .sum()
}

pub fn solution2(input: InputRef) -> u64 {
    let mut scores: Vec<u64> = input
        .iter()
        .filter_map(get_missing_chars)
        .map(|missing_chars| missing_chars.iter().map(get_points_2).collect::<Vec<u64>>())
        .map(calculate_score)
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn calculate_score(scores: Vec<u64>) -> u64 {
    let mut score = 0;
    for s in scores {
        score = score * 5 + s;
    }
    score
}

fn get_missing_chars(line: &Vec<char>) -> Option<Vec<char>> {
    let mut stack: Vec<char> = Vec::new();

    for &ch in line {
        if is_open(ch) {
            stack.push(ch);
        } else if let Some(&top) = stack.last() {
            if is_matching(top, ch) {
                stack.pop();
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(stack.iter().rev().map(invert_char).collect())
}

fn invert_char(ch: &char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("je to v pici"),
    }
}

fn find_first_ilegal(line: &Vec<char>) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();

    for &ch in line {
        if is_open(ch) {
            stack.push(ch);
        } else if let Some(&top) = stack.last() {
            if is_matching(top, ch) {
                stack.pop();
            } else {
                return Some(ch);
            }
        } else {
            return Some(ch);
        }
    }

    None
}

fn is_open(ch: char) -> bool {
    ['(', '[', '{', '<'].contains(&ch)
}

fn is_matching(ch1: char, ch2: char) -> bool {
    [('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')].contains(&(ch1, ch2))
}

fn get_points_2(ch: &char) -> u64 {
    match ch {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("je to v pici"),
    }
}

fn get_points(ch: char) -> u64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("je to v pici"),
    }
}

pub fn parse_input(input: &str) -> Input {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let content = read_to_string("inputs/input_10_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 26397);
    }

    #[test]
    fn test_solution1_real_data() {
        let content = read_to_string("inputs/input_10_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 374061);
    }

    #[test]
    fn test_solution2() {
        let content = read_to_string("inputs/input_10_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 288957);
    }

    #[test]
    fn test_solution2_real_data() {
        let content = read_to_string("inputs/input_10_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 2116639949);
    }
}
