use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day12.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let caves_to_neighbors = parse_data(data);
    count_path_to_end(&caves_to_neighbors, vec!["start"], "start", false)
}

fn part_two(data: &str) -> usize {
    let caves_to_neighbors = parse_data(data);
    count_path_to_end(&caves_to_neighbors, vec!["start"], "start", true)
}

fn parse_data(data: &str) -> HashMap<&str, Vec<&str>> {
    data.lines()
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
        })
}

fn count_path_to_end(
    caves_to_neighbors: &HashMap<&str, Vec<&str>>,
    current_path: Vec<&str>,
    child: &str,
    second_visit_allowed: bool,
) -> usize {
    if child == "end" {
        return 1;
    }
    caves_to_neighbors[child]
        .iter()
        .filter(|&child| second_visit_allowed || can_revisit(&current_path, child))
        .map(|child| {
            let mut vec1 = current_path.to_vec();
            vec1.push(child);
            (child, vec1)
        })
        .map(|(child, path_to_child)| {
            count_path_to_end(
                caves_to_neighbors,
                path_to_child,
                child,
                second_visit_allowed && can_revisit(&current_path, child),
            )
        })
        .sum()
}

fn can_revisit(current_path: &[&str], child: &str) -> bool {
    child.chars().all(|char| char.is_uppercase()) || !current_path.contains(&child)
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
