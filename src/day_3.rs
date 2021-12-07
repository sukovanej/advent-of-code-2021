use std::cmp::Ordering;

pub type BitArray = Vec<bool>;

pub fn solution1(input: &[BitArray]) -> u32 {
    let most_common_bit_array = find_most_common(input);
    let least_common_bit_array = invert_bit_array(&most_common_bit_array);
    bit_array_to_u32(&most_common_bit_array) * bit_array_to_u32(&least_common_bit_array)
}

fn find_rating(input: &[BitArray], find_fn: Box<dyn Fn(&[BitArray], usize) -> bool>) -> u32 {
    let mut remaining: Vec<BitArray> = input.to_vec();
    let mut new_remaining = vec![];

    for bit_idx in 0..input[0].len() {
        let most_common_bit = find_fn(&remaining, bit_idx);

        for bit_array in remaining.iter() {
            if bit_array[bit_idx] == most_common_bit {
                new_remaining.push(bit_array.clone());
            }
        }

        if new_remaining.len() == 1 {
            return bit_array_to_u32(&new_remaining[0]);
        }

        remaining = new_remaining.clone();
        new_remaining = vec![];
    }

    panic!("wtf bro");
}

pub fn solution2(input: &[BitArray]) -> u32 {
    let oxigin_generator_rating = find_rating(input, Box::new(find_most_common_on_position));
    let co2_scrubber_rating = find_rating(input, Box::new(find_least_common_on_position));

    oxigin_generator_rating * co2_scrubber_rating
}

fn find_least_common_on_position(input: &[BitArray], idx: usize) -> bool {
    let mut v = 0;

    for bit_array in input {
        v += match bit_array[idx] {
            true => 1,
            false => -1,
        };
    }

    v < 0
}

fn find_most_common_on_position(input: &[BitArray], idx: usize) -> bool {
    !find_least_common_on_position(input, idx)
}

fn find_most_common(input: &[BitArray]) -> BitArray {
    let mut weight_array: Vec<i32> = input[0].iter().map(|_| 0).collect();

    for line in input.iter().skip(1) {
        for (idx, &ch) in line.iter().enumerate() {
            weight_array[idx] += calculat_weight_diff(ch);
        }
    }

    weight_to_bit_array(&weight_array)
}

fn bit_array_to_u32(slice: &[bool]) -> u32 {
    slice.iter().fold(0, |acc, &b| (acc << 1) + b as u32)
}

fn calculat_weight_diff(value: bool) -> i32 {
    match value {
        true => 1,
        false => -1,
    }
}

fn invert_bit_array(xs: &[bool]) -> BitArray {
    xs.iter().map(|&i| !i).collect()
}

fn weight_to_bit_array(input: &[i32]) -> BitArray {
    input
        .iter()
        .map(|&i| match i.cmp(&0) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => false,
        })
        .collect()
}

pub fn str_to_bit_array(input: &str) -> BitArray {
    return input
        .chars()
        .map(|ch| match ch {
            '1' => true,
            '0' => false,
            _ => panic!("Expected 0 or 1"),
        })
        .collect();
}

#[test]
fn test_solution1() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]
    .iter()
    .map(|&s| str_to_bit_array(s))
    .collect::<Vec<BitArray>>();

    assert_eq!(solution1(&input), 198);
}

#[test]
fn test_solution2() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]
    .iter()
    .map(|&s| str_to_bit_array(s))
    .collect::<Vec<BitArray>>();

    assert_eq!(solution2(&input), 230);
}
