use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day20.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let (encoding, mut picture) = parse_data(data);
    enhance(encoding, &mut picture, 2);
    picture.values().filter(|&&bit| bit == 1).count()
}

fn part_two(data: &str) -> usize {
    let (encoding, mut picture) = parse_data(data);
    enhance(encoding, &mut picture, 50);
    picture.values().filter(|&&bit| bit == 1).count()
}

fn parse_data(data: &str) -> (Vec<usize>, HashMap<(isize, isize), usize>) {
    let encoding: Vec<usize> = data
        .lines()
        .take(1)
        .flat_map(|line| line.chars())
        .map(|char| if char == '#' { 1 } else { 0 })
        .collect();
    let mut picture = HashMap::new();
    for (y, line) in data.lines().skip(2).enumerate() {
        for (x, char) in line.chars().enumerate() {
            picture.insert((x as isize, y as isize), if char == '#' { 1 } else { 0 });
        }
    }
    (encoding, picture)
}

fn enhance(encoding: Vec<usize>, picture: &mut HashMap<(isize, isize), usize>, step: i32) {
    let offset: Vec<_> = (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .rev()
        .collect();
    let mut the_void = 0;
    let mut next_picture = HashMap::new();
    for _ in 0..step {
        let (x_min, y_min) = picture.keys().min().map(|(x, y)| (x - 1, y - 1)).unwrap();
        let (x_max, y_max) = picture.keys().max().map(|(x, y)| (x + 1, y + 1)).unwrap();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let code: usize = offset
                    .iter()
                    .enumerate()
                    .map(|(i, (dx, dy))| {
                        2_usize.pow(i as u32)
                            * (*picture.get(&(x + dx, y + dy)).unwrap_or(&the_void))
                    })
                    .sum();
                let value = encoding[code];
                next_picture.insert((x, y), value);
            }
        }
        *picture = next_picture.clone();
        the_void = if the_void == 0 {
            encoding[0]
        } else {
            *encoding.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day20_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 35);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 3351);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 5347);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 17172);
    }
}
