use std::collections::VecDeque;
const DATA: &str = include_str!("../inputs/day6.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u64 {
    let mut colony: VecDeque<_> = [0u64; 9].iter().copied().collect();
    for fish in data.split(',') {
        colony[fish.parse().unwrap()] += 1;
    }
    advance_day(80, &mut colony);
    colony.iter().sum::<u64>()
}

fn part_two(data: &str) -> u64 {
    let mut colony: VecDeque<_> = [0u64; 9].iter().copied().collect();
    for fish in data.split(',') {
        colony[fish.parse().unwrap()] += 1;
    }
    advance_day(256, &mut colony);
    colony.iter().sum::<u64>()
}

fn advance_day(nb_day: i32, colony: &mut VecDeque<u64>) {
    for _ in 0..nb_day {
        let breeder = colony.pop_front().unwrap();
        colony[6] += breeder;
        colony.push_back(breeder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day6_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 5934);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 26984457539);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 380243);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 1708791884591);
    }
}
