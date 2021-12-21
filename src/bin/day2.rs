use crate::Command::{Down, Forward, Up};

const DATA: &str = include_str!("../inputs/day2.txt");

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}
impl From<&str> for Command {
    fn from(line: &str) -> Self {
        let split_instruction: Vec<&str> = line.split_whitespace().collect();
        let (direction, amount) = (split_instruction[0], split_instruction[1]);
        let n = amount.parse().unwrap();
        match direction {
            "forward" => Forward(n),
            "down" => Down(n),
            "up" => Up(n),
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;

    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => horizontal += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }
    horizontal * depth
}

fn part_two(data: &str) -> i32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => {
                horizontal += n;
                depth += aim * n
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day2_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 150);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 900);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 1694130);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 1698850445);
    }
}
