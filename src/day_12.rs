use std::collections::{HashMap, HashSet};

pub type Connections = HashMap<String, HashSet<String>>;

pub fn solution1(map: &Connections) -> usize {
    let paths = get_paths(
        vec!["start".to_string()],
        map,
        HashSet::from(["start".to_string()]),
    );
    paths.len()
}

pub fn solution2(map: &Connections) -> usize {
    let paths = get_paths_2(vec!["start".to_string()], map, HashMap::new());

    paths.len()
}

fn get_paths(
    current_path: Vec<String>,
    map: &Connections,
    visited_small: HashSet<String>,
) -> HashSet<Vec<String>> {
    let current_node = current_path.last().unwrap().to_owned();
    let next_possible_nodes = map[&current_node]
        .iter()
        .filter(|&i| !visited_small.contains(i))
        .collect::<HashSet<&String>>();

    let mut paths = HashSet::new();

    for next in next_possible_nodes {
        let mut new_path = current_path.clone();
        new_path.push(next.clone());

        if next == "end" {
            paths.insert(new_path);
            continue;
        }

        let mut new_visited_small = visited_small.clone();

        if is_small(next) {
            new_visited_small.insert(next.to_string());
        }

        let new_paths = get_paths(new_path, map, new_visited_small);
        paths.extend(new_paths);
    }

    paths
}

fn get_paths_2(
    current_path: Vec<String>,
    map: &Connections,
    visited_small: HashMap<String, u8>,
) -> HashSet<Vec<String>> {
    let current_node = current_path.last().unwrap().to_owned();
    let single_cave_visited_used_limit = visited_small.values().any(|&i| i == 2);
    let next_possible_nodes = map[&current_node]
        .iter()
        .filter(|&i| {
            (!visited_small.contains_key(i) || !single_cave_visited_used_limit) && i != "start"
        })
        .collect::<HashSet<&String>>();

    let mut paths = HashSet::new();

    for next in next_possible_nodes {
        let mut new_path = current_path.clone();
        new_path.push(next.clone());

        if next == "end" {
            paths.insert(new_path);
            continue;
        }

        let mut new_visited_small = visited_small.clone();

        if is_small(next) {
            *new_visited_small.entry(next.clone()).or_insert(0) += 1;
        }

        let new_paths = get_paths_2(new_path, map, new_visited_small);
        paths.extend(new_paths);
    }

    paths
}

fn is_small(value: &str) -> bool {
    value.chars().all(|i| i.is_lowercase())
}

pub fn parse_input(input: &str) -> Connections {
    input
        .lines()
        .map(|line| {
            let split = line.split('-').collect::<Vec<&str>>();
            (split[0].to_owned(), split[1].to_owned())
        })
        .fold(HashMap::new(), |mut acc, (from, to)| {
            acc.entry(from.to_owned())
                .or_insert_with(HashSet::new)
                .insert(to.to_owned());
            acc.entry(to).or_insert_with(HashSet::new).insert(from);
            acc
        })
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_solution_1() {
        let content = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 10);
        assert_eq!(solution2(&input), 36);
    }

    #[test]
    fn test_solution_2() {
        let content = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 19);
        assert_eq!(solution2(&input), 103);
    }

    #[test]
    fn test_solution_3() {
        let content = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 226);
        assert_eq!(solution2(&input), 3509);
    }

    #[test]
    fn test_solution_real_data() {
        let content = read_to_string("inputs/input_12_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 3000);

        // TODO: is unefficient
        // assert_eq!(solution2(&input), 74222);
    }
}
