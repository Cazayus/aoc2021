const DATA: &str = include_str!("../inputs/day17.txt");
const MAX_VY_TESTED: u32 = 500;

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> i32 {
    let data = parse_data(data);
    let (_, _) = find_minmax_vx(data[0], data[1]);
    let (_, max_vy) = find_minmax_vy(data[2], data[3]);
    max_vy * (max_vy + 1) / 2
}

fn part_two(data: &str) -> usize {
    let data = parse_data(data);
    let (min_vx, max_vx) = find_minmax_vx(data[0], data[1]);
    let (min_vy, max_vy) = find_minmax_vy(data[2], data[3]);
    let mut count = 0;
    for dx in min_vx..=max_vx {
        for dy in min_vy..=max_vy {
            let mut pos = (0, 0);
            let mut local_diff = (dx, dy);
            while pos.1 > data[2] {
                pos.0 += local_diff.0;
                pos.1 += local_diff.1;
                if (data[0]..=data[1]).contains(&pos.0) && (data[2]..=data[3]).contains(&pos.1) {
                    count += 1;
                    break;
                }
                if local_diff.0 > 0 {
                    local_diff.0 -= 1;
                }
                local_diff.1 -= 1;
            }
        }
    }
    count
}

fn parse_data(data: &str) -> Vec<i32> {
    data[15..]
        .to_string()
        .split(", y=")
        .flat_map(|split| {
            split
                .split("..")
                .map(|split| split.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_minmax_vx(min_x_target: i32, max_x_target: i32) -> (i32, i32) {
    let mut min_vx = 0;
    for dx in 1..max_x_target {
        let mut pos_x = 0;
        let mut local_dx = dx;
        while local_dx > 0 && pos_x < max_x_target {
            pos_x += local_dx;
            local_dx -= 1;
            if (min_x_target..=max_x_target).contains(&pos_x) {
                if min_vx == 0 {
                    min_vx = dx;
                    break;
                }
            }
        }
    }
    (min_vx, max_x_target)
}

fn find_minmax_vy(min_y_target: i32, max_y_target: i32) -> (i32, i32) {
    let mut max_vy = 0;
    let mut latest_working_dy = 0;
    for dy in 1..MAX_VY_TESTED {
        let mut pos_y = 0;
        let mut local_dy = dy as i32;
        while pos_y > min_y_target {
            pos_y += local_dy;
            local_dy -= 1;
            if (min_y_target..=max_y_target).contains(&pos_y) {
                latest_working_dy = dy as i32;
            }
        }
        max_vy = latest_working_dy;
    }
    (min_y_target, max_vy)
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
