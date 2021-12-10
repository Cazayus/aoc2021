const DATA: &str = include_str!("../inputs/day3.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn parse_data(data: &str) -> Vec<Vec<bool>> {
    data.lines()
        .map(|line| line.chars().map(|char| char == '1').collect())
        .collect()
}

fn find_most_common_bit_as_vec(data: &[Vec<bool>]) -> Vec<bool> {
    let line_length = data[0].len();
    (0..line_length)
        .map(|index| {
            let (ones, zeroes): (Vec<bool>, Vec<bool>) =
                data.iter().map(|line| line[index]).partition(|&bit| bit);
            ones.len() >= zeroes.len()
        })
        .collect()
}

fn convert_most_common_to_least_common(most_common: &[bool]) -> Vec<bool> {
    return most_common.iter().map(|&bit| !bit).collect();
}

fn convert_vec_bit_to_i32(vec_bit: &[bool]) -> i32 {
    return i32::from_str_radix(
        vec_bit
            .iter()
            .map(|&bit| if bit { '1' } else { '0' })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap();
}

fn part_one(data: &str) -> i32 {
    let data = parse_data(data);
    let most_common_bits = find_most_common_bit_as_vec(&data);
    let least_common_bits = convert_most_common_to_least_common(&most_common_bits);
    let gamma = convert_vec_bit_to_i32(&most_common_bits);
    let epsilon = convert_vec_bit_to_i32(&least_common_bits);
    gamma * epsilon
}

fn part_two(data: &str) -> i32 {
    let data = parse_data(data);
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<Vec<bool>> = data.clone();
    while mutable_lines.len() > 1 {
        let most_common_bits = find_most_common_bit_as_vec(&mutable_lines);
        mutable_lines.retain(|line| {
            line[current_bit_under_scrutiny] == most_common_bits[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let oxygen = convert_vec_bit_to_i32(mutable_lines.first().unwrap());
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<Vec<bool>> = data;
    while mutable_lines.len() > 1 {
        let least_common_bit =
            convert_most_common_to_least_common(&find_most_common_bit_as_vec(&mutable_lines));
        mutable_lines.retain(|line| {
            line[current_bit_under_scrutiny] == least_common_bit[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let co2 = convert_vec_bit_to_i32(mutable_lines.first().unwrap());
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
