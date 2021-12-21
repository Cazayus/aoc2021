use std::collections::{HashMap, HashSet};
use std::ops::Add;

use regex::Regex;

const DATA: &str = include_str!("../inputs/day19.txt");

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Add<Point3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Point3D {
    fn dist_to_point(&self, point: &Point3D) -> Point3D {
        Point3D {
            x: point.x - self.x,
            y: point.y - self.y,
            z: point.z - self.z,
        }
    }

    fn manhattan_distance(&self, point: &Point3D) -> i32 {
        (point.x - self.x).abs() + (point.y - self.y).abs() + (point.z - self.z).abs()
    }

    fn rotate_x(&self) -> Point3D {
        Point3D {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    fn rotate_y(&self) -> Point3D {
        Point3D {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rotate_z(&self) -> Point3D {
        Point3D {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }
}

struct Beacon {
    position_in_common_space: Point3D,
    orientations: [Point3D; 24],
}

struct Scanner {
    id: i32,
    position_in_common_space: Point3D,
    beacons: Vec<Beacon>,
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let mut scanners = parse_data(data);
    let mut solved_scanners = vec![scanners.swap_remove(0)];
    let mut dist_map = HashMap::new();
    while !scanners.is_empty() {
        let (index, solved_scanner) = solve_one_scanner(&scanners, &solved_scanners, &mut dist_map);
        solved_scanners.push(solved_scanner);
        scanners.swap_remove(index);
    }
    let beacon_set = solved_scanners
        .into_iter()
        .flat_map(|scanner| scanner.beacons)
        .map(|beacon| beacon.position_in_common_space)
        .collect::<HashSet<_>>();
    beacon_set.len()
}

fn part_two(data: &str) -> i32 {
    let mut scanners = parse_data(data);
    let mut solved_scanners = vec![scanners.swap_remove(0)];
    let mut dist_map = HashMap::new();
    while !scanners.is_empty() {
        let (index, solved_scanner) = solve_one_scanner(&scanners, &solved_scanners, &mut dist_map);
        solved_scanners.push(solved_scanner);
        scanners.swap_remove(index);
    }
    solved_scanners
        .iter()
        .flat_map(|s1| {
            solved_scanners.iter().map(|s2| {
                s2.position_in_common_space
                    .manhattan_distance(&s1.position_in_common_space)
            })
        })
        .max()
        .unwrap()
}

fn parse_data(data: &str) -> Vec<Scanner> {
    data.split("\r\n\r\n")
        .map(|scanner| Scanner {
            id: {
                let pat = Regex::new(r"--- scanner (\d+) ---").unwrap();
                let caps = pat.captures(scanner.lines().next().unwrap()).unwrap();
                caps[1].parse::<i32>().unwrap()
            },
            position_in_common_space: Default::default(),
            beacons: scanner.lines().skip(1).map(get_beacon).collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>()
}

fn get_beacon(line: &str) -> Beacon {
    let mut split = line.split(',');
    let p = Point3D {
        x: split.next().unwrap().parse().unwrap(),
        y: split.next().unwrap().parse().unwrap(),
        z: split.next().unwrap().parse().unwrap(),
    };
    Beacon {
        position_in_common_space: p,
        orientations: [
            // 4 rotations with x on x
            p,
            p.rotate_x(),
            p.rotate_x().rotate_x(),
            p.rotate_x().rotate_x().rotate_x(),
            // 4 rotations with x on y
            p.rotate_z(),
            p.rotate_z().rotate_y(),
            p.rotate_z().rotate_y().rotate_y(),
            p.rotate_z().rotate_y().rotate_y().rotate_y(),
            // 4 rotations with x on z
            p.rotate_y(),
            p.rotate_y().rotate_z(),
            p.rotate_y().rotate_z().rotate_z(),
            p.rotate_y().rotate_z().rotate_z().rotate_z(),
            // 4 rotations with x on -x
            p.rotate_y().rotate_y(),
            p.rotate_y().rotate_y().rotate_x(),
            p.rotate_y().rotate_y().rotate_x().rotate_x(),
            p.rotate_y().rotate_y().rotate_x().rotate_x().rotate_x(),
            // 4 rotations with x on -y
            p.rotate_z().rotate_z().rotate_z(),
            p.rotate_z().rotate_z().rotate_z().rotate_y(),
            p.rotate_z().rotate_z().rotate_z().rotate_y().rotate_y(),
            p.rotate_z()
                .rotate_z()
                .rotate_z()
                .rotate_y()
                .rotate_y()
                .rotate_y(),
            // 4 rotations with x on -z
            p.rotate_y().rotate_y().rotate_y(),
            p.rotate_y().rotate_y().rotate_y().rotate_z(),
            p.rotate_y().rotate_y().rotate_y().rotate_z().rotate_z(),
            p.rotate_y()
                .rotate_y()
                .rotate_y()
                .rotate_z()
                .rotate_z()
                .rotate_z(),
        ],
    }
}

fn solve_one_scanner(
    scanners: &[Scanner],
    solved_scanners: &[Scanner],
    dist_map: &mut HashMap<Point3D, i32>,
) -> (usize, Scanner) {
    for (index, scanner) in scanners.iter().enumerate() {
        for solved_scanner in solved_scanners {
            // pour chacune des 24 orientation
            for n in 0..24 {
                dist_map.clear();
                // pour chaque beacon du scanner résolu
                for beacon in &solved_scanner.beacons {
                    let base_orientation = &beacon.position_in_common_space;
                    // pour chaque beacon de l'orientation n du scanner non résolu
                    for beacon_after in &scanner.beacons {
                        let beacon_after = &beacon_after.orientations[n];
                        // calculer la distance
                        let distance = beacon_after.dist_to_point(base_orientation);
                        *dist_map.entry(distance).or_insert(0) += 1;
                    }
                }
                // si on trouve 12 ou plus distances identiques, on sait résoudre le scanner_after
                if dist_map.values().any(|&count| count >= 12) {
                    let distance = dist_map.iter().max_by_key(|(_, &count)| count).unwrap().0;
                    return (
                        index,
                        Scanner {
                            id: scanner.id,
                            position_in_common_space: *distance,
                            beacons: scanner
                                .beacons
                                .iter()
                                .map(|beacon| Beacon {
                                    position_in_common_space: {
                                        beacon.orientations[n] + *distance
                                    },
                                    orientations: Default::default(),
                                })
                                .collect(),
                        },
                    );
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day19_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 79);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 3621);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 353);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 10832);
    }
}
