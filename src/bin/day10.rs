const DATA: &str = include_str!("../inputs/day10.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i64 {
    data.lines()
        .map(|line| {
            let mut stack: Vec<char> = vec![];
            compute_error_score(line, &mut stack)
        })
        .sum()
}

fn compute_error_score(line: &str, stack: &mut Vec<char>) -> i64 {
    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return 3;
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return 57;
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return 1197;
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return 25137;
                }
            }
            _ => panic!(),
        }
    }
    return 0;
}

fn part_two(data: &str) -> i64 {
    let mut scores: Vec<i64> = data
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<char> = vec![];
            let mut output = compute_error_score(line, &mut stack);
            if output == 0 {
                stack.iter().rev().for_each(|&char| {
                    output *= 5;
                    match char {
                        '(' => output += 1,
                        '[' => output += 2,
                        '{' => output += 3,
                        '<' => output += 4,
                        _ => panic!(),
                    }
                });
                Some(output)
            } else {
                None
            }
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day10_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 26397);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 288957);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 392097);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 4263222782);
    }
}
