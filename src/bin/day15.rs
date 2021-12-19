use std::collections::{HashMap, HashSet};

const DATA: &str = include_str!("../inputs/day15.txt");

type Point = (i32, i32);

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i32 {
    let graph = parse_data(data);
    dijkstra(&graph)
}

fn part_two(data: &str) -> usize {
    parse_data(data);
    5
}

fn parse_data(data: &str) -> HashMap<Point, i32> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap() as i32))
        })
        .collect()
}

// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue
fn dijkstra(graph: &HashMap<Point, i32>) -> i32 {
    let x_max = graph.keys().max_by_key(|&(x, _)| x).unwrap().0;
    let source = (0, 0);
    let target = (x_max, x_max);
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let mut dist: HashMap<Point, i32> = HashMap::new();
    let mut q: HashSet<Point> = HashSet::new();
    for &v in graph.keys() {
        dist.insert(v, i32::MAX);
        prev.insert(v, (-1, -1));
        q.insert(v);
    }
    dist.insert(source, 0);
    while !q.is_empty() {
        let u = *q
            .iter()
            .map(|point| (point, dist[&point]))
            .min_by_key(|(_, value)| *value)
            .unwrap()
            .0;
        q.remove(&u);
        if u == target {
            break;
        }
        for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let v = (u.0 + dx, u.1 + dy);
            if q.get(&v).is_some() {
                let alt = dist[&u] + i32::abs(graph[&u] + graph[&v]);
                if alt < dist[&v] {
                    dist.insert(v, alt);
                    prev.insert(v, u);
                }
            }
        }
    }
    let mut risk = 0;
    let mut u = target;

    while u != source {
        dbg!(u);
        u = prev[&u];
        risk += graph[&u];
    }
    dbg!(u);
    risk
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
        assert_eq!(part_two(SAMPLE_DATA), 61229);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 383);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 998900);
    }
}
