use aoc2021::util;
use std::collections::VecDeque;

fn main() {
    let input = util::read_lines("inputs/day6.txt");
    let mut colony: VecDeque<_> = [0u64; 9].iter().copied().collect();
    for fish in input.split(',') {
        colony[fish.parse().unwrap()] += 1;
    }
    part_one(colony.clone());
    part_two(colony.clone());
}

fn part_one(mut colony: VecDeque<u64>) {
    advance_day(80, &mut colony);
    println!("Solution is {}", colony.iter().sum::<u64>());
}

fn part_two(mut colony: VecDeque<u64>) {
    advance_day(256, &mut colony);
    println!("Solution is {}", colony.iter().sum::<u64>());
}

fn advance_day(nb_day: i32, colony: &mut VecDeque<u64>) {
    for _ in 0..nb_day {
        let breeder = colony.pop_front().unwrap();
        colony[6] += breeder;
        colony.push_back(breeder);
    }
}
