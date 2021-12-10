const DATA: &str = include_str!("../inputs/day7.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i32 {
    let positions: Vec<i32> = data
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|target| {
            positions
                .iter()
                .map(|element| (element - target).abs())
                .sum()
        })
        .min()
        .unwrap()
}

fn part_two(data: &str) -> i32 {
    let positions: Vec<i32> = data
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect();
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|target| {
            positions
                .iter()
                .map(|element| {
                    let nb_elem = (element - target).abs();
                    nb_elem * (nb_elem + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day7_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 37);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 168);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 344605);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 93699985);
    }
}
