use aoc2021::util;
use std::collections::VecDeque;

fn main() {
    let input = util::read_lines("inputs/day6.txt");
    let mut colony: VecDeque<usize> = VecDeque::new();
    for _ in 0..=8 {
        colony.push_front(0);
    }
    input
        .split(',')
        .map(|timer| timer.parse::<usize>().unwrap())
        .for_each(|timer| *colony.get_mut(timer).unwrap() += 1);
    part_one(colony.clone());
    part_two(colony.clone());
}

fn part_one(mut colony: VecDeque<usize>) {
    advance_day(80, &mut colony);
    println!("Solution is {}", colony.iter().sum::<usize>());
}

fn part_two(mut colony: VecDeque<usize>) {
    advance_day(256, &mut colony);
    println!("Solution is {}", colony.iter().sum::<usize>());
}

fn advance_day(nb_day: i32, colony: &mut VecDeque<usize>) {
    for _ in 0..nb_day {
        let breeder = colony.pop_front().unwrap();
        *colony.get_mut(6).unwrap() += breeder;
        colony.push_back(breeder);
    }
}
