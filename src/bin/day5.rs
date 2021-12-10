use std::cmp;
use std::collections::HashMap;
use std::iter::{repeat, Chain};
use std::ops::RangeInclusive;

const DATA: &str = include_str!("../inputs/day5.txt");

struct Grid {
    content: HashMap<[i32; 2], i32>,
}

// we don't know which one is higher so we iterate both ways
fn route1d(start: i32, stop: i32) -> Chain<RangeInclusive<i32>, RangeInclusive<i32>> {
    (start..=stop).chain(stop..=start)
}

impl Grid {
    fn fill_grid(&mut self, input: &[i32]) {
        if input.len() != 4 {
            panic!()
        }
        if input[0] == input[2] {
            // column
            repeat(input[0])
                .zip(route1d(input[1], input[3]))
                .for_each(|(x, y)| *self.content.entry([x, y]).or_insert(0) += 1);
        } else if input[1] == input[3] {
            // line
            route1d(input[0], input[2])
                .zip(repeat(input[1]))
                .for_each(|(x, y)| *self.content.entry([x, y]).or_insert(0) += 1);
        }
    }
    fn fill_diagonal(&mut self, input: &[i32]) {
        if input.len() != 4 {
            panic!()
        }
        if input[2] - input[0] == input[3] - input[1]
            || input[2] - input[0] == -(input[3] - input[1])
        {
            // a * x + b
            // a = (yb - ya) / (xb - xa) = 1 ou -1 pour une diagonale
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
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn parse_data(data: &str) -> (Vec<Vec<i32>>, Grid) {
    let lines: Vec<Vec<i32>> = data
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|coord| coord.split(','))
                .map(|value| value.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let grid = Grid {
        content: HashMap::new(),
    };
    (lines, grid)
}

fn part_one(data: &str) -> usize {
    let (lines, mut grid) = parse_data(data);
    for line in lines {
        grid.fill_grid(&line);
    }
    grid.content.retain(|_, &mut value| value > 1);
    grid.content.len()
}

fn part_two(data: &str) -> usize {
    let (lines, mut grid) = parse_data(data);
    for line in lines {
        grid.fill_grid(&line);
        grid.fill_diagonal(&line);
    }
    grid.content.retain(|_, &mut value| value > 1);
    grid.content.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day5_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 5);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 12);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 4655);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 20500);
    }
}
