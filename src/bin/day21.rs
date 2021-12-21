const DATA: &str = include_str!("../inputs/day21.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let mut lines = data.lines();
    let mut pos = [
        lines.next().unwrap()[28..].parse().unwrap(),
        lines.next().unwrap()[28..].parse().unwrap(),
    ];
    let mut scores = [0, 0];

    for turn in 0.. {
        let player_index = turn % 2;
        let total_roll = (1..=3)
            .map(|i| wrap_around(3 * turn + i, 100))
            .sum::<usize>();
        pos[player_index] = wrap_around(pos[player_index] + total_roll, 10);
        scores[player_index] += pos[player_index];
        if scores[player_index] >= 1000 {
            return 3 * (turn + 1) * scores[(player_index + 1) % 2];
        }
    }
    unreachable!()
}

fn part_two(data: &str) -> usize {
    data.len()
}

fn wrap_around(value: usize, end_range: usize) -> usize {
    ((value - 1) % end_range) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day21_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 739785);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 3621);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 518418);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 10832);
    }
}
