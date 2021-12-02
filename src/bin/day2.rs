use aoc2021::util;

fn main() {
    let instructions = util::read_lines_as_str("inputs/day2.txt");

    part_one(&instructions);
    part_two(instructions);
}

fn part_one(instructions: &Vec<String>) {
    let mut current_depth = 0;
    let mut current_horizontal = 0;
    instructions.iter().for_each(|instruction| {
        if instruction.starts_with("forward") {
            current_horizontal += instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        } else if instruction.starts_with("up") {
            current_depth -= instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        } else if instruction.starts_with("down") {
            current_depth += instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        } else {
            panic!()
        }
    });
    println!("Solution is {}", current_depth * current_horizontal)
}

fn part_two(instructions: Vec<String>) {
    let mut current_depth = 0;
    let mut current_horizontal = 0;
    let mut aim = 0;
    instructions.iter().for_each(|instruction| {
        if instruction.starts_with("forward") {
            let added_value = instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            current_horizontal += added_value;
            current_depth += aim * added_value;
        } else if instruction.starts_with("up") {
            aim -= instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        } else if instruction.starts_with("down") {
            aim += instruction.split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        } else {
            panic!()
        }
    });
    println!("Solution is {}", current_depth * current_horizontal)
}
