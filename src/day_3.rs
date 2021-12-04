use std::cmp::Ordering;

pub type BitArray = Vec<bool>;

pub fn solution1(input: &[BitArray]) -> u32 {
    println!("{}", (0 << 1) + 1);
    println!("{}", bit_array_to_u32(&vec![true, true, false]));

    let mut weight_array: Vec<i32> = input[0].iter().map(|_| 0).collect();

    for line in input.iter().skip(1) {
        for (idx, &ch) in line.iter().enumerate() {
            weight_array[idx] += calculat_weight_diff(ch);
        }
    }

    let most_common_bit_array = weight_to_bit_array(weight_array);
    let least_common_bit_array = invert_bit_array(&most_common_bit_array);

    let most_common = bit_array_to_u32(&most_common_bit_array);
    let least_common = bit_array_to_u32(&least_common_bit_array);

    println!("{} {}", most_common, least_common);

    most_common * least_common
}

fn bit_array_to_u32(slice: &BitArray) -> u32 {
    slice.iter().fold(0, |acc, &b| (acc << 1) + b as u32)
}

fn calculat_weight_diff(value: bool) -> i32 {
    match value {
        true => -1,
        false => 1,
    }
}

fn invert_bit_array(xs: &BitArray) -> BitArray {
    xs.iter().map(|ch| !ch).collect()
}

fn weight_to_bit_array(input: Vec<i32>) -> BitArray {
    input
        .iter()
        .map(|&i| match i.cmp(&0) {
            Ordering::Greater => true,
            Ordering::Less => false,
            _ => panic!("cant decide the number"),
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
fn test_solution1_test() {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]
    .iter()
    .map(|&s| str_to_bit_array(s))
    .collect::<Vec<BitArray>>();

    assert_eq!(solution1(&input), 198);
}
