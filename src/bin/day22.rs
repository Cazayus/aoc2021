use core::cmp::{max, min};
use regex::Regex;
use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day22.txt");

struct Instruction {
    command: String,
    volume: Cube,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Cube {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Cube {
    fn intersect(&self, cube: &Cube) -> Option<Cube> {
        let x_min = max(cube.x_min, self.x_min);
        let x_max = min(cube.x_max, self.x_max);
        let y_min = max(cube.y_min, self.y_min);
        let y_max = min(cube.y_max, self.y_max);
        let z_min = max(cube.z_min, self.z_min);
        let z_max = min(cube.z_max, self.z_max);
        if x_max >= x_min && y_max >= y_min && z_max >= z_min {
            return Some(Cube {
                x_min,
                x_max,
                y_min,
                y_max,
                z_min,
                z_max,
            });
        }
        None
    }

    fn area(&self) -> usize {
        (self.x_min..=self.x_max).count()
            * (self.y_min..=self.y_max).count()
            * (self.z_min..=self.z_max).count()
    }

    fn area_in_initialization(&self) -> usize {
        let x_min = max(-50, self.x_min);
        let x_max = min(50, self.x_max);
        let y_min = max(-50, self.y_min);
        let y_max = min(50, self.y_max);
        let z_min = max(-50, self.z_min);
        let z_max = min(50, self.z_max);
        (x_min..=x_max).count() * (y_min..=y_max).count() * (z_min..=z_max).count()
    }
}

fn main() {
    let (one, two) = solve(DATA);
    println!("part 1: {}", one);
    println!("part 2: {}", two);
}

fn solve(data: &str) -> (usize, usize) {
    let weighted_volumes = compute_weighted_volumes(data);
    let one = weighted_volumes
        .iter()
        .map(|(weight, volume)| weight * volume.area_in_initialization())
        .sum();
    let two = weighted_volumes
        .iter()
        .map(|(weight, volume)| weight * volume.area())
        .sum();
    (one, two)
}

fn parse_data(data: &str) -> Vec<Instruction> {
    let pat = Regex::new(r"(\w+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
        .unwrap();
    let instructions: Vec<_> = data
        .lines()
        .map(|line| {
            let caps = pat.captures(line).unwrap();
            Instruction {
                command: caps[1].to_string(),
                volume: Cube {
                    x_min: caps[2].parse::<i32>().unwrap(),
                    x_max: caps[3].parse::<i32>().unwrap(),
                    y_min: caps[4].parse::<i32>().unwrap(),
                    y_max: caps[5].parse::<i32>().unwrap(),
                    z_min: caps[6].parse::<i32>().unwrap(),
                    z_max: caps[7].parse::<i32>().unwrap(),
                },
            }
        })
        .collect();
    instructions
}

fn compute_weighted_volumes(data: &str) -> Vec<(usize, Cube)> {
    let instructions = parse_data(data);
    // Computing intersections of a region with all previous regions and adding that to the list with
    // inverted weight to eliminate double counting.
    // To avoid long lists of the same volume segment over and over with alternating weights of +1 and -1
    // I've added some bookkeeping to eliminate redundant entries in my list of volume weights.
    let mut weighted_volumes: Vec<(usize, Cube)> = Vec::new();
    for Instruction { command, volume } in instructions {
        let mut new_weighted_volumes = HashMap::new();
        weighted_volumes
            .iter_mut()
            .for_each(|(weight, added_volume)| {
                let intersect = volume.intersect(added_volume);
                if let Some(intersect) = intersect {
                    if intersect == *added_volume {
                        *weight = 0;
                    } else {
                        *new_weighted_volumes.entry(intersect).or_insert(0) -= *weight;
                    }
                }
            });
        for (new_volume, weight) in new_weighted_volumes.into_iter() {
            if weight != 0 {
                weighted_volumes.push((weight, new_volume));
            }
        }
        if command == "on" {
            weighted_volumes.push((1, volume))
        }
    }
    weighted_volumes
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day22_sample.txt");

    #[test]
    fn sample() {
        assert_eq!(solve(SAMPLE_DATA), (474140, 2758514936282235));
    }

    #[test]
    fn case() {
        assert_eq!(solve(DATA), (591365, 1211172281877240));
    }
}
