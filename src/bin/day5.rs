use aoc2021::util;
use std::cmp;
use std::collections::HashMap;

struct Grid {
    content: HashMap<[i32; 2], i32>,
}

impl Grid {
    fn fill_grid(&mut self, input: &Vec<i32>) {
        if input.len() != 4 {
            panic!()
        }
        if input[0] == input[2] {
            // column
            let min = cmp::min(input[1], input[3]);
            let max = cmp::max(input[1], input[3]);
            for i in min..=max {
                let count = self.content.entry([input[0], i]).or_insert(0);
                *count += 1;
            }
        } else if input[1] == input[3] {
            // line
            let min = cmp::min(input[0], input[2]);
            let max = cmp::max(input[0], input[2]);
            for i in min..=max {
                let count = self.content.entry([i, input[1]]).or_insert(0);
                *count += 1;
            }
        }
    }
    fn fill_diagonal(&mut self, input: &Vec<i32>) {
        if input.len() != 4 {
            panic!()
        }
        if input[2] - input[0] == input[3] - input[1]
            || input[2] - input[0] == -(input[3] - input[1])
        {
            // a * x + b
            // a= (yb - ya) / (xb - xa) = 1 ou -1 pour une diagonale
            // b = ya - a * xa
            let x_min = cmp::min(input[0], input[2]);
            let x_max = cmp::max(input[0], input[2]);
            let a = (input[3] - input[1]) / (input[2] - input[0]);
            let b = input[1] - a * input[0];
            for x in x_min..=x_max {
                let coord = [x, a * x + b];
                let count = self.content.entry(coord).or_insert(0);
                *count += 1;
            }
        }
    }
}

fn main() {
    let input = util::read_lines("inputs/day5.txt");
    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|coord| coord.split(','))
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<Vec<i32>>) {
    let mut grid = Grid {
        content: HashMap::new(),
    };
    for line in lines {
        grid.fill_grid(line);
    }
    grid.content.retain(|_, &mut value| value > 1);
    println!("Solution is {}", grid.content.len());
}

fn part_two(lines: &Vec<Vec<i32>>) {
    let mut grid = Grid {
        content: HashMap::new(),
    };
    for line in lines {
        grid.fill_grid(line);
        grid.fill_diagonal(line);
    }
    grid.content.retain(|_, &mut value| value > 1);
    println!("Solution is {}", grid.content.len());
}
