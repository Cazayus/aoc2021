use aoc2021::util;

fn main() {
    let input = util::read_lines("inputs/day2.txt");
    let instructions: Vec<(&str, i64)> = input
        .lines()
        .map(|line| {
            let split_instruction: Vec<&str> = line.split_whitespace().collect();
            (split_instruction[0], split_instruction[1].parse().unwrap())
        })
        .collect();
    part_one(&instructions);
    part_two(&instructions);
}

fn part_one(instructions: &[(&str, i64)]) {
    let mut depth_horizontal = (0, 0);

    for instruction in instructions {
        match instruction.0 {
            "forward" => depth_horizontal.1 += instruction.1,
            "up" => depth_horizontal.0 -= instruction.1,
            "down" => depth_horizontal.0 += instruction.1,
            _ => panic!(),
        }
    }
    println!("Solution is {}", depth_horizontal.0 * depth_horizontal.1);
}

fn part_two(instructions: &[(&str, i64)]) {
    let mut depth_horizontal_aim = (0, 0, 0);

    for instruction in instructions {
        match instruction.0 {
            "forward" => {
                depth_horizontal_aim.1 += instruction.1;
                depth_horizontal_aim.0 += depth_horizontal_aim.2 * instruction.1;
            }
            "up" => depth_horizontal_aim.2 -= instruction.1,
            "down" => depth_horizontal_aim.2 += instruction.1,
            _ => panic!(),
        }
    }
    println!(
        "Solution is {}",
        depth_horizontal_aim.0 * depth_horizontal_aim.1
    );
}
