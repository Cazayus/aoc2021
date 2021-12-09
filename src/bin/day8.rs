use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day8.txt");

// check this https://www.reddit.com/r/adventofcode/comments/rc5s3z/2021_day_8_part_2_a_simple_fast_and_deterministic/

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

struct Line {
    before: Vec<String>,
    after: Vec<String>,
    seg_list_to_value: HashMap<String, i32>,
    after_decoded: Vec<i32>,
}

impl Line {
    fn decipher(&mut self) {
        for id in self.before.iter() {
            match id.len() {
                2 => self.seg_list_to_value.insert(id.to_string(), 1),
                3 => self.seg_list_to_value.insert(id.to_string(), 7),
                4 => self.seg_list_to_value.insert(id.to_string(), 4),
                7 => self.seg_list_to_value.insert(id.to_string(), 8),
                _ => None,
            };
        }
        for id in self.before.iter() {
            if id.len() == 6 && self.seg_list_contains(id, 4) {
                self.seg_list_to_value.insert(id.to_string(), 9);
            }
            if id.len() == 5 && self.seg_list_contains(id, 1) {
                self.seg_list_to_value.insert(id.to_string(), 3);
            }
        }
        for id in self.before.iter() {
            if id.len() == 6 && !self.seg_list_contains(id, 7) {
                self.seg_list_to_value.insert(id.to_string(), 6);
            }
        }
        for id in self.before.iter() {
            if id.len() == 5 && self.seg_list_is_contained_by(id, 6) {
                self.seg_list_to_value.insert(id.to_string(), 5);
            }
        }
        for id in self.before.iter() {
            if id.len() == 6 && !self.seg_list_to_value.contains_key(id) {
                self.seg_list_to_value.insert(id.to_string(), 0);
            }
            if id.len() == 5 && !self.seg_list_to_value.contains_key(id) {
                self.seg_list_to_value.insert(id.to_string(), 2);
            }
        }
        self.after_decoded = self
            .after
            .iter()
            .map(|seg| *self.seg_list_to_value.get(seg).unwrap())
            .collect()
    }

    fn seg_list_contains(&self, seg_list: &str, to_contains: i32) -> bool {
        self.seg_list_to_value
            .iter()
            .find_map(|(key, &val)| if val == to_contains { Some(key) } else { None })
            .unwrap()
            .chars()
            .all(|char| seg_list.contains(char))
    }

    fn seg_list_is_contained_by(&self, seg_list: &str, container: i32) -> bool {
        let container = self
            .seg_list_to_value
            .iter()
            .find_map(|(key, &val)| if val == container { Some(key) } else { None })
            .unwrap();
        seg_list.chars().all(|char| container.contains(char))
    }
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn extract_sorted_seg_list(split_line: &str) -> Vec<String> {
    split_line.split_whitespace().map(sort_string).collect()
}

fn sort_string(elem: &str) -> String {
    let mut temp_vec: Vec<char> = elem.chars().collect();
    temp_vec.sort_unstable();
    temp_vec.into_iter().collect()
}

fn part_one(data: &str) -> usize {
    let input: Vec<Line> = data
        .lines()
        .map(|line| line.split(" | "))
        .map(|mut split_line| {
            let mut line = Line {
                seg_list_to_value: Default::default(),
                before: extract_sorted_seg_list(split_line.next().unwrap()),
                after: extract_sorted_seg_list(split_line.next().unwrap()),
                after_decoded: vec![],
            };
            line.decipher();
            line
        })
        .collect();
    input
        .iter()
        .flat_map(|line| &line.after_decoded)
        .filter(|&&seg| seg == 1 || seg == 4 || seg == 7 || seg == 8)
        .count()
}

fn part_two(data: &str) -> i32 {
    let input: Vec<Line> = data
        .lines()
        .map(|line| line.split(" | "))
        .map(|mut split_line| {
            let mut line = Line {
                seg_list_to_value: Default::default(),
                before: extract_sorted_seg_list(split_line.next().unwrap()),
                after: extract_sorted_seg_list(split_line.next().unwrap()),
                after_decoded: vec![],
            };
            line.decipher();
            line
        })
        .collect();
    input
        .iter()
        .map(|line| {
            line.after_decoded[0] * 1000
                + line.after_decoded[1] * 100
                + line.after_decoded[2] * 10
                + line.after_decoded[3]
        })
        .sum::<i32>()
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
