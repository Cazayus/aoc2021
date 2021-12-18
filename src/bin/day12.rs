use std::collections::HashMap;
use std::iter;

const DATA: &str = include_str!("../inputs/day12.txt");
const MAX_CAVES: usize = 12;

struct Graph {
    start_id: usize,
    end_id: usize,
    is_small: [bool; MAX_CAVES],
    id_to_children: [Vec<usize>; MAX_CAVES],
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let graph = parse_data(data);
    count_path_to_end(&graph, [false; MAX_CAVES], graph.start_id, false)
}

fn part_two(data: &str) -> usize {
    let graph = parse_data(data);
    count_path_to_end(&graph, [false; MAX_CAVES], graph.start_id, true)
}

fn parse_data(data: &str) -> Graph {
    let mut is_small = [false; MAX_CAVES];
    let mut id_to_children: [Vec<usize>; MAX_CAVES] = Default::default();
    let caves_to_children = data
        .lines()
        .flat_map(|line| {
            let mut split_line = line.split('-');
            let before = split_line.next().unwrap();
            let after = split_line.next().unwrap();
            [(before, after), (after, before)]
        })
        .filter(|&(before, after)| before != "end" && after != "start")
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert_with(Vec::new).push(v);
            acc
        });
    let mut cave_to_id: HashMap<&str, usize> = HashMap::new();
    for (id, &cave) in caves_to_children
        .keys()
        .chain(iter::once(&"end"))
        .enumerate()
    {
        is_small[id] = cave.chars().all(char::is_lowercase);
        cave_to_id.insert(cave, id);
    }
    for (cave, children) in caves_to_children {
        id_to_children[cave_to_id[cave]] =
            children.into_iter().map(|name| cave_to_id[name]).collect()
    }
    Graph {
        start_id: cave_to_id["start"],
        end_id: cave_to_id["end"],
        is_small,
        id_to_children,
    }
}

fn count_path_to_end(
    graph: &Graph,
    mut visited: [bool; MAX_CAVES],
    child: usize,
    second_visit_allowed: bool,
) -> usize {
    if child == graph.end_id {
        return 1;
    }
    visited[child] = graph.is_small[child];
    graph.id_to_children[child]
        .iter()
        .filter(|&&child| second_visit_allowed || !visited[child])
        .map(|&child| {
            count_path_to_end(
                graph,
                visited,
                child,
                second_visit_allowed && !visited[child],
            )
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day12_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 226);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 3509);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 3738);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 120506);
    }
}
