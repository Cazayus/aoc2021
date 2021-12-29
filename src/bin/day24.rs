use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day24.txt");

fn main() {
    let (one, two) = solve(DATA);
    println!("part 1: {}", one);
    println!("part 2: {}", two);
}

fn solve(data: &str) -> (usize, usize) {
    let input = data
        .split("inp w\r\n")
        .filter(|bloc| bloc.len() > 0)
        .inspect(|bloc| println!("{} & {}", bloc.trim(), bloc.len()))
        .map(|bloc| bloc.trim().lines().collect())
        .map(|instructions: Vec<&str>| {
            let mut toto = HashMap::new();
            for x in 1..9 {
                toto.clear();
                toto.insert("w", x);
                toto.insert("x", 0);
                toto.insert("y", 0);
                toto.insert("z", 0);
                for instruction in instructions.clone() {
                    let split = instruction.split_whitespace().collect::<Vec<_>>();
                    let a = toto.entry(split[1]);
                    match split[0] {
                        "add" => {}
                        "mul" => {}
                        "div" => {}
                        "mod" => {}
                        "eql" => {}
                        _ => {}
                    }
                }
            }
        })
        .collect::<Vec<_>>();
    dbg!(input.len());
    let one = data.len();
    let two = data.len();
    (one, two)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day24_sample.txt");

    #[test]
    fn sample() {
        assert_eq!(solve(SAMPLE_DATA), (0, 0));
    }

    #[test]
    fn case() {
        assert_eq!(solve(DATA), (0, 0));
    }
}
