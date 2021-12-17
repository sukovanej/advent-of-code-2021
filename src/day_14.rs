use std::collections::HashMap;

pub type Rule = (char, char, char);
type Template = HashMap<Tuple, u128>;
type Tuple = (char, char);
pub type Input = (Template, Vec<Rule>);

pub fn solution1(input: &Input) -> u128 {
    solution(input, 10)
}

pub fn solution2(input: &Input) -> u128 {
    solution(input, 40)
}

pub fn solution((template, rules): &Input, steps: u32) -> u128 {
    let mut template = template.clone();

    for _ in 0..steps {
        template = apply_rules(&template, rules);
    }

    let mut occurences = get_occurences(&template);
    occurences.sort_unstable();
    occurences.last().unwrap() - occurences.first().unwrap()
}

fn get_occurences(template: &Template) -> Vec<u128> {
    let mut occurences_map_1: HashMap<char, u128> = HashMap::new();
    let mut occurences_map_2: HashMap<char, u128> = HashMap::new();

    for ((ch1, ch2), &value) in template {
        *occurences_map_1.entry(*ch1).or_insert(0) += value;
        *occurences_map_2.entry(*ch2).or_insert(0) += value;
    }

    occurences_map_2.values().cloned().collect()
}

fn apply_rules(template: &Template, rules: &[Rule]) -> Template {
    let mut new_template = template.to_owned();

    for (from, to, rule) in rules {
        let (from, to, rule) = (*from, *to, *rule);

        if let Some(&entry) = template.get(&(from, to)) {
            if entry == 0 {
                continue;
            }

            *new_template.get_mut(&(from, to)).unwrap() -= entry;
            *new_template.entry((from, rule)).or_insert(0) += entry;
            *new_template.entry((rule, to)).or_insert(0) += entry;
        }
    }

    new_template
}

pub fn parse_input(input: &str) -> Input {
    let input_string = input.to_string();
    let mut input_iter = input_string.lines();
    let template_chars: Vec<char> = input_iter.next().unwrap().chars().collect();
    let mut template = Template::new();

    for ch in template_chars.windows(2) {
        assert_eq!(ch.len(), 2);
        let tuple = (ch[0], ch[1]);
        *template.entry(tuple).or_insert(0) += 1;
    }

    assert_eq!(input_iter.next().unwrap(), "");
    let rules = input_iter.map(parse_rule).collect();

    (template, rules)
}

fn parse_rule(input: &str) -> Rule {
    let split_input = input.split(" -> ").collect::<Vec<&str>>();
    assert_eq!(split_input.len(), 2);

    let (left, right) = (split_input[0], split_input[1]);
    assert_eq!(left.len(), 2);
    assert_eq!(right.len(), 1);

    let mut left_iter = left.chars();
    (
        left_iter.next().unwrap(),
        left_iter.next().unwrap(),
        right.to_string().chars().next().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_solution1() {
        let content = read_to_string("inputs/input_14_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 1588)
    }

    #[test]
    fn test_solution1_real_data() {
        let content = read_to_string("inputs/input_14_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 2360);
    }

    #[test]
    fn test_solution2() {
        let content = read_to_string("inputs/input_14_test.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 2188189693529);
    }

    #[test]
    fn test_1() {
        let input = Template::from([(('C', 'C'), 2)]);
        let expected = Template::from([(('C', 'A'), 2), (('A', 'C'), 2), (('C', 'C'), 0)]);
        let rules = &[('C', 'C', 'A'), ('A', 'C', 'A')];
        assert_eq!(apply_rules(&input, rules), expected);
    }

    #[test]
    fn test_solution2_real_data() {
        let content = read_to_string("inputs/input_14_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution2(&input), 2967977072188);
    }


    #[test]
    fn test_je_tam_tma() {
        let content = read_to_string("inputs/input_14_test.txt").unwrap();
        let (_, rules) = parse_input(&content);
        let input = Template::from([
            (('C', 'B'), 0),
            (('B', 'C'), 1),
            (('C', 'H'), 1),
            (('N', 'N'), 0),
            (('N', 'C'), 1),
            (('N', 'B'), 1),
            (('C', 'N'), 1),
            (('H', 'B'), 1),
        ]);
        let expected = Template::from([
            (('C', 'B'), 2),
            (('B', 'H'), 1),
            (('B', 'B'), 2),
            (('H', 'C'), 1),
            (('B', 'C'), 2),
            (('C', 'H'), 0),
            (('C', 'C'), 1),
            (('N', 'N'), 0),
            (('N', 'C'), 0),
            (('N', 'B'), 2),
            (('C', 'N'), 1),
            (('H', 'B'), 0),
        ]);
        assert_eq!(apply_rules(&input, &rules), expected);
    }
}
