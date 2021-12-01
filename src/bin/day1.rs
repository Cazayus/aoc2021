use aoc2021::util;

fn read_ints(path: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let lines = util::read_lines(path).unwrap();

    for line in lines {
        let num = line.unwrap().parse::<i32>().unwrap();
        out.push(num);
    }

    out
}

fn main() {
    let nums = read_ints("inputs/day1.txt");
    println!("Read {} nums", nums.len());

    part_one(&nums);
    part_two(nums);
}

fn part_one(nums: &Vec<i32>) {
    println!(
        "Nb of increases is {}",
        nums.windows(2)
            .filter(|window| window[0] < window[1])
            .count()
    );
}

fn part_two(nums: Vec<i32>) {
    println!(
        "Nb of increases is {}",
        nums.windows(3)
            .map(|window| window.iter().sum::<i32>())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|window| window[0] < window[1])
            .count()
    );

    println!(
        "Nb of increases is {}",
        nums.windows(4).filter(|w| w[0] < w[3]).count()
    );
}
