use std::collections::HashSet;
use std::fs::read_to_string;

/// 0 -> 6
/// 1 -> 2
/// 2 -> 5
/// 3 -> 5
/// 4 -> 4
/// 5 -> 5
/// 6 -> 6
/// 7 -> 3
/// 8 -> 7
/// 9 -> 6

pub type Input = Vec<(Vec<SegmentType>, Vec<SegmentType>)>;
pub type InputRef<'a> = &'a [(Vec<SegmentType>, Vec<SegmentType>)];

pub fn solution1(input: InputRef) -> u64 {
    let unique_lengs = vec![2, 4, 3, 7];
    input
        .iter()
        .map(|line| {
            line.1
                .iter()
                .filter(|&i| unique_lengs.contains(&(i.len() as u64)))
                .count() as u64
        })
        .sum()
}

fn parse_segments_output(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|x| x.split(" | ").last().unwrap())
        .flat_map(|x| x.split_whitespace().map(|y| y.to_string()))
        .collect()
}

pub fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|x| {
            x.split(" | ")
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
        })
        .map(|x| {
            vec_to_tuple(
                &x.into_iter()
                    .map(|j| {
                        j.split_whitespace()
                            .map(|y| y.chars().collect::<SegmentType>())
                            .collect::<Vec<SegmentType>>()
                    })
                    .collect::<Vec<Vec<SegmentType>>>(),
            )
        })
        .collect()
}

fn vec_to_tuple<T: Clone>(xs: &[Vec<T>]) -> (Vec<T>, Vec<T>) {
    assert_eq!(xs.len(), 2);
    (xs[0].clone(), xs[1].clone())
}

pub fn solution2(input: InputRef) -> u32 {
    input.iter().map(decode_input_line).sum()
}

fn decode_input_line((noise, output): &(Vec<SegmentType>, Vec<SegmentType>)) -> u32 {
    let all_segments = noise
        .iter()
        .chain(output)
        .map(|i| i.to_owned())
        .collect::<Vec<SegmentType>>();
    decode(&all_segments, output)
}

type SegmentType = HashSet<char>;

fn decode(all_segments: &[SegmentType], output: &[SegmentType]) -> u32 {
    let uniques = find_uniques(all_segments);
    output
        .iter()
        .map(|segment| decode_segment(&uniques, segment).to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap()
}

fn intersect_segments(left: &SegmentType, right: &SegmentType) -> SegmentType {
    left.intersection(right).copied().collect::<SegmentType>()
}

fn decode_segment(uniques: &[SegmentType; 3], segment: &SegmentType) -> u32 {
    let [one_segment, four_segment, seven_segment] = uniques;

    match segment.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if intersect_segments(segment, one_segment) == *one_segment {
                3
            } else if intersect_segments(segment, four_segment).len() == 3 {
                5
            } else {
                2
            }
        }
        6 => {
            if intersect_segments(segment, seven_segment) != *seven_segment {
                6
            } else if intersect_segments(segment, four_segment) == *four_segment {
                9
            } else {
                0
            }
        }
        7 => 8,
        _ => panic!("je to v pici"),
    }
}

fn find_uniques(all_segments: &[HashSet<char>]) -> [SegmentType; 3] {
    let (mut one, mut four, mut seven) = (None, None, None);

    for segment in all_segments {
        match segment.len() {
            2 => one = Some(segment.clone()),
            4 => four = Some(segment.clone()),
            3 => seven = Some(segment.clone()),
            _ => continue,
        }
    }

    [one, four, seven].map(|i| i.unwrap())
}

#[test]
fn test_solution2() {
    let test_input = read_to_string("input/input_8_test.txt").unwrap();
    let parsed_input = parse_input(&test_input);
    assert_eq!(solution2(&parsed_input), 61229);

    let test_input = read_to_string("input/input_8_1.txt").unwrap();
    let parsed_input = parse_input(&test_input);
    assert_eq!(solution2(&parsed_input), 1007675);
}

#[test]
fn test_solution1_test() {
    let input_filename = "inputs/input_8_test.txt";
    let input = parse_input(input_filename);

    assert_eq!(solution1(&input), 26);
}

#[test]
fn test_solution1() {
    let input_filename = "inputs/input_8_1.txt";
    let input = parse_input(input_filename);

    assert_eq!(solution1(&input), 473);
}

#[test]
fn test_solution2_test() {
    let input_filename = "inputs/input_8_1.txt";
    let input = parse_input(input_filename);

    assert_eq!(solution1(&input), 5353);
}
