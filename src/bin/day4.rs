const DATA: &str = include_str!("../inputs/day4.txt");

struct Board {
    content: [(i32, bool); 25],
}

impl Board {
    fn set_line(&mut self, input: &str, line_index: usize) {
        let input: Vec<i32> = input
            .split_whitespace()
            .map(|value| value.parse::<i32>().unwrap())
            .collect();
        if input.len() != 5 {
            panic!()
        }
        input
            .iter()
            .enumerate()
            .for_each(|(i, &value)| self.content[i + 5 * line_index] = (value, false))
    }
    fn set_true(&mut self, instruction: i32) {
        self.content.iter_mut().for_each(|mut content| {
            if content.0 == instruction {
                content.1 = true
            }
        });
    }
    fn is_completed(&self) -> bool {
        let is_any_line_completed = self
            .content
            .chunks(5)
            .any(|line| line.iter().all(|&element| element.1));
        let is_any_column_completed = (0..5)
            .map(|index_column| self.content.iter().skip(index_column).step_by(5))
            .any(|mut column| column.all(|element| element.1));
        is_any_line_completed || is_any_column_completed
    }
    fn compute_value(&self, instruction: i32) -> i32 {
        let count: i32 = self
            .content
            .iter()
            .map(|element| if element.1 { 0 } else { element.0 })
            .sum();
        count * instruction
    }
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i32 {
    let (instructions, mut boards) = parse_data(data);
    for instruction in instructions {
        for board in boards.iter_mut() {
            board.set_true(instruction);
            if board.is_completed() {
                return board.compute_value(instruction);
            }
        }
    }
    panic!()
}

fn part_two(data: &str) -> i32 {
    let (instructions, mut boards) = parse_data(data);
    for instruction in instructions {
        for board in boards.iter_mut() {
            board.set_true(instruction);
        }
        if boards.len() == 1 {
            let board = boards.first().unwrap();
            if board.is_completed() {
                return board.compute_value(instruction);
            }
        }
        boards.retain(|board| !board.is_completed());
    }
    panic!();
}

fn parse_data(data: &str) -> (Vec<i32>, Vec<Board>) {
    let lines = data.lines().collect::<Vec<_>>();
    let mut lines = lines.iter();
    let instructions: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    loop {
        let next = lines.next();
        if next != None {
            let next = next.unwrap();
            if next.is_empty() {
                let mut board = Board {
                    content: [(-1, false); 25],
                };
                for line_index in 0..5 {
                    board.set_line(lines.next().unwrap(), line_index);
                }
                boards.push(board)
            }
        } else {
            break;
        }
    }
    (instructions, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day4_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 4512);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 1924);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 71708);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 34726);
    }
}
