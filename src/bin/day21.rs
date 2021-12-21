const DATA: &str = include_str!("../inputs/day21.txt");

struct Player {
    pos: u32,
    score: u32,
    consecutive_play: u8,
}

impl Player {
    fn advance_player(&mut self, value: u32) {
        self.pos = (self.pos + value - 1) % 10 + 1;
        self.consecutive_play += 1;
    }

    fn advance_score(&mut self) {
        self.score += self.pos;
    }
}

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> u32 {
    let mut lines = data.lines();
    let mut player1 = Player {
        pos: lines.next().unwrap()[28..].parse::<u32>().unwrap(),
        score: 0,
        consecutive_play: 0,
    };
    let mut player2 = Player {
        pos: lines.next().unwrap()[28..].parse::<u32>().unwrap(),
        score: 0,
        consecutive_play: 0,
    };
    let mut current_dice_value = 100;
    let mut dice_roll = 0;
    while player1.score < 1000 && player2.score < 1000 {
        dice_roll += 1;
        current_dice_value = (current_dice_value) % 100 + 1;
        if player1.consecutive_play < 3 {
            player1.advance_player(current_dice_value);
            if player1.consecutive_play == 3 {
                player1.advance_score();
                player2.consecutive_play = 0;
            }
        } else {
            player2.advance_player(current_dice_value);
            if player2.consecutive_play == 3 {
                player2.advance_score();
                player1.consecutive_play = 0;
            }
        }
    }
    if player1.score < 1000 {
        player1.score * dice_roll
    } else {
        player2.score * dice_roll
    }
}

fn part_two(data: &str) -> usize {
    data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_DATA: &str = include_str!("../inputs/day21_sample.txt");

    #[test]
    fn test_one_sample() {
        assert_eq!(part_one(SAMPLE_DATA), 739785);
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
