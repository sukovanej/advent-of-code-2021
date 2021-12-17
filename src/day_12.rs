use std::collections::HashMap;

pub type Connections = HashMap<String, String>;

pub fn solution1(input: &Connections) -> u32 {
    todo!()
}

pub fn parse_input(input: &str) -> Connections {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_solution1_1() {
        let content = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 10);
    }

    #[test]
    fn test_solution1_2() {
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
    }

    #[test]
    fn test_solution1_3() {
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
        assert_eq!(solution1(&input), 19);
    }

    #[test]
    fn test_solution1_real_data() {
        let content = read_to_string("inputs/input_12_1.txt").unwrap();
        let input = parse_input(&content);
        assert_eq!(solution1(&input), 19);
    }

}
