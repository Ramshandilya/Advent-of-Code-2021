use std::fs;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn from_command_line(direction: &str, distance: i32) -> Command {
        match direction {
            "forward" => Command::Forward(distance),
            "down" => Command::Down(distance),
            "up" => Command::Up(distance),
            other => panic!("invalid direction: {}", other),
        }
    }

    fn position(&self) -> i32 {
        match *self {
            Command::Forward(val) => return val,
            _ => return 0
        }
    }

    fn depth(&self) -> i32 {
        match *self {
            Command::Up(val) => return -val,
            Command::Down(val) => return val,
            _ => return 0,
        }
    }
}

fn main() {
    let commands = read_file("day-02/input.txt");
    let result1 = part_one(&commands);
    let result2 = part_two(&commands);

    println!("Result = {}", result1);
    println!("Result = {}", result2);
}

fn read_file(filename: &str) -> Vec<Command> {
    let commands = fs::read_to_string(filename)
        .expect("File not found")
        .lines()
        .map(line_to_command)
        .collect();
    commands
}

fn line_to_command(line: &str) -> Command {
    let (direction, distance) = line.split_once(" ").unwrap();
    let distance = distance.parse::<i32>().unwrap();
    Command::from_command_line(direction, distance)
}

fn part_one(commands: &Vec<Command>) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        position += command.position();
        depth += command.depth();
    }

    return position * depth;
}

fn part_two(commands: &Vec<Command>) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        position += command.position();
        aim += command.depth();
        depth += command.position() * aim;

        // println!("Pos : {}\t depth: {}\t aim: {}", position, depth, aim);
    }

    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let commands = read_file("test_input.txt");
        let result = part_one(&commands);
        assert_eq!(result, 150)
    }

    #[test]
    fn test_part_two() {
        let commands = read_file("test_input.txt");
        let result = part_two(&commands);
        assert_eq!(result, 900)
    }
}