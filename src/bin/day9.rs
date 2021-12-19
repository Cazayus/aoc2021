use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day9.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u32 {
    let grid = parse_data(data);
    let low_points = find_low_points(&grid);
    low_points.iter().map(|coord| grid[coord] + 1).sum()
}

fn part_two(data: &str) -> usize {
    let grid = parse_data(data);
    let low_points = find_low_points(&grid);
    let basins: Vec<Vec<(i32, i32)>> = low_points
        .iter()
        .map(|coord| {
            let mut basin: Vec<(i32, i32)> = vec![];
            walk(&grid, coord, &mut basin);
            basin
        })
        .collect();
    let mut basins_size: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
    basins_size.sort_unstable();
    basins_size.iter().rev().take(3).product()
}

fn parse_data(data: &str) -> HashMap<(i32, i32), u32> {
    data.lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .enumerate()
                .map(|(j, value)| ((i as i32, j as i32), value))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_neighbors(coord: &(i32, i32)) -> Vec<(i32, i32)> {
    let i = coord.0;
    let j = coord.1;
    vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
}

fn find_low_points(grid: &HashMap<(i32, i32), u32>) -> Vec<(i32, i32)> {
    let mut low_points: Vec<(i32, i32)> = vec![];
    grid.iter().for_each(|(coord, &current)| {
        if find_neighbors(coord)
            .iter()
            .map(|neighbor_coord| grid.get(neighbor_coord).unwrap_or(&9))
            .all(|&value| value > current)
        {
            low_points.push(*coord);
        }
    });
    low_points
}

fn walk(grid: &HashMap<(i32, i32), u32>, coord: &(i32, i32), basin: &mut Vec<(i32, i32)>) {
    if !basin.contains(coord) && *grid.get(coord).unwrap_or(&9) < 9 {
        basin.push(*coord);
        find_neighbors(coord)
            .iter()
            .for_each(|neighbor_coord| walk(grid, neighbor_coord, basin));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day9_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 15);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 1134);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 570);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 899392);
    }
}
