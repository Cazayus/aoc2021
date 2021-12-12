use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day8.txt");

//  Combination list:
//  0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// 0 : 6 segments
// 1 : 2 segments !!
// 2 : 5 segments
// 3 : 5 segments
// 4 : 4 segments !!
// 5 : 5 segments
// 6 : 6 segments
// 7 : 3 segments !!
// 8 : 7 segments !!
// 9 : 6 segments

// Les faciles :
// Celui à 2 segments est 1
// Celui à 4 segments est 4
// Celui à 3 segments est 7
// Celui à 7 segments est 8

// Les 6 segments (0, 6, 9):
// celui à 6 segments qui contient 4 est 9
// celui à 6 segments qui ne contient pas 7 est 6
// le dernier 6 est 0

// Les 5 segments (2, 3, 5):
// celui à 5 segments qui contient 1 est 3
// celui à 5 segments qui est contenu dans 6 est 5
// le dernier 5 est 2

// autre solution implémentée ici
// en se basant sur https://www.reddit.com/r/adventofcode/comments/rc5s3z/2021_day_8_part_2_a_simple_fast_and_deterministic/

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn compute_char_occurrences_count(normal_digits: &str) -> HashMap<char, i32> {
    let mut char_occurrence_count: HashMap<char, i32> = HashMap::new();
    normal_digits.split_whitespace().for_each(|digit| {
        digit
            .chars()
            .for_each(|char| *char_occurrence_count.entry(char).or_insert(0) += 1)
    });
    char_occurrence_count
}

fn digit_hash(
    normal_digits: &str,
    char_occurrence_count: &HashMap<char, i32>,
) -> HashMap<i32, usize> {
    normal_digits
        .split_whitespace()
        .map(|digit| {
            digit
                .chars()
                .map(|char| char_occurrence_count.get(&char).unwrap())
                .sum::<i32>()
        })
        .enumerate()
        .map(|(index, hash)| (hash, index))
        .collect()
}

fn parse_data(data: &str) -> Vec<Vec<usize>> {
    let normal_digits = "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg";
    let normal_digit_hash = digit_hash(
        normal_digits,
        &compute_char_occurrences_count(normal_digits),
    );
    data.lines()
        .map(|line| line.split(" | "))
        .map(|mut split_line| {
            let before = split_line.next().unwrap();
            let before_char_occurrences_count = compute_char_occurrences_count(before);
            let after = split_line.next().unwrap();
            after
                .split_whitespace()
                .map(|digit| {
                    let digit_hash = digit
                        .chars()
                        .map(|char| *before_char_occurrences_count.get(&char).unwrap())
                        .sum();
                    *normal_digit_hash.get(&digit_hash).unwrap()
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn part_one(data: &str) -> usize {
    let decrypted_digits_after_pipe = parse_data(data);
    decrypted_digits_after_pipe
        .iter()
        .flatten()
        .filter(|&&seg| seg == 1 || seg == 4 || seg == 7 || seg == 8)
        .count()
}

fn part_two(data: &str) -> usize {
    let decrypted_digits_after_pipe = parse_data(data);
    decrypted_digits_after_pipe
        .iter()
        .map(|line| line[0] * 1000 + line[1] * 100 + line[2] * 10 + line[3])
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day8_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 26);
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
