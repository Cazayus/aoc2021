const DATA: &str = include_str!("../inputs/day1.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part_two(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .windows(4)
        .filter(|&w| w[0] < w[3])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day1_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 7);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 5);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 1715);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 1739);
    }
}
