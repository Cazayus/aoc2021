use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day11.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let mut grid = parse_data(data);
    advance_step(100, &mut grid)
}

fn part_two(data: &str) -> usize {
    let mut grid = parse_data(data);
    let mut nb_step = 0;
    loop {
        advance_step(1, &mut grid);
        nb_step += 1;
        if grid.values().all(|&value| value == 0) {
            return nb_step;
        }
    }
}

fn parse_data(data: &str) -> HashMap<(i32, i32), i32> {
    data.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .map(|number| number.to_digit(10).unwrap() as i32)
                .enumerate()
                .map(|(j, value)| ((i as i32, j as i32), value))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn advance_step(step: i32, grid: &mut HashMap<(i32, i32), i32>) -> usize {
    let mut flashes = 0;
    for _ in 0..step {
        grid.values_mut().for_each(|value| *value += 1);
        let mut flashing_octopuses: Vec<(i32, i32)> = find_flashing_octopuses(grid);
        while !flashing_octopuses.is_empty() {
            flashes += flashing_octopuses.len();
            flashing_octopuses
                .iter()
                .for_each(|coord| *grid.get_mut(coord).unwrap() = -1);
            flashing_octopuses.iter().for_each(|coord| {
                find_neighbors(coord).iter().for_each(|neighbors_cord| {
                    let neighbor_value = grid.get_mut(neighbors_cord);
                    if let Some(i) = neighbor_value {
                        if *i != -1 {
                            *i += 1;
                        }
                    }
                })
            });
            flashing_octopuses = find_flashing_octopuses(grid);
        }
        grid.values_mut().for_each(|value| {
            if *value == -1 {
                *value = 0
            }
        });
    }
    flashes
}

fn find_flashing_octopuses(grid: &HashMap<(i32, i32), i32>) -> Vec<(i32, i32)> {
    grid.iter()
        .filter_map(|(&coord, &value)| if value > 9 { Some(coord) } else { None })
        .collect()
}

fn find_neighbors(coord: &(i32, i32)) -> Vec<(i32, i32)> {
    let i = coord.0;
    let j = coord.1;
    vec![
        (i - 1, j - 1),
        (i, j - 1),
        (i + 1, j - 1),
        (i - 1, j),
        (i + 1, j),
        (i - 1, j + 1),
        (i, j + 1),
        (i + 1, j + 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day11_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 1656);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 195);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 1732);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 290);
    }
}
