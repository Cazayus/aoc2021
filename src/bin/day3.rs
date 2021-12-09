const DATA: &str = include_str!("../inputs/day3.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn find_most_common_bit_as_vec(data: &Vec<&str>) -> Vec<char> {
    let line_length = data[0].len();
    let mut output: Vec<char> = Vec::new();
    for index in 0..line_length {
        let sum: i32 = data
            .iter()
            .map(|b| b[index..index + 1].parse::<i32>().unwrap())
            .sum();
        if sum as f64 >= data.len() as f64 / 2.0 {
            output.push('1');
        } else {
            output.push('0');
        }
    }
    output
}

fn convert_most_common_to_least_common(most_common: &Vec<char>) -> Vec<char> {
    return most_common
        .iter()
        .map(|&c| if c == '1' { '0' } else { '1' })
        .collect();
}

fn part_one(data: &str) -> i32 {
    let most_common_bits = find_most_common_bit_as_vec(&data.lines().collect());
    let least_common_bits = convert_most_common_to_least_common(&most_common_bits);
    let gamma =
        i32::from_str_radix(most_common_bits.iter().collect::<String>().as_str(), 2).unwrap();
    let epsilon =
        i32::from_str_radix(least_common_bits.iter().collect::<String>().as_str(), 2).unwrap();
    gamma * epsilon
}

fn part_two(data: &str) -> i32 {
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = data.lines().collect::<Vec<&str>>().to_vec();
    while mutable_lines.len() > 1 {
        let most_common_bits = find_most_common_bit_as_vec(&mutable_lines);
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == most_common_bits[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let oxygen = i32::from_str_radix(mutable_lines.first().unwrap(), 2).unwrap();
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = data.lines().collect::<Vec<&str>>().to_vec();
    while mutable_lines.len() > 1 {
        let least_common_bit =
            convert_most_common_to_least_common(&find_most_common_bit_as_vec(&mutable_lines));
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == least_common_bit[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let co2 = i32::from_str_radix(mutable_lines.first().unwrap(), 2).unwrap();
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day3_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 198);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 230);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 3242606);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 4856080);
    }
}
