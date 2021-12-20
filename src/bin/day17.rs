use regex::Regex;

const DATA: &str = include_str!("../inputs/day17.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> isize {
    let [_, _, min_y_target, _] = parse_data(data);
    // the probe comes back at Y=0 with the same speed you shoot it up but in reverse
    // in order not to overshoot, the maximum allowed speed is then abs(min_y_target) - 1 and the
    // next step would see us barely touching the bottom side of the area
    let max_vy = -min_y_target - 1;
    max_vy * (max_vy + 1) / 2 // Simple somme arithmétique de raison 1
}

fn part_two(data: &str) -> isize {
    let [min_x_area, max_x_area, min_y_area, max_y_area] = parse_data(data);
    let mut count = 0;
    // No need to check beyond a x velocity of max_x_target (we would shoot beyond the area)
    for init_vx in 0..=max_x_area {
        // No need to check before a y velocity of min_y_target (we would shoot below the area)
        // max y velocity is the same as part 1
        for init_vy in min_y_area..=(-min_y_area - 1) {
            let (mut x, mut y) = (0, 0);
            let (mut vx, mut vy) = (init_vx, init_vy);
            while x <= max_x_area && y > min_y_area {
                x += vx;
                y += vy;
                vx -= vx.signum();
                vy -= 1;
                if vx == 0 && x < min_x_area {
                    break;
                }
                if (min_x_area..=max_x_area).contains(&x) && (min_y_area..=max_y_area).contains(&y)
                {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn parse_data(data: &str) -> [isize; 4] {
    let pat = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let caps = pat.captures(data).unwrap();
    [&caps[1], &caps[2], &caps[3], &caps[4]].map(|s| s.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day17_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 45);
    }

    #[test]
    fn test_two_sample() {
        assert_eq!(part_two(SAMPLE_DATA), 112);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 5995);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 3202);
    }
}
