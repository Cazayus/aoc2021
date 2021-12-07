use aoc2021::util;

fn main() {
    let input = util::read_lines("inputs/day3.txt");
    let lines: Vec<&str> = input.lines().collect();
    part_one(&lines);
    part_two(&lines);
}

fn find_most_common_bit_as_vec(lines: &[&str]) -> Vec<char> {
    let line_length = lines[0].len();
    let mut output: Vec<char> = Vec::new();
    for index in 0..line_length {
        let sum: i32 = lines
            .iter()
            .map(|b| b[index..index + 1].parse::<i32>().unwrap())
            .sum();
        if sum as f64 >= lines.len() as f64 / 2.0 {
            output.push('1');
        } else {
            output.push('0');
        }
    }
    output
}

fn convert_most_common_to_least_common(most_common: &[char]) -> Vec<char> {
    return most_common
        .iter()
        .map(|&c| if c == '1' { '0' } else { '1' })
        .collect();
}

fn part_one(lines: &[&str]) {
    let most_common_bits = find_most_common_bit_as_vec(lines);
    let least_common_bits = convert_most_common_to_least_common(&most_common_bits);
    let gamma =
        isize::from_str_radix(most_common_bits.iter().collect::<String>().as_str(), 2).unwrap();
    let epsilon =
        isize::from_str_radix(least_common_bits.iter().collect::<String>().as_str(), 2).unwrap();
    println!("Solution is {}", gamma * epsilon);
}

fn part_two(lines: &[&str]) {
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = lines.to_vec();
    while mutable_lines.len() > 1 {
        let most_common_bits = find_most_common_bit_as_vec(&mutable_lines);
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == most_common_bits[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let oxygen = isize::from_str_radix(mutable_lines.first().unwrap(), 2).unwrap();
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = lines.to_vec();
    while mutable_lines.len() > 1 {
        let least_common_bit =
            convert_most_common_to_least_common(&find_most_common_bit_as_vec(&mutable_lines));
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == least_common_bit[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let co2 = isize::from_str_radix(mutable_lines.first().unwrap(), 2).unwrap();
    println!("Solution is {}", oxygen * co2);
}
