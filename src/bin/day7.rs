use aoc2021::util;

fn main() {
    let input = util::read_lines("inputs/day7.txt");
    let positions: Vec<i32> = input
        .split(',')
        .map(|position| position.parse().unwrap())
        .collect();
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    part_one(&positions, *min, *max);
    part_two(&positions, *min, *max);
}

fn part_one(positions: &Vec<i32>, min: i32, max: i32) {
    let answer: i32 = (min..=max)
        .map(|target| {
            positions
                .iter()
                .map(|element| (element - target).abs())
                .sum()
        })
        .min()
        .unwrap();
    println!("Solution is {}", answer);
}

fn part_two(positions: &Vec<i32>, min: i32, max: i32) {
    let answer: i32 = (min..=max)
        .map(|target| {
            positions
                .iter()
                .map(|element| {
                    let nb_elem = (element - target).abs();
                    return nb_elem * (nb_elem + 1) / 2;
                })
                .sum()
        })
        .min()
        .unwrap();
    println!("Solution is {}", answer);
}
