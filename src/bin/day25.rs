const DATA: &str = include_str!("../inputs/day25.txt");

fn main() {
    let (one, two) = solve(DATA);
    println!("part 1: {}", one);
    println!("part 2: {}", two);
}

fn solve(data: &str) -> (usize, usize) {
    let one = data.len();
    let two = data.len();
    (one, two)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day25_sample.txt");

    #[test]
    fn sample() {
        assert_eq!(solve(SAMPLE_DATA), (0, 0));
    }

    #[test]
    fn case() {
        assert_eq!(solve(DATA), (0, 0));
    }
}
