use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day14.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u64 {
    compute_iteration(data, 10)
}

fn part_two(data: &str) -> u64 {
    compute_iteration(data, 40)
}

fn compute_iteration(data: &str, nb_step: u32) -> u64 {
    let lines = data.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let polymer = lines[0];
    let mut pair_to_count: HashMap<[u8; 2], u64> = HashMap::new();
    polymer.windows(2).for_each(|window| {
        *pair_to_count
            .entry(<[u8; 2]>::try_from(window).unwrap())
            .or_insert(0) += 1
    });
    let instructions: HashMap<[u8; 2], u8> = lines[2..]
        .iter()
        .map(|&line| match *line {
            [a, b, .., c] => ([a, b], c),
            _ => unreachable!(),
        })
        .collect();
    for _ in 0..nb_step {
        let mut next_pair_to_count: HashMap<[u8; 2], u64> = HashMap::new();
        pair_to_count.iter().for_each(|(&pair, count)| {
            if let Some(i) = instructions.get(&pair) {
                *next_pair_to_count.entry([pair[0], *i]).or_insert(0) += count;
                *next_pair_to_count.entry([*i, pair[1]]).or_insert(0) += count;
            }
        });
        pair_to_count = next_pair_to_count;
    }
    let mut char_count: HashMap<u8, u64> = HashMap::new();
    pair_to_count
        .iter()
        .for_each(|(pair, count)| *char_count.entry(pair[0]).or_insert(0) += count);
    *char_count.entry(polymer[polymer.len() - 1]).or_insert(0) += 1;
    char_count.values().max().unwrap() - char_count.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day14_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 1588);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 2188189693529);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 2321);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 2399822193707);
    }
}
