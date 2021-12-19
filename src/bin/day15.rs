use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const DATA: &str = include_str!("../inputs/day15.txt");

type Point = (i32, i32);

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u32 {
    let graph = parse_data(data);
    dijkstra(&graph)
}

fn part_two(data: &str) -> u32 {
    let graph = parse_data(data);
    let (tile_width, tile_height) = graph.keys().max().map(|(x, y)| (x + 1, y + 1)).unwrap();
    let graph = graph
        .into_iter()
        .flat_map(|((x, y), risk)| {
            (0..5)
                .into_iter()
                .flat_map(|tile_x| (0..5).into_iter().map(move |tile_y| (tile_x, tile_y)))
                .map(move |(tile_x, tile_y)| {
                    (
                        (tile_x * tile_width + x, tile_y * tile_height + y),
                        (risk + tile_x as u32 + tile_y as u32 - 1) % 9 + 1,
                    )
                })
        })
        .collect();
    dijkstra(&graph)
}

fn parse_data(data: &str) -> HashMap<Point, u32> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn dijkstra(graph: &HashMap<Point, u32>) -> u32 {
    let start = (0, 0);
    let &goal = graph.keys().max().unwrap();
    let mut dists: HashMap<Point, u32> = HashMap::new();
    for &v in graph.keys() {
        dists.insert(v, u32::MAX);
    }
    let mut heap = BinaryHeap::from([(Reverse(0), start)]);
    while let Some((Reverse(cost), position)) = heap.pop() {
        if position == goal {
            return cost;
        }
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let edge = (position.0 + dx, position.1 + dy);
            if let Some(edge_cost) = graph.get(&edge) {
                if cost + edge_cost < dists[&edge] {
                    heap.push((Reverse(cost + edge_cost), edge));
                    dists.insert(edge, cost + edge_cost);
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day15_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 40);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 315);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 685);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 2995);
    }
}
