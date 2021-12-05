use aoc2021::util;

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
        for i in 0..5 {
            self.content[i + 5 * line_index] = (input[i], false);
        }
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
            .any(|line| line.iter().all(|&element| element.1 == true));
        let is_any_column_completed = (0..5)
            .map(|index_column| self.content.iter().skip(index_column).step_by(5))
            .any(|mut column| column.all(|element| element.1 == true));
        return is_any_line_completed || is_any_column_completed;
    }
    fn compute_value(&self, instruction: i32) -> i32 {
        let count: i32 = self
            .content
            .iter()
            .filter(|element| element.1 == false)
            .map(|element| element.0)
            .sum();
        return count * instruction;
    }
}

fn main() {
    let input = util::read_lines("inputs/day4.txt");
    let lines = input.lines().collect::<Vec<&str>>();
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
    part_one(&instructions, &mut boards);
    part_two(&instructions, &mut boards);
}

fn part_one(instructions: &Vec<i32>, boards: &mut Vec<Board>) {
    for &instruction in instructions {
        for board in boards.iter_mut() {
            board.set_true(instruction);
            if board.is_completed() {
                println!("Solution is {}", board.compute_value(instruction));
                return;
            }
        }
    }
}

fn part_two(instructions: &Vec<i32>, boards: &mut Vec<Board>) {
    for &instruction in instructions {
        for board in boards.iter_mut() {
            board.set_true(instruction);
        }
        if boards.len() == 1 {
            let board = boards.first().unwrap();
            if board.is_completed() {
                println!("Solution is {}", board.compute_value(instruction));
                return;
            }
        }
        boards.retain(|board| !board.is_completed());
    }
}
