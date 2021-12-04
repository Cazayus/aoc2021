use aoc2021::util;

fn main() {
    let input = util::read_lines("inputs/day3.txt");
    let lines: Vec<&str> = input.lines().collect();
    part_one(&lines);
    part_two(&lines);
}

fn find_most_and_least_common_bit_as_vec(lines: &Vec<&str>) -> (Vec<char>, Vec<char>) {
    let mut ones_vec = Vec::new();
    for line in lines {
        for (index, b) in line.bytes().enumerate() {
            if ones_vec.get(index).is_none() {
                ones_vec.push(0);
            }
            if b == b'1' {
                ones_vec[index] += 1;
            }
        }
    }
    return (
        ones_vec
            .iter()
            .map(|&ones| {
                return if ones == lines.len()
                    || ones as f64 >= (lines.len() as f64 / 2.0) && ones != 0
                {
                    '1'
                } else {
                    '0'
                };
            })
            .collect::<Vec<char>>(),
        ones_vec
            .iter()
            .map(|&ones| {
                return if ones == lines.len()
                    || (ones as f64) < (lines.len() as f64 / 2.0) && ones != 0
                {
                    '1'
                } else {
                    '0'
                };
            })
            .collect::<Vec<char>>(),
    );
}

fn convert_binary_to_decimal(binary: &Vec<char>) -> i32 {
    let mut output = 0;
    for (index, &bit) in binary.iter().rev().enumerate() {
        if bit == '1' {
            output += 2_i32.pow(index as u32);
        }
    }
    return output;
}

fn part_one(lines: &Vec<&str>) {
    let (most_common_bits, least_common_bits) = find_most_and_least_common_bit_as_vec(&lines);
    let gamma = convert_binary_to_decimal(&most_common_bits);
    let epsilon = convert_binary_to_decimal(&least_common_bits);
    println!("Solution is {}", gamma * epsilon);
}

fn part_two(lines: &Vec<&str>) {
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = lines.clone();
    while mutable_lines.len() > 1 {
        let (most_common_bits, _) = find_most_and_least_common_bit_as_vec(&mutable_lines);
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == most_common_bits[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let oxygen = convert_binary_to_decimal(&mutable_lines.first().unwrap().chars().collect());
    let mut current_bit_under_scrutiny = 0;
    let mut mutable_lines: Vec<&str> = lines.clone();
    while mutable_lines.len() > 1 {
        let (_, least_common_bit) = find_most_and_least_common_bit_as_vec(&mutable_lines);
        mutable_lines.retain(|line| {
            line.chars().nth(current_bit_under_scrutiny).unwrap()
                == least_common_bit[current_bit_under_scrutiny]
        });
        current_bit_under_scrutiny += 1;
    }
    let co2 = convert_binary_to_decimal(&mutable_lines.first().unwrap().chars().collect());
    println!("Solution is {}", oxygen * co2);
}
