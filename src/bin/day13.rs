use std::collections::HashSet;

const DATA: &str = include_str!("../inputs/day13.txt");

type Point = (u32, u32);

enum Instruction {
    X(u32),
    Y(u32),
}

fn main() {
    println!("part 1: {}", part_one(DATA));
}

fn part_one(data: &str) -> usize {
    let mut points: HashSet<Point> = HashSet::new();
    let mut instructions: Vec<Instruction> = vec![];
    for line in data.lines() {
        if line.starts_with("fold") {
            match &line[11..12] {
                "x" => instructions.push(Instruction::X(line[13..].parse().unwrap())),
                "y" => instructions.push(Instruction::Y(line[13..].parse().unwrap())),
                _ => unreachable!(),
            }
        } else if !line.is_empty() {
            let coord: Vec<u32> = line
                .split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect();
            points.insert((coord[0], coord[1]));
        }
    }
    let point_count: Vec<usize> = instructions
        .iter()
        .map(|instruction| {
            points = points
                .iter()
                .map(|&coord| match instruction {
                    Instruction::X(x) if coord.0 > *x => (2 * x - coord.0, coord.1),
                    Instruction::Y(y) if coord.1 > *y => (coord.0, 2 * y - coord.1),
                    _ => (coord.0, coord.1),
                })
                .collect();
            points.len()
        })
        .collect();
    print_grid(&points);
    point_count[0]
}

fn print_grid(points: &HashSet<Point>) {
    let x_max = points.iter().max_by_key(|&(x, _)| x).unwrap().0;
    let y_max = points.iter().max_by_key(|&(_, y)| y).unwrap().1;
    for y in 0..=y_max {
        for x in 0..=x_max {
            let point = points.get(&(x, y));
            print!("{}", if point.is_some() { "█" } else { " " });
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day13_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 17);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 781);
    }
}
