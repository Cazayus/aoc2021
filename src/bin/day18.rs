use regex::Regex;
use std::str::Chars;

const DATA: &str = include_str!("../inputs/day18.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u32 {
    let mut lines = data.lines();
    let mut result = lines.next().unwrap().to_string();
    for line in lines {
        result = add(&result, line);
    }
    magnitude(&result)
}

fn part_two(data: &str) -> u32 {
    data.lines()
        .flat_map(|line1| {
            data.lines()
                .map(|line2| magnitude(&add(&line1.to_string(), line2)))
        })
        .max()
        .unwrap()
}

fn add(number1: &str, number2: &str) -> String {
    reduce(&format!("[{},{}]", number1, number2))
}

fn reduce(number: &str) -> String {
    let mut result = number.to_string();
    let mut has_exploded = true;
    let mut has_split = true;
    while has_exploded || has_split {
        while has_exploded {
            has_exploded = explode(&mut result);
        }
        has_split = split(&mut result);
        has_exploded = explode(&mut result);
    }
    result
}

fn explode(number: &mut String) -> bool {
    let local_number = number.clone();
    let mut chars = local_number.chars();
    number.clear();
    let mut open_count = 0;
    while let Some(char) = chars.next() {
        match char {
            '[' => open_count += 1,
            ']' => open_count -= 1,
            _ => {}
        }
        if open_count == 5 {
            let (left, right) = find_pair_to_explode(&mut chars);
            add_left_to_number(number, left);
            number.push('0');
            add_right_to_number(number, &mut chars, right);
            finish_to_consume_chars(number, &mut chars);
            return true;
        }
        number.push(char);
    }
    false
}

fn find_pair_to_explode(chars: &mut Chars) -> (u32, u32) {
    let mut pair = String::new();
    for char in chars {
        if char == ']' {
            break;
        } else {
            pair.push(char)
        }
    }
    let mut split = pair.split(',');
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

fn add_left_to_number(number: &mut String, left: u32) {
    let mut temp_string = String::new();
    let mut digit = String::new();
    while let Some(mut char) = number.pop() {
        if char.is_digit(10) {
            while char.is_digit(10) {
                digit.push(char);
                char = number.pop().unwrap();
            }
            number.push(char);
            let mut digit = digit
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            digit += left;
            digit.to_string().chars().for_each(|c| number.push(c));
            break;
        } else {
            temp_string.push(char);
        }
    }
    while let Some(char) = temp_string.pop() {
        number.push(char);
    }
}

fn add_right_to_number(number: &mut String, chars: &mut Chars, right: u32) {
    let mut digit = String::new();
    while let Some(mut char) = chars.next() {
        if char.is_digit(10) {
            while char.is_digit(10) {
                digit.push(char);
                char = chars.next().unwrap();
            }
            let mut digit = digit.parse::<u32>().unwrap();
            digit += right;
            digit.to_string().chars().for_each(|c| number.push(c));
            number.push(char);
            break;
        } else {
            number.push(char);
        }
    }
}

fn finish_to_consume_chars(number: &mut String, chars: &mut Chars) {
    for char in chars {
        number.push(char);
    }
}

fn split(number: &mut String) -> bool {
    let local_number = number.clone();
    let mut chars = local_number.chars();
    number.clear();
    let mut digit = String::new();
    while let Some(char) = chars.next() {
        if char.is_digit(10) {
            digit.push(char);
            if digit.len() > 1 {
                let digit = digit.parse::<u32>().unwrap();
                let left = digit / 2;
                let right = (digit + 1) / 2;
                number.push('[');
                left.to_string().chars().for_each(|c| number.push(c));
                number.push(',');
                right.to_string().chars().for_each(|c| number.push(c));
                number.push(']');
                finish_to_consume_chars(number, &mut chars);
                return true;
            }
        } else {
            if !digit.is_empty() {
                number.push(digit.pop().unwrap());
            }
            number.push(char);
        }
    }
    false
}

fn magnitude(number: &str) -> u32 {
    let mut local_number: String = number.to_string();
    let pat = Regex::new(r"\[(\d+),(\d+)]").unwrap();
    while let Some(caps) = pat.captures(&local_number) {
        let concat: i32 = (3 * &caps[1].parse().unwrap()) + (2 * &caps[2].parse().unwrap());
        local_number = local_number.replacen(&caps[0], concat.to_string().as_str(), 1);
    }
    local_number.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day18_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 4140);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 3993);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 4435);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 4802);
    }
}
