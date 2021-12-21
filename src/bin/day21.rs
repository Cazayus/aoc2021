use std::collections::HashMap;

const DATA: &str = include_str!("../inputs/day21.txt");

fn main() {
    println!("part 1: {}", part_one(DATA));
    println!("part 2: {}", part_two(DATA));
}

fn part_one(data: &str) -> usize {
    let mut lines = data.lines();
    let mut positions = [
        lines.next().unwrap()[28..].parse().unwrap(),
        lines.next().unwrap()[28..].parse().unwrap(),
    ];
    let mut scores = [0, 0];

    for turn in 0.. {
        let player_index = turn % 2;
        let total_roll = (1..=3)
            .map(|i| wrap_around(3 * turn + i, 100))
            .sum::<usize>();
        positions[player_index] = wrap_around(positions[player_index] + total_roll, 10);
        scores[player_index] += positions[player_index];
        if scores[player_index] >= 1000 {
            return 3 * (turn + 1) * scores[(player_index + 1) % 2];
        }
    }
    unreachable!()
}

fn part_two(data: &str) -> usize {
    let mut lines = data.lines();
    let positions = [
        lines.next().unwrap()[28..].parse::<usize>().unwrap(),
        lines.next().unwrap()[28..].parse::<usize>().unwrap(),
    ];
    let scores = [0, 0];
    // Comme pour le jour 6 et le jour 14, on peut se contenter de tracker le nombre de game dans chaque état au lieu de tout générer et exploser
    // A chaque étape, 27 univers sont créés.
    // Pour chacun de ces 27 univers, on peut calculer une fois pour toute de combien ils font avancer la position
    // Pour chacune des 10 positions possibles, on peut calculer une fois pour toute les 27 nouvelles positions + contribution au score
    // J'obtiens donc une map avec en clef la position de 1 à 10, et en valeur un tableau de 27 paires (new_pos, add_to_score) qui permettront de
    // calculer l'état suivant plus facilement
    let mut position_to_new_universe = HashMap::new();
    let mut games = HashMap::new();
    games.insert(Game { positions, scores }, 1);
    for position in 1..=10 {
        for dice1 in 1..=3 {
            for dice2 in 1..=3 {
                for dice3 in 1..=3 {
                    let vec = position_to_new_universe
                        .entry(position)
                        .or_insert_with(Vec::new);
                    let new_pos = wrap_around(position + dice1 + dice2 + dice3, 10);
                    let add_to_score = new_pos;
                    vec.push(Universe {
                        new_pos,
                        add_to_score,
                    });
                }
            }
        }
    }
    let mut games_won_by_player = [0, 0];
    for turn in 0.. {
        let player_index = turn % 2;
        let local_games = games.clone();
        games.clear();
        for (game, game_count) in local_games {
            let pos = game.positions[player_index];
            let score = game.scores[player_index];
            for universe in &position_to_new_universe[&pos] {
                let pos_universe = universe.new_pos;
                let score_universe = score + universe.add_to_score;
                if score_universe >= 21 {
                    games_won_by_player[player_index] += game_count;
                } else {
                    let mut next_game = Game {
                        positions: [0, 0],
                        scores: [0, 0],
                    };
                    let other_player_index = (player_index + 1) % 2;
                    next_game.positions[player_index] = pos_universe;
                    next_game.positions[other_player_index] = game.positions[other_player_index];
                    next_game.scores[player_index] = score_universe;
                    next_game.scores[other_player_index] = game.scores[other_player_index];
                    *games.entry(next_game).or_insert(0) += game_count;
                }
            }
        }
        if games.is_empty() {
            return games_won_by_player[0].max(games_won_by_player[1]);
        }
    }
    unreachable!()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Game {
    positions: [usize; 2],
    scores: [usize; 2],
}

struct Universe {
    new_pos: usize,
    add_to_score: usize,
}

fn wrap_around(value: usize, end_range: usize) -> usize {
    ((value - 1) % end_range) + 1
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
        assert_eq!(part_two(SAMPLE_DATA), 444356092776315);
    }

    #[test]
    fn test_one() {
        assert_eq!(part_one(DATA), 518418);
    }

    #[test]
    fn test_two() {
        assert_eq!(part_two(DATA), 116741133558209);
    }
}
