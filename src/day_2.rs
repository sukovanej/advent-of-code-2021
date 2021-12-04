pub fn solution1(input: &str) -> i32 {
    let commands = read_input(input);
    let mut depth = 0;
    let mut horizontal_position = 0;

    for command in commands {
        match command.direction {
            Direction::Vertical => depth += command.value,
            Direction::Horizontal => horizontal_position += command.value,
        }
    }

    depth * horizontal_position
}

pub fn solution2(input: &str) -> i32 {
    let commands = read_input(input);
    let mut depth = 0;
    let mut horizontal_position = 0;
    let mut aim = 0;

    for command in commands {
        match command.direction {
            Direction::Vertical => {
                aim += command.value;
            }
            Direction::Horizontal => {
                horizontal_position += command.value;
                depth += command.value * aim;
                println!("depth increased by {} * {}", command.value, aim);
            }
        }
    }
    depth * horizontal_position
}

#[derive(Debug)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
struct Command {
    pub value: i32,
    pub direction: Direction,
}

impl Command {
    fn from_str(input: &str) -> Command {
        let split_line: Vec<&str> = input.split(' ').collect();
        let value = split_line[1].parse::<i32>().expect(&*format!(
            "Can't parse from line {}, {:?}",
            input, split_line
        ));

        let (direction, value) = match split_line[0] {
            "forward" => (Direction::Horizontal, value),
            "up" => (Direction::Vertical, -value),
            "down" => (Direction::Vertical, value),
            _ => panic!("unknown direction"),
        };

        Command { direction, value }
    }
}

fn read_input(input: &str) -> Vec<Command> {
    input.lines().map(Command::from_str).collect()
}

#[test]
fn test_day2_solution1() {
    let input_data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    assert_eq!(solution1(input_data), 150);
}

#[test]
fn test_day2_solution2() {
    let input_data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    assert_eq!(solution2(input_data), 900);
}
